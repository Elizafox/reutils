/* bufoutput.rs - wrapper around stdout and files
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs;
use std::io;

#[allow(dead_code)]
pub enum BufOutput<'a> {
    File(fs::File),
    Buffer(io::BufWriter<fs::File>),
    Standard(io::StdoutLock<'a>),
}

impl io::Write for BufOutput<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            BufOutput::Standard(ref mut s) => s.write(buf),
            BufOutput::Buffer(ref mut b) => b.write(buf),
            BufOutput::File(ref mut f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            BufOutput::Standard(ref mut s) => s.flush(),
            BufOutput::Buffer(ref mut b) => b.flush(),
            BufOutput::File(ref mut f) => f.flush(),
        }
    }
}

impl BufOutput<'_> {
    #[allow(dead_code)]
    pub const fn is_file(&self) -> bool {
        match self {
            BufOutput::Standard(_) => false,
            BufOutput::Buffer(_) => true,
            BufOutput::File(_) => true,
        }
    }
}
