/* utils/reutils.rs - Implementation of the reutils command
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::process::Command;

use crate::utils::DISPATCH_TABLE;
use crate::err::{AppletError, AppletResult};

pub fn util_reutils(args: Vec<String>) -> AppletResult
{
    if args.len() <= 1
    {
        // FIXME FIXME FIXME!!!
        eprintln!("reutils v0.0.0");
        eprintln!("This program is free software.");
        return Err(AppletError::new_nomsg(1));
    }

    // Determine if what we're executing is a builtin
    // If it is, run it and leave.
    let util_entry = DISPATCH_TABLE.get(&args[1]).cloned();
    if util_entry.is_some()
    {
        return util_entry.unwrap().1(args.into_iter().skip(1).collect());
    }

    let status = Command::new(&args[1]).args(args.iter().skip(1)).status();
    match status
    {
        Ok(status) =>
        {
            match status.code()
            {
                Some(code) =>
                {
                    eprintln!("Exited with status code {code}");
                    Err(AppletError::new(code, format!("Exited with status code {code}")))
                },
                None =>
                {
                    Err(AppletError::new(255, format!("Process terminated by signal")))
                }
            }
        },
        Err(e) =>
        {
            Err(AppletError::new(255, format!("Could not execute command: {}", e)))
        }
    }

}
