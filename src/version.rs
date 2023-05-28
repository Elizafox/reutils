const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_REVISION: &str = env!("VERGEN_GIT_SHA");
const BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

const RUSTC_SEMVER: &str = env!("VERGEN_RUSTC_SEMVER");
const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
const CARGO_HOST_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
const LLVM_VERSION: &str = env!("VERGEN_RUSTC_LLVM_VERSION");
const OS_VERSION: &str = env!("VERGEN_SYSINFO_OS_VERSION");

const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn about(verbose: bool) {
    eprintln!("reutils v{VERSION}-{GIT_REVISION}");
    if verbose {
        eprintln!("Build timestamp: {BUILD_TIMESTAMP}");
        eprintln!("Built on {OS_VERSION} with rustc v{RUSTC_SEMVER} ({RUSTC_HOST_TRIPLE}), LLVM {LLVM_VERSION}");
        if CARGO_HOST_TRIPLE != RUSTC_HOST_TRIPLE {
            eprintln!("(Cross-compiled from {CARGO_HOST_TRIPLE})");
        }
    }
    eprintln!("Copyright (C) 2023 {}", AUTHORS.replace(':', ", "));
    eprintln!("This program is free software; you may redistribute it under the terms of");
    eprintln!("the GNU General Public License version 2 ONLY.");
    eprintln!("TThis program has absolutely no warranty.");
}
