/* build.rs: reutils build script.
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

extern crate libc;

use std::env;
use std::error::Error;
use std::ffi::CStr;

use libc::{c_char, gethostname};

use vergen::EmitBuilder;

fn split_authors(s: &str) -> String {
    let authors: Vec<_> = s.split(':').collect();
    match authors.len() {
        0 => String::from("Unknown author"),
        1 => String::from(authors[0]),
        2 => authors.join(" and "),
        _ => {
            let (all_but_last, last) = authors.split_at(authors.len() - 1);
            format!("{}, and {}", all_but_last.join(", "), last[0])
        }
    }
}

fn gethostname_safe() -> String {
    let mut buf: [c_char; 256usize] = [c_char::from(0); 256usize];
    let res = unsafe { gethostname(buf.as_mut_ptr(), buf.len() - 1) };

    if res != 0 {
        // We don't care why, in this context.
        return "unknown".to_string();
    }

    let cstr = unsafe { CStr::from_ptr(buf.as_ptr()) };

    cstr.to_str().unwrap_or("unknown").to_string()
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

    println!(
        "cargo:rustc-env=REUTILS_PKG_AUTHORS={}",
        split_authors(env!("CARGO_PKG_AUTHORS"))
    );

    println!("cargo:rustc-env=REUTILS_BUILD_HOST={}", gethostname_safe());

    Ok(())
}
