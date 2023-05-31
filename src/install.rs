/* install.rs - installation routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::env::{args_os, consts::EXE_SUFFIX, current_exe};
use std::fs::{canonicalize, copy, create_dir_all};
use std::path::{Path, PathBuf};

use crate::err::{Error, Result};
use crate::utils::DISPATCH_TABLE;

#[derive(Copy, Clone)]
enum ErrorCode {
    //Unknown = 1,  // Reserved
    //InvalidArgument = 2,  // Reserved
    CreateDir = 3,
    Symlink = 4,
    Copy = 5,
    #[cfg(windows)]
    Privilege = 6, // Windows only for now, reserved for Unix
}

#[cfg(unix)]
const DEFAULT_PREFIX: &str = "/";
#[cfg(windows)]
const DEFAULT_PREFIX: &str = "C:\\Program Files\\reutils"; // XXX

#[cfg(unix)]
fn create_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(p: &P, q: &Q) -> std::io::Result<()> {
    use std::os::unix::fs::symlink;
    symlink(p, q)?;
    Ok(())
}

#[cfg(windows)]
fn create_file_symlink<P: AsRef<Path>, Q: AsRef<Path>>(p: &P, q: &Q) -> std::io::Result<()> {
    use std::os::windows::fs::symlink_file;
    symlink_file(p, q)?;
    Ok(())
}

fn current_exe_path() -> PathBuf {
    let arg0 = args_os().next().expect("Could not get arg0");
    let name = Path::new(&arg0)
        .file_name()
        .expect("Could not get binary name");
    let mut path = current_exe().expect("Could not get exe path");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    let mut name = name.to_os_string();
    name.push(EXE_SUFFIX);
    path.push(&name);
    canonicalize(&path).unwrap_or(path)
}

pub fn perform(prefix: &str) -> Result {
    let prefix = if prefix.is_empty() {
        Path::new(DEFAULT_PREFIX)
    } else {
        Path::new(prefix)
    };

    let reutils_exe_base_path = DISPATCH_TABLE.get("reutils").copied().unwrap().0;
    let reutils_exe_path = prefix.join(reutils_exe_base_path);

    #[cfg(windows)]
    if !is_elevated::is_elevated() {
        Err(Error::new(
            ErrorCode::Privilege as i32,
            "Admin privileges are required on Windows to install",
        ))
    }

    println!("Starting installation");

    // Install the binary if we must
    let current_exe_path = current_exe_path();
    if current_exe_path == reutils_exe_path {
        println!(
            "reutils binary located at {}",
            reutils_exe_path.to_string_lossy()
        );
    } else {
        println!(
            "Copying reutils binary from {} => {}",
            current_exe_path.to_string_lossy(),
            reutils_exe_path.to_string_lossy()
        );

        let parent = reutils_exe_path.parent().unwrap();
        if !parent.exists() {
            println!("Creating prefix directory {}", parent.to_str().unwrap());
            create_dir_all(parent).map_err(|e| {
                Error::new(
                    ErrorCode::CreateDir as i32,
                    format!("Could not create prefix directory: {e}"),
                )
            })?;
        }

        copy(&current_exe_path, &reutils_exe_path).map_err(|e| {
            Error::new(
                ErrorCode::Copy as i32,
                format!("Could not copy reutils binary: {e}"),
            )
        })?;
    }

    // Install the utilities
    for (util, (util_path, _)) in &DISPATCH_TABLE {
        let util_path = prefix.join(util_path);
        if util_path.exists() {
            println!("Skipping {util} as it is already installed");
            continue;
        }

        println!("Installing {util} => {}", util_path.to_str().unwrap());

        let parent = util_path.parent().unwrap();
        if !parent.exists() {
            println!("Creating directory {}", parent.to_str().unwrap());
            create_dir_all(parent).map_err(|e| {
                Error::new(
                    ErrorCode::CreateDir as i32,
                    format!("Directory could not be created: {e}"),
                )
            })?;
        }

        create_file_symlink(&reutils_exe_path, &util_path).map_err(|e| {
            Error::new(
                ErrorCode::Symlink as i32,
                format!("Could not create symlink: {e}"),
            )
        })?;
    }

    Ok(())
}
