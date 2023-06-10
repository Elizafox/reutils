/* utils/wc.rs - implementation of wc
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs::File;
use std::io;
use std::io::prelude::*;

use getargs::{Opt, Options};

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

// 4 blocks on an AF disk at a time, or 32 blocks on a traditional disk
// Good enough for anyone.
const BUFFSIZE: usize = 16384usize;

// -m, -c, or neither
#[derive(PartialEq, Eq)]
pub enum FlagsUnitType {
    NoneType,
    Char,
    Byte,
}

// Flags
pub struct Flags {
    pub chars_bytes: FlagsUnitType,
    pub words: bool,
    pub lines: bool,
}

impl Flags {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chars_bytes: FlagsUnitType::NoneType,
            words: false,
            lines: false,
        }
    }
}

pub struct Stats {
    pub chars: usize,
    pub words: usize,
    pub lines: usize,
    pub encoding_error: Option<io::Error>,
}

impl Stats {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            chars: 0usize,
            words: 0usize,
            lines: 0usize,
            encoding_error: None,
        }
    }
}

// Used to keep track of counting state
pub struct CountStatsState {
    pub in_word: bool,
}

impl CountStatsState {
    #[must_use]
    pub const fn new() -> Self {
        Self { in_word: false }
    }
}

// Get the counts stats for a string, returning what a human "expects".
fn count_stats_str(string: &str, tracking_state: &mut CountStatsState) -> Stats {
    let mut stats = Stats::new();

    for c in string.chars() {
        stats.chars += 1;

        if c.is_whitespace() {
            if c == '\n' {
                stats.lines += 1;
            }
            if tracking_state.in_word {
                stats.words += 1;
                tracking_state.in_word = false;
            }
        } else {
            tracking_state.in_word = true;
        }
    }

    stats
}

// Get the counts stats for a bunch of bytes
fn count_stats_bytes(bytes: &[u8], tracking_state: &mut CountStatsState) -> Stats {
    let mut stats = Stats::new();
    stats.chars = bytes.len();

    for b in bytes {
        // Whitespace
        if (*b >= b'\x09' && *b <= b'\x0D') || *b == b'\x20' {
            if *b == b'\n' {
                stats.lines += 1;
            }
            if tracking_state.in_word {
                stats.words += 1;
                tracking_state.in_word = false;
            }
        } else {
            tracking_state.in_word = true;
        }
    }

    stats
}

fn read_file(reader: &mut BufInput, flags: &Flags) -> io::Result<Stats> {
    let mut stats = Stats::new();

    let mut buffer = [0u8; BUFFSIZE];
    let mut tracking_state = CountStatsState::new();
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
                    // If we were still in a word, then count the last one.
                    if tracking_state.in_word {
                        stats.words += 1;
                    }

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

        let this_stats = if flags.chars_bytes == FlagsUnitType::Char {
            // Unicode
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
                    str_data = unsafe { std::str::from_utf8_unchecked(&buffer[..valid_up_to]) }
                        .to_string();

                    if let Some(invalid_bytes) = e.error_len() {
                        // Skip the bad bits.
                        stats.encoding_error = Some(io::Error::from(io::ErrorKind::InvalidData));
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

            count_stats_str(&str_data, &mut tracking_state)
        } else {
            count_stats_bytes(&buffer[..len], &mut tracking_state)
        };

        stats.lines += this_stats.lines;
        stats.words += this_stats.words;
        stats.chars += this_stats.chars;
    }

    Ok(stats)
}

fn print_stats(flags: &Flags, stats: &Stats, filename: &str) {
    if flags.lines {
        print!(" {}", stats.lines);
    }

    if flags.words {
        print!(" {}", stats.words);
    }

    if flags.chars_bytes != FlagsUnitType::NoneType {
        print!(" {}", stats.chars);
    }

    if filename.is_empty() {
        println!();
    } else {
        println!(" {filename}");
    }
}

// XXX - bool param for signalling encoding errors is bogus
fn handle_file(reader: &mut BufInput, flags: &Flags) -> io::Result<Stats> {
    let mut stats: Stats;
    #[allow(clippy::cast_possible_truncation)]
    if flags.chars_bytes == FlagsUnitType::Byte && !flags.lines && !flags.words && reader.is_file()
    {
        // If we just have -c, and it's a normal reader, we can just stat the reader and go home.
        let BufInput::File(f) = reader else { unreachable!() };
        let metadata = f.get_ref().metadata()?;
        stats = Stats::new();
        stats.chars = metadata.len() as usize;
        return Ok(stats);
    }

    stats = read_file(reader, flags)?;
    Ok(stats)
}

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-c|-m] [-lw] [file...]");
}

pub fn util(args: &[String]) -> Result {
    let mut do_default = true;
    let mut flags = Flags::new();

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('c') => {
                do_default = false;
                flags.chars_bytes = FlagsUnitType::Byte;
            }
            Opt::Short('m') => {
                do_default = false;
                flags.chars_bytes = FlagsUnitType::Char;
            }
            Opt::Short('l') => {
                do_default = false;
                flags.lines = true;
            }
            Opt::Short('w') => {
                do_default = false;
                flags.words = true;
            }
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    if do_default {
        // Default options
        flags.chars_bytes = FlagsUnitType::Byte;
        flags.lines = true;
        flags.words = true;
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

    let file_count = files.len();

    let mut stats = Stats::new();
    for (filename, ref mut file) in &mut files {
        let result = handle_file(file, &flags);
        match result {
            Ok(stats_result) => {
                if let Some(ref encoding_error) = stats.encoding_error {
                    eprintln!("{}: {filename}: {encoding_error}", args[0]);
                }

                print_stats(&flags, &stats_result, filename);

                stats.lines += stats_result.lines;
                stats.words += stats_result.words;
                stats.chars += stats_result.chars;
            }
            Err(e) => eprintln!("{}: Error reading file {filename}: {e}", args[0]),
        }
    }

    if file_count > 1usize {
        print_stats(&flags, &stats, "total");
    }

    Ok(())
}
