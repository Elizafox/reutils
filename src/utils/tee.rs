/* utils/tee.rs - implementation of tee
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

use getargs::{Opt, Options};

use crate::bufoutput::BufOutput;
use crate::err::{Error, Result};

const BUFFSIZE: usize = 16384usize;

fn block_sigint() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
}

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-ai] [files]...");
}

#[allow(clippy::unnecessary_wraps)]
pub fn util(args: &[String]) -> Result {
    let mut do_append = false;
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('a') => do_append = true,
            Opt::Short('i') => block_sigint(),
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    let mut files: Vec<(&str, BufOutput)> = vec![("stdout", BufOutput::Standard(stdout().lock()))];

    for filename in opts.positionals() {
        let file = BufOutput::File(if do_append {
            File::options()
                .read(false)
                .append(true)
                .open(filename)
                .map_err(|e| Error::new(1, format!("Could not open file {filename}: {e}")))?
        } else {
            File::create(filename)
                .map_err(|e| Error::new(1, format!("Could not open file {filename}: {e}")))?
        });

        files.push((filename, file));
    }

    let mut stdin_handle = stdin().lock();

    loop {
        let mut buff = [0u8; BUFFSIZE];
        let len = stdin_handle
            .read(&mut buff)
            .map_err(|e| Error::new(1, format!("Could not read from stdin: {e}")))?;

        if len == 0 {
            break;
        }

        for (filename, file) in &mut files {
            file.write_all(&buff[..len])
                .map_err(|e| Error::new(1, format!("Could not write to file {filename}: {e}")))?;
        }
    }

    Ok(())
}
