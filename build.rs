/* build.rs: reutils build script.
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::env;
use std::error::Error;
use vergen::EmitBuilder;

fn split_authors(s: &str) -> String {
    let authors: Vec<_> = s.split(':').collect();
    match authors.len() {
        0 => String::from("Unknown author"),
        1 => String::from(authors[0]),
        2 => String::from(authors.join(" and ")),
        _ => {
            let (all_but_last, last) = authors.split_at(authors.len() - 1);
            String::from(format!("{}, and {}", all_but_last.join(", "), last[0]))
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .git_describe(true, true, None)
        .emit()?;

    println!("cargo:rustc-env=REUTILS_PKG_AUTHORS={}",
        split_authors(env!("CARGO_PKG_AUTHORS")));

    Ok(())
}
