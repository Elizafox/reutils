/* utils/pwd.rs - implementation of pwd
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::env;
use std::io;

use getargs::{Opt, Options};

use crate::err::Result;

#[cfg(unix)]
fn getcwd_logical() -> io::Result<String> {
    use std::fs::metadata;
    use std::os::unix::fs::MetadataExt;

    let pwd = env::var("PWD")
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get PWD env var"))?;
    if pwd.starts_with('/') {
        let logical_md = metadata(&pwd)?;
        let physical_md = metadata(".")?;

        if logical_md.dev() == physical_md.dev() && logical_md.ino() == physical_md.ino() {
            return Ok(pwd);
        }
    }

    Err(io::Error::from(io::ErrorKind::InvalidData))
}

#[cfg(windows)]
fn getcwd_logical() -> io::Result<String> {
    // Might be set by MingW
    let pwd = env::var("PWD")
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get PWD env var"))?;
    Ok(pwd)
}

#[allow(clippy::unnecessary_wraps)]
pub fn util(args: &[String]) -> Result {
    let mut logical: bool = true;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('L') => logical = true,
            Opt::Short('P') => logical = false,
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("Usage: {} [-L|-P]", args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    if logical {
        // If this fails we will try physical
        if let Ok(dir) = getcwd_logical() {
            println!("{dir}");
            return Ok(());
        }
    }

    env::current_dir().map_or_else(
        |_| println!("."),
        |dir| println!("{}", dir.to_str().unwrap_or(".")),
    );

    Ok(())
}
