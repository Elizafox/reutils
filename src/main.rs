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
mod utils;
mod version;

use std::env;
use std::path::Path;
use std::process::exit;

use crate::err::Result;
use crate::utils::DISPATCH_TABLE;

#[cfg(target_os = "windows")]
fn get_util_name(arg0: &str) -> String {
    if &arg0[arg0.len() - 3..] == "exe" {
        // If the file ends in .exe, strip it off.
        return String::from(
            Path::new(&arg0)
                .file_stem()
                .expect("Failed to get path name!")
                .to_str()
                .expect("Failed to get path name!"),
        );
    } else {
        // Use the usual implementation
        return String::from(
            Path::new(&arg0)
                .file_name()
                .expect("Failed to get path name!")
                .to_str()
                .expect("Failed to get path name!"),
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn get_util_name(arg0: &str) -> String {
    String::from(
        Path::new(&arg0)
            .file_name()
            .expect("Failed to get path name!")
            .to_str()
            .expect("Failed to get path name!"),
    )
}

fn do_exit(result: Result) -> ! {
    match result {
        Ok(_) => exit(0),
        Err(e) => {
            if let Some(message) = e.message {
                eprintln!("{message}");
            }

            exit(e.code)
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let util = get_util_name(&args[0]);

    // Attempt to find the utility
    if let Some(util_entry) = DISPATCH_TABLE.get(&util).copied() {
        do_exit(util_entry.1(args.as_slice()))
    }

    eprintln!("{util}: utility not found");
    exit(1);
}
