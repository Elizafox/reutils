/* utils/dirname.rs - implementation of dirname
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::path::Path;

use crate::err::{Error, Result};

fn usage(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} path");
    Error::new_nomsg(1)
}

fn dirname(path: &str) -> Result<String, Error> {
    Ok(match Path::new(&path).parent() {
        Some(base) => base.to_str()
            .ok_or_else(|| Error::new(1, "Could not convert path".to_string()))?
            .to_string(),
        None => path.to_string(),
    })
}

pub fn util(args: &[String]) -> Result {
    let path = args.get(1).ok_or_else(|| usage(&args[0]))?;

    println!("{}", dirname(path)?);

    Ok(())
}
