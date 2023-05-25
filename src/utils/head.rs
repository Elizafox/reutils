/* utils/head.rs - implementation of head
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;

use getargs::{Opt, Options};

use crate::err::{AppletError, AppletResult};

fn usage(args: &[String]) {
    eprintln!("Usage: {} [-n] lines [-h|--help] [FILE] ...", args[0]);
}

pub fn util_head(args: Vec<String>) -> AppletResult {
    let mut total = 10u64; // POSIX default

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('n') => match u64::from_str(opts.value().unwrap()) {
                Ok(result) => total = result,
                Err(e) => {
                    return Err(AppletError::new(1, format!("Invalid total: {}", e)));
                }
            },
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args);
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

    for (filename, file) in files {
        let mut count = 0u64;

        for line in file.lines() {
            count += 1;
            if count > total {
                break;
            }

            match line {
                Ok(line) => {
                    println!("{}", line);
                }
                Err(e) => {
                    return Err(AppletError::new(
                        1,
                        format!("Error reading from {}: {}", filename, e),
                    ));
                }
            }
        }
    }

    Ok(())
}
