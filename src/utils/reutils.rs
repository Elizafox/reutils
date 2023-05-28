/* utils/reutils.rs - Implementation of the reutils command
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::process::Command;

use crate::err::{Error, Result};
use crate::utils::DISPATCH_TABLE;
use crate::version;

pub fn util_reutils(args: Vec<String>) -> Result {
    if args.len() <= 1 {
        eprintln!("reutils v{}-{}", version::VERSION, version::GIT_REVISION);
        eprintln!("Copyright (C) 2023 {}", version::AUTHORS.replace(':', ", "));
        eprintln!("This program is free software; you may redistribute it under the terms of");
        eprintln!("the GNU General Public License version 2 ONLY.");
        eprintln!("TThis program has absolutely no warranty.");
        return Err(Error::new_nomsg(1));
    }

    // Determine if what we're executing is a builtin
    // If it is, run it and leave.
    if let Some(util_entry) = DISPATCH_TABLE.get(&args[1]).cloned() {
        return util_entry.1(args.into_iter().skip(1).collect());
    }

    let status = Command::new(&args[1]).args(args.iter().skip(1)).status();
    match status {
        Ok(status) => match status.code() {
            Some(code) => {
                eprintln!("Exited with status code {code}");
                Err(Error::new(code, format!("Exited with status code {code}")))
            }
            None => Err(Error::new(255, "Process terminated by signal".to_string())),
        },
        Err(e) => Err(Error::new(255, format!("Could not execute command: {}", e))),
    }
}
