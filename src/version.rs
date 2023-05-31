/* version.rs - version information for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_REVISION: &str = env!("VERGEN_GIT_DESCRIBE");
const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

const RUSTC_SEMVER: &str = env!("VERGEN_RUSTC_SEMVER");
const RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
const CARGO_HOST_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
const LLVM_VERSION: &str = env!("VERGEN_RUSTC_LLVM_VERSION");

const OS_NAME: &str = env!("VERGEN_SYSINFO_NAME");
const OS_VERSION: &str = env!("VERGEN_SYSINFO_OS_VERSION");

const AUTHORS: &str = env!("REUTILS_PKG_AUTHORS");

pub fn about(verbose: bool) {
    eprintln!("reutils v{VERSION} (git: {GIT_REVISION})");
    if verbose {
        eprintln!("Build timestamp: {BUILD_TIMESTAMP}");
        eprintln!("Build OS: {OS_VERSION} ({OS_NAME})");
        eprintln!("rustc version: {RUSTC_SEMVER} ({RUSTC_CHANNEL}), LLVM {LLVM_VERSION}");
        eprintln!("rustc host: {RUSTC_HOST_TRIPLE}");
        if CARGO_HOST_TRIPLE != RUSTC_HOST_TRIPLE {
            eprintln!("Cargo host: {CARGO_HOST_TRIPLE}");
        }
    }
    eprintln!("Copyright (C) 2023 {AUTHORS}");
    eprintln!("This program is free software; you may redistribute it under the terms of");
    eprintln!("the GNU General Public License version 2 ONLY.");
    eprintln!("This program has absolutely no warranty.");
}
