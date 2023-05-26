/* TODO: a more efficient implementation reading backwards from the file is in order.
 * But this is fine to get something out the door for now.
 */
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::str::FromStr;

use getargs::{Opt, Options};

use crate::err::{Error, Result};
use crate::bufinput::BufInput;

fn usage(args: &[String]) {
    eprintln!("Usage: {} [-n] lines [-h|--help] [FILE] ...", args[0]);
}

fn open((name, total): (&str, usize)) -> Result<(BufInput, usize)> {
    if name == "-" {
        return Ok((
            BufInput::Standard(io::stdin().lock()),
            total,
        ));
    }
    else {
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

fn output((file, total): (BufInput, usize)) -> Result
{
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

    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
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
