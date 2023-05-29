/* utils/dirname.rs - implementation of dirname
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

fn usage(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} path");
    Error::new_nomsg(1)
}

#[cfg(target_os = "windows")]
fn dirname(path: &str) -> Result<String, Error> {
    use std::path::Path;
    Ok(match Path::new(&path).parent() {
        Some(base) => String::from(
            base.to_str()
                .ok_or_else(|| Error::new(1, format!("Could not convert path")))?,
        ),
        None => String::from(path),
    })
}

#[cfg(not(target_os = "windows"))]
fn dirname(path: &str) -> Result<String, Error> {
    use std::ffi::{CStr, CString};
    use std::mem::transmute;

    let dn;
    let path = CString::new(path)
        .map_err(|e| Error::new(1, format!("Could not get C string from path: {e}")))?;

    unsafe {
        let mut path_buf: Vec<libc::c_char> = transmute(path.into_bytes_with_nul());
        dn = CStr::from_ptr(libc::dirname(path_buf.as_mut_ptr()))
            .to_str()
            .map_err(|e| Error::new(1, format!("Could not convert string: {e}")))?;
    }

    Ok(String::from(dn))
}

pub fn util_dirname(args: Vec<String>) -> Result {
    let path = args.get(1).ok_or_else(|| usage(&args[0]))?;

    println!("{}", dirname(path)?);

    Ok(())
}
