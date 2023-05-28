/* build.rs: reutils build script.
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .git_sha(true)
        .emit()?;
    Ok(())
}
