/* utils/cat.rs - implementation of cat
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::fs::File;
use std::io::{copy, prelude::*, stdin, stdout, BufRead, BufReader};

use getargs::{Opt, Options};

use crate::err::{AppletError, AppletResult};

pub fn util_cat(args: Vec<String>) -> AppletResult {
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('u') => { /* No-op */ }
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("Usage: {} [-u] [-h|--help] [FILE] ...", args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    let mut files: Vec<(&str, Box<dyn BufRead>)> = Vec::new();

    for filename in opts.positionals() {
        let file = File::open(filename);
        match file {
            Ok(file) => {
                files.push((filename, Box::new(BufReader::new(file))));
            }
            Err(e) => {
                return Err(AppletError::new(
                    1,
                    format!("Could not open file: {}: {}", filename, e),
                ));
            }
        }
    }

    if files.is_empty() {
        // If ain't nobody got me, stdin got me.
        files.push(("stdin", Box::new(BufReader::new(stdin()))));
    }

    for (filename, mut file) in files {
        if let Err(e) = copy(&mut file, &mut stdout()) {
            return Err(AppletError::new(
                1,
                format!("Could not write to {}: {}", filename, e),
            ));
        }
    }

    if let Err(e) = stdout().flush() {
        return Err(AppletError::new(
            1,
            format!("Could not write to stdout: {}", e),
        ));
    }

    Ok(())
}
