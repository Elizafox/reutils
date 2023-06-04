/* platform/unix/linux/fsent.rs - Linux filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::ffi::{CStr, CString};
use std::io;
use std::mem::MaybeUninit;

use libc::{endmntent, getmntent_r, mntent, setmntent, statvfs, PATH_MAX};

use crate::platform::fsent::{FilesystemEntry, FilesystemStats};

#[allow(clippy::missing_errors_doc)]
pub fn get_mounted_filesystems() -> io::Result<Vec<FilesystemEntry>> {
    let mut entries = Vec::<FilesystemEntry>::new();
    let mut result: Option<io::Error> = None;

    /* If you're building for a REALLY old Linux, from before kernel 2.4.19:
     * 0) Impressive that you got Rust to build on an antique like that
     * 1) Upgrade
     * 2) We will fall back to /etc/mtab if this fails.
     *
     * It's been 21 years as of this writing, /proc/self/mounts is old enough to drink. And if your
     * environment doesn't have procfs, your environment sucks and you need to rethink your life
     * choices.
     * --Elizafox
     */
    let mut mntfile = unsafe { setmntent(b"/proc/self/mounts\0".as_ptr(), "r\0".as_ptr()) };
    if mntfile.is_null() {
        // Sigh.
        mntfile = unsafe { setmntent(b"/etc/mtab\0".as_ptr(), "r\0".as_ptr()) };
        if mntfile.is_null() {
            return Err(io::Error::last_os_error());
        }

        eprintln!("WARNING: could not open /proc/self/mounts, falling back to /etc/mtab.");
        eprintln!("Information may not be accurate!");
    }

    loop {
        let mut mnt = MaybeUninit::<mntent>::uninit();
        let mut buf: [u8; (PATH_MAX * 4) as usize] = [0; (PATH_MAX * 4) as usize];

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        let buflen = buf.len() as i32;
        if unsafe { getmntent_r(mntfile, mnt.as_mut_ptr(), buf.as_mut_ptr(), buflen).is_null() } {
            let err = io::Error::last_os_error();
            if let Some(errno) = err.raw_os_error() {
                if errno > 0 {
                    result = Some(err);
                }
            }
            break;
        }

        let mnt = unsafe { mnt.assume_init() };

        entries.push(unsafe {
            FilesystemEntry {
                filesystem_name: CStr::from_ptr(mnt.mnt_type).to_string_lossy().into_owned(),
                mount_point: CStr::from_ptr(mnt.mnt_dir).to_string_lossy().into_owned(),
                mount_from: CStr::from_ptr(mnt.mnt_fsname)
                    .to_string_lossy()
                    .into_owned(),
                mount_options: CStr::from_ptr(mnt.mnt_opts).to_string_lossy().into_owned(),
            }
        });
    }

    unsafe {
        endmntent(mntfile);
    }

    result.map_or_else(|| Ok(entries), Err)
}

#[allow(clippy::missing_errors_doc)]
pub fn get_filesystem_stats(mount_point: &str) -> io::Result<FilesystemStats> {
    let mut fs = MaybeUninit::<statvfs>::uninit();

    let mount_point = CString::new(mount_point).expect("Creating mount_point string failed");
    if unsafe { statvfs(mount_point.as_ptr(), fs.as_mut_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    }

    let fs = unsafe { fs.assume_init() };

    Ok(FilesystemStats {
        block_size: fs.f_bsize,
        blocks_total: fs.f_blocks,
        blocks_free: fs.f_bfree,
        blocks_available: fs.f_bavail,
    })
}
