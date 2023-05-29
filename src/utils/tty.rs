/* utils/tty.rs - implementation of tty
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

#[cfg(unix)]
fn ttyname() -> Result<String> {
    use std::io;
    use std::os::fd::{AsFd, AsRawFd, BorrowedFd};
    use std::ffi::CStr;

    let stdin = io::stdin();
    let bfd: BorrowedFd<'_> = stdin.as_fd();
    let fd = bfd.as_raw_fd();
    let name_ptr = unsafe {
        libc::ttyname(fd)
    };

    if name_ptr.is_null() {
        // Uh oh!
        let e = io::Error::last_os_error();
        return Err(Error::new(1, format!("Could not get TTY name: {e}")));
    }

    let c_name = unsafe {
        CStr::from_ptr(name_ptr)
    };
    let name = c_name
        .to_str()
        .map_err(|e| Error::new(1, format!("Could not get TTY name: {e}")))?;

    return Ok(String::from(name));
}

#[cfg(windows)]
fn ttyname() -> Result<String> {
    return Err(Error::new(255, "Not implemented".to_string()));
}

pub fn util(_args: &[String]) -> Result {
    println!("{}", ttyname()?);
    Ok(())
}
