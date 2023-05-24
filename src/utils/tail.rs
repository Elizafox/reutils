use std::collections::VecDeque;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read, Seek, Stdin};
use std::str::FromStr;

use getargs::{Opt, Options};
use rev_lines::RevLines;

use crate::err::{AppletError, AppletResult};

fn usage(args: &[String]) {
    eprintln!("Usage: {} [-n] lines [-h|--help] [FILE] ...", args[0]);
}

trait Tailable {
    fn tail(self, total: u64) -> AppletResult;
}

impl Tailable for (String, BufReader<File>) {
    fn tail(self, total: u64) -> AppletResult {
        if self
            .1
            .get_ref()
            .metadata()
            .map_err(|e| {
                AppletError::new(
                    1,
                    format!("Failed to get metadata for file: {}: {}", self.0, e),
                )
            })?
            .is_file()
        {
            tail_rev(self.1, total)
        } else {
            tail_seq(self.1, total)
        }
    }
}

impl Tailable for BufReader<Stdin> {
    fn tail(self, total: u64) -> AppletResult {
        tail_seq(self, total)
    }
}

// This implementation is used for stdin and FIFOs
fn tail_seq<R>(file: BufReader<R>, total: u64) -> AppletResult
where
    R: Read,
{
    let mut buf = VecDeque::<String>::new();

    for line in file.lines() {
        buf.push_back(line.unwrap());
        if (buf.len() as u64) > total {
            buf.pop_front();
        }
    }

    for line in buf {
        println!("{}", line);
    }

    Ok(())
}

// This implementation is used for regular files
fn tail_rev<R>(file: BufReader<R>, total: u64) -> AppletResult
where
    R: Read + Seek,
{
    let mut buf = VecDeque::<String>::new();
    let rev_lines = RevLines::new(file).unwrap();

    for line in rev_lines {
        buf.push_front(line);
        if (buf.len() as u64) == total {
            break;
        }
    }

    for line in buf {
        println!("{}", line);
    }

    Ok(())
}

fn build_tuple(fname: String) -> Result<(String, BufReader<File>), AppletError> {
    let file = File::open(&fname)
        .map_err(|e| AppletError::new(1, format!("Could not open file: {}: {}", fname, e)))?;

    Ok((fname, BufReader::new(file)))
}

pub fn util_tail(args: Vec<String>) -> AppletResult {
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

    let files = opts
        .positionals()
        .map(str::to_string)
        .map(build_tuple)
        .collect::<Result<Vec<_>, AppletError>>()?;

    if files.is_empty() {
        BufReader::new(stdin()).tail(total)
    } else {
        files
            .into_iter()
            .map(|t| t.tail(total))
            .collect::<Result<Vec<_>, AppletError>>()
            .map(|_| ())
    }
}
