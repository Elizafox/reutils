/* utils/tail.rs - implementation of tail
 * Copyright (C) 2023 Elizabeth Myers, Malina Thomas. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use getargs::{Opt, Options};
use notify::event::EventKind::Modify;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind};
use reverse_lines::ReverseLines;

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

fn usage(arg0: &str) {
    eprintln!("Usage: {arg0} [-n] lines [-h|--help] [FILE] ...");
}

fn follow(name: &str, total: usize) -> Result {
    let path = Path::new(name);
    let file = BufReader::new(
        File::open(path).map_err(|e| Error::new(1, format!("Could not open file {name}: {e}")))?,
    );

    let config = if RecommendedWatcher::kind() == WatcherKind::PollWatcher {
        Config::default()
            .with_poll_interval(Duration::from_millis(500))
            .with_compare_contents(true)
    } else {
        Config::default()
    };

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, config)
        .map_err(|e| Error::new(1, format!("Could not watch file {name}: {e}")))?;

    watcher
        .watch(path.as_ref(), RecursiveMode::NonRecursive)
        .map_err(|e| Error::new(1, format!("Could not watch file {name}: {e}")))?;

    // Print initial lines
    let mut line_iter = file.lines();
    let mut buff = VecDeque::with_capacity(total);
    line_iter
        .by_ref()
        .map(|l| add_line(&mut buff, l, total))
        .collect::<Result<Vec<_>, Error>>()
        .map(|_| ())?;

    for line in buff {
        println!("{line}");
    }

    for res in rx {
        match res {
            Ok(event) => {
                if let Modify(_) = event.kind {
                    let mut buff = VecDeque::new();
                    line_iter
                        .by_ref()
                        .map(|l| add_line(&mut buff, l, total))
                        .collect::<Result<Vec<_>, Error>>()
                        .map(|_| ())?;

                    for line in buff {
                        println!("{line}");
                    }
                }
            }
            Err(e) => return Err(Error::new(1, format!("Failed to watch file {name}: {e}"))),
        }
    }

    Ok(())
}

fn open((name, total): (&str, usize)) -> Result<(BufInput, usize)> {
    if name == "-" {
        return Ok((BufInput::Standard(io::stdin().lock()), total));
    }

    let f =
        File::open(name).map_err(|e| Error::new(1, format!("Failed to open file: {name}: {e}")))?;

    let f = BufInput::File(BufReader::new(f));

    Ok((f, total))
}

fn add_line(buff: &mut VecDeque<String>, line: Result<String, io::Error>, total: usize) -> Result {
    let line = line.map_err(|e| Error::new(1, format!("Failed to fetch line: {e}")))?;

    buff.push_back(line);
    if buff.len() > total {
        buff.pop_front();
    }

    Ok(())
}

fn output((file, total): (BufInput, usize)) -> Result {
    let mut buff = VecDeque::with_capacity(total);

    if let BufInput::File(file) = file {
        // Use the efficient way
        let reverse_lines = ReverseLines::new(file)
            .map_err(|e| Error::new(1, format!("Failed to get reverse iterator: {e}")))?;

        reverse_lines
            .take(total)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|l| add_line(&mut buff, l, total))
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())?;
    } else {
        file.lines()
            .map(|l| add_line(&mut buff, l, total))
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())?;
    }

    for line in buff {
        println!("{line}");
    }

    Ok(())
}

pub fn util(args: &[String]) -> Result {
    let mut total = 10usize; // POSIX default
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    let mut do_stream = false;

    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('f') => do_stream = true,
            Opt::Short('n') => {
                total = usize::from_str(
                    opts.value()
                        .map_err(|_| Error::new(1, "-n: no line count given".to_string()))?,
                )
                .map_err(|e| Error::new(1, format!("Invalid total: {e}")))?;
            }
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    if do_stream {
        // We only care about the first argument in this case
        if let Some(name) = opts.positionals().next() {
            // POSIX sez we ignore -f for stdin
            if name != "-" {
                return follow(name, total);
            }
        }
    }

    let files = opts
        .positionals()
        .zip(iter::repeat(total))
        .map(open)
        .collect::<Result<Vec<_>>>()?;

    if files.is_empty() {
        return output((BufInput::Standard(io::stdin().lock()), total));
    }

    files
        .into_iter()
        .map(output)
        .collect::<Result<Vec<_>>>()
        .map(|_| ())
}
