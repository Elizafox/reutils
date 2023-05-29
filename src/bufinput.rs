/* bufinput.rs - wrapper around stdin and files
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs;
use std::io;

pub enum BufInput<'a> {
    File(io::BufReader<fs::File>),
    Standard(io::StdinLock<'a>),
}

impl io::Read for BufInput<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            BufInput::Standard(ref mut s) => s.read(buf),
            BufInput::File(ref mut f) => f.read(buf),
        }
    }
}

impl io::BufRead for BufInput<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        match self {
            BufInput::Standard(ref mut s) => s.fill_buf(),
            BufInput::File(ref mut f) => f.fill_buf(),
        }
    }

    fn consume(&mut self, amt: usize) {
        match self {
            BufInput::Standard(ref mut s) => s.consume(amt),
            BufInput::File(ref mut f) => f.consume(amt),
        }
    }
}
