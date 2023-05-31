/* utils/reutils.rs - Implementation of the reutils command
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::process::Command;

use getargs::{Opt, Options};

use crate::err::{Error, Result};
use crate::install::perform;
use crate::utils::DISPATCH_TABLE;
use crate::version::about;

pub fn util(args: &[String]) -> Result {
    if args.len() <= 1 {
        about(false);
        return Err(Error::new_nomsg(1));
    }

    // Parse opts
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!(
                    "Usage: {} [--version|-v] [--install [basedir]] [-h|--help] | [utility] ...",
                    args[0]
                );
                return Ok(());
            }
            Opt::Short('v') | Opt::Long("version") => {
                about(true);
                return Ok(());
            }
            Opt::Long("install") => {
                let prefix = opts.value().unwrap_or("");
                return perform(prefix);
            }
            _ => {}
        }
    }

    // Determine if what we're executing is a builtin
    // If it is, run it and leave.
    if let Some(util_entry) = DISPATCH_TABLE.get(&args[1]).copied() {
        return util_entry.1(&args[1..]);
    }

    let status = Command::new(&args[1]).args(args.iter().skip(1)).status();
    match status {
        Ok(status) => status.code().map_or_else(
            || Err(Error::new(255, "Process terminated by signal".to_string())),
            |code| Err(Error::new(code, format!("Exited with status code {code}"))),
        ),
        Err(e) => Err(Error::new(255, format!("Could not execute command: {e}"))),
    }
}
