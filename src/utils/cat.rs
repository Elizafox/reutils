/* utils/cat.rs - implementation of cat
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs::File;
use std::io::{self, Write};

use getargs::{Opt, Options};

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

pub fn util(args: &[String]) -> Result {
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("Usage: {} [-u] [-h|--help] [FILE] ...", args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    let mut files: Vec<(&str, BufInput)> = Vec::new();

    for filename in opts.positionals() {
        if filename == "-" {
            files.push(("stdin", BufInput::Standard(io::stdin().lock())));
        } else {
            let file = File::open(filename);
            match file {
                Ok(file) => {
                    files.push((filename, BufInput::File(io::BufReader::new(file))));
                }
                Err(e) => {
                    return Err(Error::new(
                        1,
                        format!("Could not open file: {filename}: {e}"),
                    ));
                }
            }
        }
    }

    if files.is_empty() {
        // If ain't nobody got me, stdin got me.
        files.push(("stdin", BufInput::Standard(io::stdin().lock())));
    }

    for (filename, mut file) in files {
        if let Err(e) = io::copy(&mut file, &mut io::stdout()) {
            return Err(Error::new(
                1,
                format!("Could not write to {filename}: {e}"),
            ));
        }
    }

    if let Err(e) = io::stdout().flush() {
        return Err(Error::new(1, format!("Could not write to stdout: {e}")));
    }

    Ok(())
}
