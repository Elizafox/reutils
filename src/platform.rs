/* platform.rs - platform-specific routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

/* The idea behind this module is that it smooths over major platform-specific differences and
 * commonly-used idioms and code. It presents just enough abstraction to make utilities work. Most
 * utilities won't need this, but some stuff is *heavily* platform-specific, and therefore they do.
 * Things like getting the process table, disk usage, etc. is not just Windows-specific, but
 * OS-specific. But there's other idioms too, like blocking ctrl-c, which are used a lot and would
 * be desirable to have in one place to avoid needless code duplication.
 *
 * Eventually, most platform-specific code should go here, to make it easier to port reutils to new
 * systems. But for many utilities, it's hardly worth the effort, especially when your only two
 * choices are Unix and Windows, and they use one-offs.
 * --Elizafox
 */

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

pub mod fsent;
pub mod signal;
