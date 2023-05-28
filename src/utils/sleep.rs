/* utils/sleep.rs - implementation of sleep
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::thread::sleep;
use std::time::Duration;

use crate::err::{Error, Result};

fn usage(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} seconds");
    Error::new_nomsg(1)
}

fn block_sigalrm() {
    unsafe {
        libc::signal(libc::SIGALRM, libc::SIG_IGN);
    }
}

pub fn util_sleep(args: Vec<String>) -> Result {
    block_sigalrm(); // POSIX sez this is a valid option

    let sleep_nsec = (args
        .get(1)
        .ok_or_else(|| usage(&args[0]))?
        .parse::<f64>()
        .map_err(|_| usage(&args[0]))?
        * 1e9)
        .round() as u64;

    sleep(Duration::from_nanos(sleep_nsec));

    Ok(())
}
