/* platform/fsent.rs - filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#[cfg(unix)]
pub use crate::platform::unix::fsent::*;

#[cfg(windows)]
pub use crate::platform::windows::fsent::*;

#[derive(Debug)]
pub struct FilesystemEntry {
    pub filesystem_name: String,
    pub mount_point: String,
    pub mount_from: String,
    pub mount_options: String,
}

#[derive(Debug)]
pub struct FilesystemStats {
    pub block_size: u64,
    pub blocks_total: u64,
    pub blocks_free: u64,
    pub blocks_available: u64,
}
