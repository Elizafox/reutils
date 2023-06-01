/* utils/ln_link.rs - implementation of ln and link
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::path::Path;

use getargs::{Opt, Options};

use crate::err::{Error, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
enum LinkType {
    HardlinkNormal,
    Symlink,
    HardlinkToSymlink,
    HardlinkToSymlinkDirect,
}

#[cfg(unix)]
fn perform_link<A: AsRef<Path>, B: AsRef<Path>>(
    old: &A,
    new: &B,
    link_type: LinkType,
    force: bool,
) -> Result {
    use libc::{link, linkat, symlink, unlink, AT_FDCWD, AT_SYMLINK_FOLLOW};
    use std::{ffi::CString, io, os::unix::ffi::OsStrExt};

    let old_c_str = CString::new(old.as_ref().as_os_str().as_bytes()).unwrap();
    let new_c_str = CString::new(new.as_ref().as_os_str().as_bytes()).unwrap();

    if force && new.as_ref().exists() {
        let ret = unsafe { unlink(new_c_str.as_ptr()) };
        if ret != 0 {
            return Err(Error::new(
                1,
                format!("Could not unlink: {}", io::Error::last_os_error()),
            ));
        }
    }

    match link_type {
        LinkType::Symlink => {
            let ret = unsafe { symlink(old_c_str.as_ptr(), new_c_str.as_ptr()) };
            if ret != 0 {
                return Err(Error::new(
                    1,
                    format!("Could not do symlink: {}", io::Error::last_os_error()),
                ));
            }
        }
        LinkType::HardlinkNormal => {
            let ret = unsafe { link(old_c_str.as_ptr(), new_c_str.as_ptr()) };
            if ret != 0 {
                return Err(Error::new(
                    1,
                    format!("Could not do hard link: {}", io::Error::last_os_error()),
                ));
            }
        }
        LinkType::HardlinkToSymlink => {
            let ret = unsafe {
                linkat(
                    AT_FDCWD,
                    old_c_str.as_ptr(),
                    AT_FDCWD,
                    new_c_str.as_ptr(),
                    0,
                )
            };
            if ret != 0 {
                return Err(Error::new(
                    1,
                    format!("Could not do hard link: {}", io::Error::last_os_error()),
                ));
            }
        }
        LinkType::HardlinkToSymlinkDirect => {
            let ret = unsafe {
                linkat(
                    AT_FDCWD,
                    old_c_str.as_ptr(),
                    AT_FDCWD,
                    new_c_str.as_ptr(),
                    AT_SYMLINK_FOLLOW,
                )
            };
            if ret != 0 {
                return Err(Error::new(
                    1,
                    format!("Could not do hard link: {}", io::Error::last_os_error()),
                ));
            }
        }
    }

    Ok(())
}

#[cfg(windows)]
fn perform_link<A: AsRef<Path>, B: AsRef<Path>>(
    old: &A,
    new: &B,
    link_type: LinkType,
    force: bool,
) -> Result {
    use std::fs::hard_link;
    use symlink::{remove_symlink_auto, symlink_auto};

    if force && new.as_ref().exists() {
        remove_symlink_auto(new)
            .map_err(|e| Error::new(1, format!("Could not remove file or directory: {e}")))?;
    }

    if link_type == LinkType::Symlink {
        symlink_auto(old, new)
            .map_err(|e| Error::new(1, format!("Could not create symlink: {e}")))?;
    } else {
        if link_type == LinkType::HardlinkToSymlink
            || link_type == LinkType::HardlinkToSymlinkDirect
        {
            // FIXME
            eprintln!("WARNING: -F and -L are presently ignored on Windows");
        }

        hard_link(old, new)
            .map_err(|e| Error::new(1, format!("Could not create hard link: {e}")))?;
    }

    Ok(())
}

fn usage_ln(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} [-fs] [-L|-P] source_file target_file");
    eprintln!("Usage: {arg0} [-fs] [-L|-P] source_file... target_dir");
    Error::new_nomsg(1)
}

fn usage_link(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} source_file target_file");
    Error::new_nomsg(1)
}

pub fn util_ln(args: &[String]) -> Result {
    let mut link_type: LinkType = LinkType::HardlinkNormal;
    let mut force = false;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                usage_ln(&args[0]);
                return Ok(());
            }
            Opt::Short('s') => link_type = LinkType::Symlink,
            Opt::Short('f') => force = true,
            Opt::Short('L') => {
                if link_type == LinkType::HardlinkToSymlinkDirect {
                    eprintln!("Error: -L conflicts with -P");
                    return Err(usage_ln(&args[0]));
                }
                link_type = LinkType::HardlinkToSymlink;
            }
            Opt::Short('P') => {
                if link_type == LinkType::HardlinkToSymlink {
                    eprintln!("Error: -P conflicts with -L");
                    return Err(usage_ln(&args[0]));
                }
                link_type = LinkType::HardlinkToSymlinkDirect;
            }
            _ => {}
        }
    }

    let mut positionals = opts.positionals().collect::<Vec<_>>();
    match positionals.len() {
        0 | 1 => return Err(usage_ln(&args[0])),
        2 => {
            let old = Path::new(positionals.first().expect("Could not get argument"));
            let new = Path::new(positionals.get(1).expect("Could not get argument"));
            perform_link(&old, &new, link_type, force)?;
        }
        _ => {
            let dir = Path::new(positionals.pop().unwrap());
            if !dir.exists() {
                return Err(Error::new(
                    1,
                    "Error creating links: no such directory".to_string(),
                ));
            }

            for file in positionals {
                let old = Path::new(file);
                let new = dir.join(file);
                if let Err(e) = perform_link(&old, &new, link_type, force) {
                    eprintln!(
                        "Error creating {}: {}",
                        new.to_string_lossy(),
                        e.message.unwrap_or_else(|| "unknown".to_string())
                    );
                }
            }
        }
    }

    Ok(())
}

// The implementation of link is significantly simpler
pub fn util_link(args: &[String]) -> Result {
    if args.len() < 3 {
        return Err(usage_link(&args[0]));
    }

    let old = Path::new(&args[1]);
    let new = Path::new(&args[2]);
    perform_link(&old, &new, LinkType::HardlinkNormal, false)?;
    Ok(())
}
