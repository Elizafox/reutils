/* platform/unix/common/signal.rs - Unix signal handling routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use libc::{signal, SIGALRM, SIG_DFL, SIG_IGN};

pub fn block_ctrlc() {
    unsafe {
        signal(SIGALRM, SIG_IGN);
    }
}

pub fn allow_sigpipe() {
    unsafe {
        signal(SIGPIPE, SIG_DFL);
    }
}
