/* utils/basename.rs - implementation of basename
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

fn usage(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} path");
    Error::new_nomsg(1)
}

#[cfg(target_os = "windows")]
fn basename(path: &str) -> Result<String, Error> {
    use std::path::Path;
    Ok(match Path::new(&path).file_name() {
        Some(base) => String::from(
            base.to_str()
                .ok_or_else(|| Error::new(1, format!("Could not convert path")))?,
        ),
        None => String::from(path),
    })
}

#[cfg(not(target_os = "windows"))]
fn basename(path: &str) -> Result<String, Error> {
    /* XXX hack hack hack hack hack hack hack hack hack!!!
     * Workaround libc crate bug; it doesn't export posix_basename on BSD for some reason.
     */
    #[cfg(target_os = "linux")]
    const LIBC_BASENAME: unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_char =
        libc::posix_basename;

    #[cfg(not(target_os = "linux"))]
    const LIBC_BASENAME: unsafe extern "C" fn(*mut libc::c_char) -> *mut libc::c_char =
        libc::basename;

    use std::ffi::{CStr, CString};
    use std::mem::transmute;

    let dn;
    let path = CString::new(path)
        .map_err(|e| Error::new(1, format!("Could not get C string from path: {e}")))?;

    unsafe {
        let mut path_buf: Vec<libc::c_char> = transmute(path.into_bytes_with_nul());
        dn = CStr::from_ptr(LIBC_BASENAME(path_buf.as_mut_ptr()))
            .to_str()
            .map_err(|e| Error::new(1, format!("Could not convert string: {e}")))?;
    }

    return Ok(String::from(dn));
}

pub fn util_basename(args: Vec<String>) -> Result {
    let path = args.get(1).ok_or_else(|| usage(&args[0]))?;

    println!("{}", basename(path)?);

    Ok(())
}
