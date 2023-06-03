/* platform.rs - platform-specific routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

/* The idea behind this module is that it smooths over major platform-specific differences and
 * commonly-used idioms and code. It presents just enough abstraction to make utilities work. Most
 * utilities won't need this, but some stuff is *heavily* platform-specific, and therefore they do.
 * --Elizafox
 */

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

pub mod fsent;
pub mod signal;
