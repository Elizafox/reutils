/* main.rs - entrypoint for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod err;
mod bufinput;
mod utils;

use std::env;
use std::path::Path;
use std::process::exit;

use crate::utils::DISPATCH_TABLE;

fn main() {
    // 2023-05-24 AMR TODO: Vec<&str>? &[&str]? Vec<_>?
    let args: Vec<String> = env::args().collect();
    let util = Path::new(&args[0])
        .file_name()
        .expect("Failed to get path name!")
        .to_str()
        .expect("Failed to get path name!");

    // Attempt to find the utility
    if let Some(util_entry) = DISPATCH_TABLE.get(util).cloned() {
        match util_entry.1(args) {
            Ok(_) => exit(0),
            Err(e) => {
                eprintln!("failed {e:?}");
                if let Some(message) = e.message {
                    eprintln!("{}", message);
                }

                exit(e.code)
            }
        }
    }

    eprintln!("{}: utility not found", util);
    exit(1);
}
