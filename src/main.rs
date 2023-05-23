/* main.rs - entrypoint for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

mod utils;
mod err;

use std::env;
use std::process::exit;
use std::path::Path;

use crate::utils::DISPATCH_TABLE;

fn main()
{
    let args: Vec<String> = env::args().collect();
    let util = Path::new(&args[0]).file_name().expect("Failed to get path name!").to_str().expect("Failed to get path name!");

    // Attempt to find the utility
    let util_entry = DISPATCH_TABLE.get(util).cloned();
    if util_entry.is_some()
    {
        match util_entry.unwrap().1(args)
        {
            Ok(_) => exit(0),
            Err(e) =>
            {
                if let Some(message) = e.message
                {
                    eprintln!("{}", message);
                }

                exit(e.status_code)
            }
        }
    }

    eprintln!("{}: utility not found", util);
    exit(1);
}
