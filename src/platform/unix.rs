/* platform/unix.rs - Unix platform routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

pub mod common;
pub mod fsent;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "freebsd")]
pub mod freebsd;

#[cfg(target_os = "macos")]
pub mod macos;
