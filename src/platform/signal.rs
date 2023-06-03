/* platform/signal.rs - platform signal routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#[cfg(unix)]
pub use crate::platform::unix::common::signal::*;

#[cfg(windows)]
pub use crate::platform::windows::signal::*;
