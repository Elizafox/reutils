/* platform.rs - platform-specific routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

pub mod signal;
pub mod fsent;
