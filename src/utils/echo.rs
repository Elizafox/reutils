/* utils/echo.rs - implementation of echo
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn util(args: &[String]) -> Result {
    if args.len() > 1 {
        for arg in &args[1..] {
            println!("{arg}");
        }
    } else {
        println!();
    }

    Ok(())
}
