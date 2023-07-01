/* platform/windows/signal.rs - Windows signal handling routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::ptr;

use windows::Win32::System::Console::{SetConsoleCtrlHandler, PHANDLER_ROUTINE};

pub fn block_ctrlc() {
    SetConsoleCtrlHandler(ptr::null_ptr(), true);
}

pub const fn allow_sigpipe() {
    // No-op
}
