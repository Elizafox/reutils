/* utils/wc.rs - implementation of wc
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

use getargs::{Opt, Options};

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

// 4 blocks on an AF disk at a time, or 32 blocks on a traditional disk
// Good enough for anyone.
const BUFFSIZE: usize = 16384usize;

#[derive(PartialEq, Eq)]
pub enum FlagsOffsetType {
    NoneType,
    Hex,
    Octal,
    Dec,
}

// Flags
pub struct Flags {
    pub offset_type: FlagsOffsetType,
    pub min_len: u64,
}

impl Flags {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offset_type: FlagsOffsetType::NoneType,
            min_len: 4u64,
        }
    }
}

fn read_file(reader: &mut BufInput, flags: &Flags) -> io::Result<()> {
    let mut buffer = [0u8; BUFFSIZE];
    let mut start = 0usize;
    let mut has_eof = false;
    while !has_eof || start > 0 {
        let len = if !has_eof && start < buffer.len() {
            // Ingest more data
            let rlen = reader.read(&mut buffer[start..])?;
            if rlen == 0 {
                has_eof = true;

                if start > 0 {
                    // We still have characters in the buffer.
                    // We should parse them.
                    start
                } else {
                    // EOF reached
                    continue;
                }
            } else {
                // Not EOF, combine starting position and length
                rlen + start
            }
        } else {
            // Process the string, do not ingest more data.
            start
        };

        let str_data;
        match std::str::from_utf8(&buffer[..len]) {
            Ok(data) => {
                // Valid string, good to go
                str_data = data.to_string();
                start = 0;
            }
            Err(e) => {
                // Hmm, seems we have a problem.
                let valid_up_to = e.valid_up_to();
                str_data =
                    unsafe { std::str::from_utf8_unchecked(&buffer[..valid_up_to]) }.to_string();

                if let Some(invalid_bytes) = e.error_len() {
                    // Skip the bad bits.
                    let skip = invalid_bytes + valid_up_to;
                    buffer.copy_within(skip..len, 0);
                    start = len - skip;
                } else {
                    // This is part of another char, just put it at the beginning.
                    buffer.copy_within(valid_up_to..len, 0);
                    start = len - valid_up_to;
                }
            }
        }

        let str_data = str_data
            .chars()
            .filter(|&c| !(c.is_control() || c.is_whitespace() && c != ' '))
            .collect::<String>();
        if str_data.len() as u64 > flags.min_len {
            println!("{str_data}");
        }
    }

    Ok(())
}

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-a] [-n] [-t type] [file...]");
}

pub fn util(args: &[String]) -> Result {
    let mut flags = Flags::new();

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('n') => {
                let Ok(arg) = opts.value() else { return Err(Error::new(1, "-n must have a total".to_string())) };
                match u64::from_str(arg) {
                    Ok(result) => {
                        if result == 0 {
                            return Err(Error::new(1, "-n: Total cannot be zero".to_string()));
                        }
                        flags.min_len = result;
                    }
                    Err(e) => {
                        return Err(Error::new(1, format!("-n: Invalid total: {e}")));
                    }
                }
            }
            Opt::Short('t') => {
                let Ok(arg) = opts.value() else { return Err(Error::new(1, "-t must have an argument".to_string())) };
                flags.offset_type = match arg {
                    "d" => FlagsOffsetType::Dec,
                    "o" => FlagsOffsetType::Octal,
                    "x" => FlagsOffsetType::Hex,
                    _ => {
                        return Err(Error::new(1, format!("-t: invalid format specifier {arg}")));
                    }
                };
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

    for (filename, ref mut file) in &mut files {
        if let Err(e) = read_file(file, &flags) {
            eprintln!("{}: Could not read {filename}: {e}", args[0]);
        }
    }

    Ok(())
}
