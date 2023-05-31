/* utils/tty.rs - implementation of tty
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

#[cfg(unix)]
fn ttyname() -> Result<String> {
    use std::ffi::CStr;
    use std::io;
    use std::os::fd::{AsFd, AsRawFd, BorrowedFd};

    let stdin = io::stdin();
    let bfd: BorrowedFd<'_> = stdin.as_fd();
    let fd = bfd.as_raw_fd();
    let name_ptr = unsafe { libc::ttyname(fd) };

    if name_ptr.is_null() {
        // Uh oh!
        let e = io::Error::last_os_error();
        return Err(Error::new(1, format!("Could not get TTY name: {e}")));
    }

    let c_name = unsafe { CStr::from_ptr(name_ptr) };
    let name = c_name
        .to_str()
        .map_err(|e| Error::new(1, format!("Could not get TTY name: {e}")))?;

    Ok(name.to_string())
}

#[cfg(windows)]
fn ttyname() -> Result<String> {
    /* FIXME - MingW does emulate a pty device... but we can't rely on it.
     * Windows does have some PTY stuff in modern versions, but, idk if we can use it?
     * Hard to say.
     * --Elizafox
     */
    Err(Error::new(255, "Not implemented on Windows".to_string()))
}

pub fn util(_args: &[String]) -> Result {
    println!("{}", ttyname()?);
    Ok(())
}
