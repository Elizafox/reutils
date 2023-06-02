/* platform.rs - platform-specific routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

// Windows specific routines go below here

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub mod unix;

pub mod signal {
    #[cfg(windows)]
    pub use crate::platform::windows::signal::block_ctrlc;

    #[cfg(unix)]
    pub use crate::platform::unix::signal::block_ctrlc;
}
