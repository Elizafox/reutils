/* err.rs - error routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub code: i32,
    pub message: Option<String>,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

impl Error {
    pub const fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message: Some(message),
        }
    }

    pub const fn new_nomsg(code: i32) -> Self {
        Self {
            code,
            message: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(e) => write!(f, "{e}"),
            None => write!(f, "Error code {}", self.code),
        }
    }
}
