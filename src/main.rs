/* main.rs - entrypoint for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)] -- too many false positives

mod bufinput;
mod bufoutput;
mod err;
mod install;
mod platform;
mod utils;
mod version;

use std::env;
use std::path::Path;
use std::process::exit;

use crate::err::{Error, Result};
use crate::utils::DISPATCH_TABLE;
use crate::platform::signal::allow_sigpipe;

#[cfg(target_os = "windows")]
fn get_util_name(arg0: &str) -> String {
    if &arg0[arg0.len() - 3..] == "exe" {
        // If the file ends in .exe, strip it off.
        return Path::new(&arg0)
            .file_stem()
            .expect("Failed to get path name!")
            .to_str()
            .expect("Failed to get path name!")
            .to_string();
    }

    // Use the usual implementation
    return Path::new(&arg0)
        .file_name()
        .expect("Failed to get path name!")
        .to_str()
        .expect("Failed to get path name!")
        .to_string();
}

#[cfg(not(target_os = "windows"))]
fn get_util_name(arg0: &str) -> String {
    Path::new(&arg0)
        .file_name()
        .expect("Failed to get path name!")
        .to_str()
        .expect("Failed to get path name!")
        .to_string()
}

fn do_exit(result: Result) -> ! {
    match result {
        Ok(_) => exit(0),
        Err(e) => {
            if e.message.is_some() {
                eprintln!("{e}");
            }
            exit(e.code)
        }
    }
}

fn main() {
    // Rust blocks SIGPIPE by default, we have to restore it.
    allow_sigpipe();

    let args: Vec<_> = env::args().collect();
    let util = get_util_name(&args[0]);

    // Attempt to find the utility
    if let Some(util_entry) = DISPATCH_TABLE.get(&util).copied() {
        do_exit(util_entry.1(args.as_slice()))
    }

    do_exit(Err(Error::new(1, format!("{util}: utility not found"))));
}
