/* utils/head.rs - implementation of head
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use getargs::{Opt, Options};

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-n] lines [-h|--help] [FILE] ...");
}

pub fn util(args: &[String]) -> Result {
    let mut total = 10u64; // POSIX default

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('n') => {
                let Ok(arg) = opts.value() else { return Err(Error::new(1, "-n must have a total".to_string())) };

                match u64::from_str(arg) {
                    Ok(result) => total = result,
                    Err(e) => {
                        return Err(Error::new(1, format!("Invalid total: {e}")));
                    }
                }
            }
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args[0]);
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

    for (filename, file) in files {
        let mut count = 0u64;

        let lines = file.lines();
        for line in lines {
            count += 1;
            if count > total {
                break;
            }

            match line {
                Ok(line) => {
                    println!("{line}");
                }
                Err(e) => {
                    return Err(Error::new(1, format!("Error reading from {filename}: {e}")));
                }
            }
        }
    }

    Ok(())
}
