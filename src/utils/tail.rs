/* utils/tail.rs - implementation of tail
 * Copyright (C) 2023 Elizabeth Myers, Malina Thomas. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

/* TODO: a more efficient implementation reading backwards from the file is in order.
 * But this is fine to get something out the door for now.
 */
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use getargs::{Opt, Options};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, WatcherKind};
use notify::event::{EventKind::Modify, ModifyKind::Any};

use crate::bufinput::BufInput;
use crate::err::{Error, Result};

fn usage(args: &[String]) {
    eprintln!("Usage: {} [-n] lines [-h|--help] [FILE] ...", args[0]);
}

fn follow(name: &str, total: usize) -> Result {
    let path = Path::new(name);
    let file = BufReader::new(
        File::open(&path).map_err(|e| Error::new(1, format!("Could not open file {name}: {e}")))?,
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

    buff.into_iter().for_each(|l| println!("{l}"));

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Modify(Any) {
                    let mut buff = VecDeque::new();
                    line_iter
                        .by_ref()
                        .map(|l| add_line(&mut buff, l, total))
                        .collect::<Result<Vec<_>, Error>>()
                        .map(|_| ())?;

                    buff.into_iter().for_each(|l| println!("{l}"));
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
    } else {
        let f = File::open(name)
            .map_err(|e| Error::new(1, format!("Failed to open file: {name}: {e}")))?;

        let f = BufInput::File(BufReader::new(f));

        return Ok((f, total));
    }
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

    file.lines()
        .map(|l| add_line(&mut buff, l, total))
        .collect::<Result<Vec<_>, Error>>()
        .map(|_| ())?;

    buff.into_iter().for_each(|l| println!("{l}"));

    Ok(())
}

pub fn util_tail(args: Vec<String>) -> Result {
    let mut total = 10usize; // POSIX default
    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    let mut do_stream = false;

    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('f') => do_stream = true,
            Opt::Short('n') => match usize::from_str(opts.value().unwrap()) {
                Ok(result) => total = result,
                Err(e) => {
                    return Err(Error::new(1, format!("Invalid total: {}", e)));
                }
            },
            Opt::Short('h') | Opt::Long("help") => {
                usage(&args);
                return Ok(());
            }
            _ => {}
        }
    }

    if do_stream {
        // We only care about the first argument in this case
        if let Some(name) = opts.positionals().nth(0) {
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
        output((BufInput::Standard(io::stdin().lock()), total))
    } else {
        files
            .into_iter()
            .map(output)
            .collect::<Result<Vec<_>>>()
            .map(|_| ())
    }
}
