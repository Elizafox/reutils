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
fn count_stats_str(string: &str, state: &mut CountStatsState) -> (usize, usize, usize) {
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;

    for c in string.chars() {
        chars += 1;

        if c.is_whitespace() {
            if c == '\n' {
                lines += 1;
            }
            if state.in_word {
                words += 1;
                state.in_word = false;
            }
        } else {
            state.in_word = true;
        }
    }

    (lines, words, chars)
}

// Get the counts stats for a bunch of bytes
fn count_stats_bytes(bytes: &[u8], state: &mut CountStatsState) -> (usize, usize, usize) {
    // ASCII whitespace chars
    const WHITESPACE: [u8; 6] = *b"\x09\x0A\x0B\x0C\x0D\x20";

    let mut lines = 0usize;
    let mut words = 0usize;
    let chars = bytes.len();

    for b in bytes {
        if WHITESPACE.contains(b) {
            if *b == b'\n' {
                lines += 1;
            }
            if state.in_word {
                words += 1;
                state.in_word = false;
            }
        } else {
            state.in_word = true;
        }
    }

    (lines, words, chars)
}

// Returns (lines, words, chars, encoding_errors) or Err.
// XXX - really should do better for encoding errors
fn read_file(reader: &mut BufInput, do_chars: bool) -> io::Result<(usize, usize, usize, bool)> {
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;

    let mut buffer = [0u8; BUFFSIZE];
    let mut state = CountStatsState::new();
    let mut start = 0usize;
    let mut has_eof = false;
    let mut has_encoding_errors = false;
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
                    if state.in_word {
                        words += 1;
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

        if do_chars {
            // Unicode
            let str_data;
            match std::str::from_utf8(&buffer[..len]) {
                Ok(data) => {
                    str_data = data.to_string();
                    start = 0;
                }
                Err(e) => {
                    let valid_up_to = e.valid_up_to();
                    str_data = unsafe { std::str::from_utf8_unchecked(&buffer[..valid_up_to]) }
                        .to_string();

                    if let Some(invalid_bytes) = e.error_len() {
                        // Skip the bad bits.
                        has_encoding_errors = true;
                        let skip = invalid_bytes + valid_up_to;
                        buffer.copy_within(skip..len, 0);
                        start = len - skip;
                    } else {
                        buffer.copy_within(valid_up_to..len, 0);
                        start = len - valid_up_to;
                    }
                }
            }

            let (c_lines, c_words, c_chars) = count_stats_str(&str_data, &mut state);
            lines += c_lines;
            words += c_words;
            chars += c_chars;
        } else {
            // Raw bytes
            let (c_lines, c_words, c_chars) = count_stats_bytes(&buffer[..len], &mut state);
            lines += c_lines;
            words += c_words;
            chars += c_chars;
        }
    }

    Ok((lines, words, chars, has_encoding_errors))
}

fn print_stats(
    do_lines: bool,
    do_words: bool,
    do_bytes_chars: bool,
    lines: usize,
    words: usize,
    chars: usize,
    filename: &str,
) {
    if do_lines {
        print!(" {lines}");
    }

    if do_words {
        print!(" {words}");
    }

    if do_bytes_chars {
        print!(" {chars}");
    }

    if filename.is_empty() {
        println!();
    } else {
        println!(" {filename}");
    }
}

// XXX - bool param for signalling encoding errors is bogus
// XXX - also need a proper flags structure
#[allow(clippy::fn_params_excessive_bools)]
fn handle_file(
    reader: &mut BufInput,
    do_lines: bool,
    do_words: bool,
    do_chars: bool,
    do_bytes: bool,
) -> io::Result<(usize, usize, usize, bool)> {
    if do_bytes && !do_lines && !do_words && reader.is_file() {
        // If we just have -c, and it's a normal reader, we can just stat the reader and go home.
        let BufInput::File(f) = reader else { unreachable!() };
        let metadata = f.get_ref().metadata()?;
        #[allow(clippy::cast_possible_truncation)]
        return Ok((0, 0, metadata.len() as usize, false));
    }

    let result = read_file(reader, do_chars)?;
    Ok(result)
}

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-c|-m] [-lw] [file...]");
}

pub fn util(args: &[String]) -> Result {
    // We have a separate default flag.
    let mut do_default = true;
    // XXX this should all go in a struct
    let mut do_bytes = false; // XXX this should be an enum
    let mut do_chars = false; // XXX
    let mut do_lines = false;
    let mut do_words = false;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('c') => {
                do_default = false;
                do_bytes = true;
                do_chars = false;
            }
            Opt::Short('m') => {
                do_default = false;
                do_bytes = false;
                do_chars = true;
            }
            Opt::Short('l') => {
                do_default = false;
                do_lines = true;
            }
            Opt::Short('w') => {
                do_default = false;
                do_words = true;
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
        do_bytes = true;
        do_lines = true;
        do_words = true;
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

    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;
    for (filename, ref mut file) in &mut files {
        let result = handle_file(file, do_lines, do_words, do_chars, do_bytes);
        match result {
            Ok((this_lines, this_words, this_chars, has_encoding_errors)) => {
                if has_encoding_errors {
                    eprintln!("{}: {filename}: Illegal byte sequence", args[0]);
                }

                print_stats(
                    do_lines,
                    do_words,
                    do_bytes || do_chars,
                    this_lines,
                    this_words,
                    this_chars,
                    filename,
                );

                lines += this_lines;
                words += this_words;
                chars += this_chars;
            }
            Err(e) => eprintln!("{}: Error reading file {filename}: {e}", args[0]),
        }
    }

    if file_count > 1usize {
        print_stats(
            do_lines,
            do_words,
            do_bytes || do_chars,
            lines,
            words,
            chars,
            "total",
        );
    }

    Ok(())
}
