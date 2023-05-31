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

#[cfg(not(target_os = "windows"))]
fn block_sigalrm() {
    unsafe {
        libc::signal(libc::SIGALRM, libc::SIG_IGN);
    }
}

#[cfg(target_os = "windows")]
const fn block_sigalrm() {}

pub fn util(args: &[String]) -> Result {
    block_sigalrm(); // POSIX sez this is a valid option

    let arg = args.get(1).ok_or_else(|| usage(&args[0]))?;

    if arg.starts_with('-') {
        return Err(usage(&args[0]));
    }

    // We check the sign above, so we don't care.
    // We also don't have a better method to convert, unless we use nightly, so meh.
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let sleep_nsec = (arg.parse::<f32>().map_err(|_| usage(&args[0]))? * 1e9).round() as u64;

    sleep(Duration::from_nanos(sleep_nsec));

    Ok(())
}
