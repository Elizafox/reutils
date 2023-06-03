/* platform/unix/macos/fsent.rs - macOS filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::ffi::{c_int, c_void, CStr, CString};
use std::io;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

use libc::{
    free, statfs, statvfs, MNT_ASYNC, MNT_AUTOMOUNTED, MNT_CPROTECT, MNT_DEFWRITE, MNT_DONTBROWSE,
    MNT_DOVOLFS, MNT_EXPORTED, MNT_IGNORE_OWNERSHIP, MNT_JOURNALED, MNT_LOCAL, MNT_MULTILABEL,
    MNT_NOATIME, MNT_NODEV, MNT_NOEXEC, MNT_NOSUID, MNT_NOUSERXATTR, MNT_NOWAIT, MNT_QUARANTINE,
    MNT_QUOTA, MNT_RDONLY, MNT_ROOTFS, MNT_SNAPSHOT, MNT_SYNCHRONOUS, MNT_UNION,
};

use crate::platform::fsent::*;

// Not exported by libc. Sigh. --Elizafox
extern "C" {
    pub fn getmntinfo_r_np(mntbufp: *mut *mut statfs, flags: c_int) -> c_int;
}

// macOS doesn't provide this as a string, let's do that!
fn get_mount_options(fs: &statfs) -> String {
    let flags = fs.f_flags as i32;
    let mut opts = Vec::<&str>::new();

    if flags & MNT_RDONLY != 0 {
        opts.push("ro");
    }
    if flags & MNT_SYNCHRONOUS != 0 {
        opts.push("sync");
    }
    if flags & MNT_NOEXEC != 0 {
        opts.push("noexec");
    }
    if flags & MNT_NOSUID != 0 {
        opts.push("nosuid");
    }
    if flags & MNT_NODEV != 0 {
        opts.push("nodev");
    }
    if flags & MNT_UNION != 0 {
        opts.push("union");
    }
    if flags & MNT_ASYNC != 0 {
        opts.push("async");
    }
    if flags & MNT_EXPORTED != 0 {
        opts.push("export");
    }
    if flags & MNT_LOCAL != 0 {
        opts.push("local");
    }
    if flags & MNT_EXPORTED != 0 {
        opts.push("export");
    }
    if flags & MNT_QUARANTINE != 0 {
        opts.push("quarantine");
    }
    if flags & MNT_QUOTA != 0 {
        opts.push("quota");
    }
    if flags & MNT_ROOTFS != 0 {
        opts.push("root");
    }
    if flags & MNT_DOVOLFS != 0 {
        opts.push("volfs");
    }
    if flags & MNT_DONTBROWSE != 0 {
        opts.push("nobrowse");
    }
    if flags & MNT_IGNORE_OWNERSHIP != 0 {
        opts.push("noperms");
    }
    if flags & MNT_AUTOMOUNTED != 0 {
        opts.push("automount");
    }
    if flags & MNT_JOURNALED != 0 {
        opts.push("journal");
    }
    if flags & MNT_DEFWRITE != 0 {
        opts.push("defwrite");
    }
    if flags & MNT_MULTILABEL != 0 {
        opts.push("multilabel");
    }
    if flags & MNT_CPROTECT != 0 {
        opts.push("cprotect");
    }
    if flags & MNT_NOUSERXATTR != 0 {
        opts.push("nouserxattr");
    }
    if flags & MNT_NOATIME != 0 {
        opts.push("noatime");
    }
    if flags & MNT_SNAPSHOT != 0 {
        opts.push("snapshot");
    }

    format!("({})", opts.join(", "))
}

#[ignore(clippy::missing_errors_doc)]
pub fn get_mounted_filesystems() -> io::Result<Vec<FilesystemEntry>> {
    let mut result = Vec::<FilesystemEntry>::new();

    let mut mounts: *mut statfs = null_mut();
    let mountlen = unsafe { getmntinfo_r_np(&mut mounts, MNT_NOWAIT.into()) };
    if mountlen == 0 {
        return Err(io::Error::last_os_error());
    }

    let slice = unsafe { from_raw_parts(mounts, mountlen as usize) };

    for fs in slice {
        result.push(unsafe {
            FilesystemEntry {
                filesystem_name: CStr::from_ptr(fs.f_fstypename.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                mount_point: CStr::from_ptr(fs.f_mntonname.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                mount_from: CStr::from_ptr(fs.f_mntfromname.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                mount_options: get_mount_options(&fs),
            }
        });
    }

    unsafe {
        free(mounts as *mut c_void);
    }
    Ok(result)
}

#[ignore(clippy::missing_errors_doc)]
pub fn get_filesystem_stats(mount_point: &str) -> io::Result<FilesystemStats> {
    let mut fs = MaybeUninit::<statvfs>::uninit();

    let mount_point = CString::new(mount_point).expect("Creating mount_point string failed");
    if unsafe { statvfs(mount_point.as_ptr(), fs.as_mut_ptr()) } < 0 {
        return Err(io::Error::last_os_error());
    }

    let fs = unsafe { fs.assume_init() };

    Ok(FilesystemStats {
        block_size: fs.f_bsize.into(),
        blocks_total: fs.f_blocks.into(),
        blocks_free: fs.f_bfree.into(),
        blocks_available: fs.f_bavail.into(),
    })
}
