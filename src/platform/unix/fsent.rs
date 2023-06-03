/* platform/unix/fsent.rs - Unix filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#[cfg(target_os = "linux")]
pub use crate::platform::unix::linux::fsent::*;

#[cfg(target_os = "freebsd")]
pub use crate::platform::unix::freebsd::fsent::*;

#[cfg(target_os = "macos")]
pub use crate::platform::unix::macos::fsent::*;
