/* utils/nice.rs - implementation of nice
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use getargs::{Opt, Options};

use crate::err::{Error, Result};

#[cfg(unix)]
fn set_priority(niceness: i32) {
    use errno::{errno, set_errno, Errno};
    use libc::{getpriority, setpriority, PRIO_PROCESS};
    use std::io;

    set_errno(Errno(0));
    let mut current_priority = unsafe { getpriority(PRIO_PROCESS, 0) };
    if current_priority == -1 && errno().0 != 0 {
        eprintln!(
            "Could not get process priority: {}",
            io::Error::last_os_error()
        );
        current_priority = 0;
    }

    let new_priority = niceness + current_priority;
    if unsafe { setpriority(PRIO_PROCESS, 0, new_priority) } < 0 {
        eprintln!(
            "Could not set process priority: {}",
            io::Error::last_os_error()
        );
    }
}

#[cfg(unix)]
fn spawn_process(niceness: i32, command: &str, args: &[&str]) -> Result {
    use std::os::unix::process::CommandExt;
    use std::process::Command;

    let mut cmd = unsafe {
        Command::new(command)
            .args(args)
            .pre_exec(move || {
                set_priority(niceness);
                Ok(())
            })
            .spawn()
            .map_err(|e| Error::new(1, format!("Could not spawn command: {e}")))?
    };

    let status = cmd
        .wait()
        .map_err(|e| Error::new(1, format!("Command not running: {e}")))?;
    status.code().map_or(Ok(()), |code| {
        if code == 0 {
            Ok(())
        } else {
            Err(Error::new_nomsg(code))
        }
    })
}

#[cfg(windows)]
fn spawn_process(niceness: i32, command: &str, args: &[&str]) -> Result {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use windows::Win32::System::Threading::{
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS, HIGH_PRIORITY_CLASS,
        IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS, REALTIME_PRIORITY_CLASS,
    };

    /* This is somewhat arbitrary
     * We try to map windows priorities to the historical Unix range of -20 to 19:
     *   - -20 and below is REALTIME_PRIORITY_CLASS
     *   - -19 through -10 is HIGH_PRIORITY_CLASS
     *   - -9 through -1 is ABOVE_NORMAL_PRIORITY_CLASS
     *   - 0 is NORMAL_PRIORITY_CLASS
     *   - 1 through 18 is BELOW_NORMAL_PRIORITY_CLASS
     *   - 19 and above is IDLE_PRIORITY_CLASS
     */
    let priority = {
        if niceness <= -20 {
            REALTIME_PRIORITY_CLASS.0
        } else if (-19..=-10).contains(&niceness) {
            HIGH_PRIORITY_CLASS.0
        } else if (-9..=-1).contains(&niceness) {
            ABOVE_NORMAL_PRIORITY_CLASS.0
        } else if niceness == 0 {
            NORMAL_PRIORITY_CLASS.0
        } else if (1..=18).contains(&niceness) {
            BELOW_NORMAL_PRIORITY_CLASS.0
        } else if niceness >= 19 {
            IDLE_PRIORITY_CLASS.0
        } else {
            // Shouldn't get here
            NORMAL_PRIORITY_CLASS.0
        }
    };

    let mut cmd = Command::new(command)
        .args(args)
        .creation_flags(priority)
        .spawn()
        .map_err(|e| Error::new(1, format!("Could not spawn command: {e}")))?;

    let status = cmd
        .wait()
        .map_err(|e| Error::new(1, format!("Command not running: {e}")))?;
    status.code().map_or(Ok(()), |code| {
        if code == 0 {
            Ok(())
        } else {
            Err(Error::new_nomsg(code))
        }
    })
}

fn usage(arg0: &str) -> Error {
    Error::new(1, format!("Usage: {arg0} [-n number] command..."))
}

pub fn util(args: &[String]) -> Result {
    let mut niceness = 10i32;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("{}", usage(&args[0]));
                return Ok(());
            }
            Opt::Short('n') => {
                let niceness_str = opts.value().map_err(|_| {
                    eprintln!("Error: No niceness value specified");
                    usage(&args[0])
                })?;

                niceness = niceness_str.parse::<i32>().map_err(|_| {
                    eprintln!("Bad niceness value");
                    usage(&args[0])
                })?;
            }
            _ => {}
        }
    }

    let args = opts.positionals().collect::<Vec<_>>();

    if args.is_empty() {
        eprintln!("No command specified");
        return Err(usage(args[0]));
    }

    spawn_process(niceness, args[0], &args[1..])
}
