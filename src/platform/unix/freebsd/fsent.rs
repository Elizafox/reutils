/* platform/unix/freebsd/fsent.rs - FreeBSD filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::ffi::{CStr, CString};
use std::io;
use std::mem::MaybeUninit;
use std::process::exit;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

use libc::{
    getmntinfo, statfs, statvfs, MNT_ACLS, MNT_ASYNC, MNT_AUTOMOUNTED, MNT_DELEXPORT, MNT_EMPTYDIR,
    MNT_EXPORTED, MNT_FORCE, MNT_GJOURNAL, MNT_LOCAL, MNT_MULTILABEL, MNT_NFS4ACLS, MNT_NOATIME,
    MNT_NOCLUSTERR, MNT_NOCLUSTERW, MNT_NOCOVER, MNT_NOEXEC, MNT_NOSUID, MNT_NOSYMFOLLOW,
    MNT_NOWAIT, MNT_QUOTA, MNT_RDONLY, MNT_RELOAD, MNT_SNAPSHOT, MNT_SOFTDEP, MNT_SUIDDIR, MNT_SUJ,
    MNT_SYNCHRONOUS, MNT_UNION, MNT_UNTRUSTED, MNT_UPDATE, MNT_VERIFIED,
};

use crate::platform::fsent::*;

// These constants were taken from FreeBSD's sys/mount.h
fn get_mount_options(fs: &statfs) -> String {
    let mut opts = Vec::<&str>::new();
    let flags = fs.f_flags as u64;

    if flags & (MNT_ASYNC as u64) != 0 {
        opts.push("asynchronous");
    }
    if flags & (MNT_EXPORTED as u64) != 0 {
        opts.push("NFS exported");
    }
    if flags & (MNT_LOCAL as u64) != 0 {
        opts.push("local");
    }
    if flags & (MNT_NOATIME as u64) != 0 {
        opts.push("noatime");
    }
    if flags & (MNT_NOEXEC as u64) != 0 {
        opts.push("noexec");
    }
    if flags & (MNT_NOSUID as u64) != 0 {
        opts.push("nosuid");
    }
    if flags & (MNT_NOSYMFOLLOW as u64) != 0 {
        opts.push("nosymfollow");
    }
    if flags & (MNT_QUOTA as u64) != 0 {
        opts.push("with quotas");
    }
    if flags & (MNT_RDONLY as u64) != 0 {
        opts.push("read-only");
    }
    if flags & (MNT_SYNCHRONOUS as u64) != 0 {
        opts.push("synchronous");
    }
    if flags & (MNT_UNION as u64) != 0 {
        opts.push("union");
    }
    if flags & (MNT_NOCLUSTERR as u64) != 0 {
        opts.push("noclusterr");
    }
    if flags & (MNT_NOCLUSTERW as u64) != 0 {
        opts.push("noclusterw");
    }
    if flags & (MNT_SUIDDIR as u64) != 0 {
        opts.push("suiddir");
    }
    if flags & (MNT_SOFTDEP as u64) != 0 {
        opts.push("soft-updates");
    }
    if flags & (MNT_SUJ as u64) != 0 {
        opts.push("journaled soft-updates");
    }
    if flags & (MNT_MULTILABEL as u64) != 0 {
        opts.push("multilabel");
    }
    if flags & (MNT_ACLS as u64) != 0 {
        opts.push("acls");
    }
    if flags & (MNT_NFS4ACLS as u64) != 0 {
        opts.push("nfsv4acls");
    }
    if flags & (MNT_GJOURNAL as u64) != 0 {
        opts.push("gjournal");
    }
    if flags & (MNT_AUTOMOUNTED as u64) != 0 {
        opts.push("automounted");
    }
    if flags & (MNT_VERIFIED as u64) != 0 {
        opts.push("verified");
    }
    if flags & (MNT_UNTRUSTED as u64) != 0 {
        opts.push("untrusted");
    }
    if flags & (MNT_NOCOVER as u64) != 0 {
        opts.push("nocover");
    }
    if flags & (MNT_EMPTYDIR as u64) != 0 {
        opts.push("emptydir");
    }
    if flags & (MNT_UPDATE as u64) != 0 {
        opts.push("update");
    }
    if flags & (MNT_DELEXPORT as u64) != 0 {
        opts.push("delexport");
    }
    if flags & (MNT_RELOAD as u64) != 0 {
        opts.push("reload");
    }
    if flags & (MNT_FORCE as u64) != 0 {
        opts.push("force");
    }
    if flags & (MNT_SNAPSHOT as u64) != 0 {
        opts.push("snapshot");
    }

    format!("({})", opts.join(", "))
}

pub fn get_mounted_filesystems() -> io::Result<Vec<FilesystemEntry>> {
    let mut result = Vec::<FilesystemEntry>::new();

    let mut mounts: *mut statfs = null_mut();
    let mountlen = unsafe { getmntinfo(&mut mounts, MNT_NOWAIT.into()) };
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

    Ok(result)
}

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
