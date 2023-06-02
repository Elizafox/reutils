/* utils/renice.rs - implementation of renice
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

#[allow(unused_imports)]
use getargs::{Opt, Options};

use crate::err::{Error, Result};

#[cfg(unix)]
fn usage(arg0: &str) -> Error {
    Error::new(1, format!("Usage: {arg0} [-g|-p|-u] [-n number] ID..."))
}

/* I don't intend to implement -g or -u for Windows, so it uses a different implementation
 * entirely. --Elizafox
 */
#[cfg(windows)]
pub const fn util(_args: &[String]) -> Result {
    Err(Error::new(255, "Not implemented on Windows yet"))
}

/* This tries to mirror POSIX behaviour, although the return code mechanism works like BSD. POSIX
 * doesn't specify the return code AFAICT, so this is okay, and it's somewhat compatible.
 * --Elizafox
 */
#[cfg(unix)]
pub fn util(args: &[String]) -> Result {
    use std::ffi::{c_int, CString};
    use std::io;

    use errno::{errno, set_errno, Errno};
    use libc::{getpriority, getpwnam, id_t, setpriority, PRIO_PGRP, PRIO_PROCESS, PRIO_USER};

    let mut niceness: c_int = 10;
    let mut which = PRIO_PROCESS;
    let mut errs = 0i32;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("{}", usage(&args[0]));
                return Ok(());
            }
            Opt::Short('g') => which = PRIO_PGRP,
            Opt::Short('u') => which = PRIO_USER,
            Opt::Short('p') => which = PRIO_PROCESS,
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
        eprintln!("No ID's specified");
        return Err(usage(args[0]));
    }

    for str_id in args {
        let mut int_id: Option<id_t> = None;

        if which == PRIO_USER {
            let pwstr = CString::new(str_id).expect("Could not convert ID to C string");
            let pwd = unsafe { getpwnam(pwstr.as_ptr()) };
            if !pwd.is_null() {
                int_id = Some(unsafe { (*pwd).pw_uid });
            }
        }

        if int_id.is_none() {
            if let Ok(i) = str_id.parse::<id_t>() {
                int_id = Some(i);
            } else {
                eprintln!("{str_id}: invalid ID");
                errs += 1;
                continue;
            }
        }

        // Should be safe to unwrap at this point
        let int_id = int_id.unwrap();

        set_errno(Errno(0));
        let current_priority = unsafe { getpriority(which, int_id) };
        if current_priority == -1 && errno().0 != 0 {
            eprintln!(
                "Could not get priority for {int_id}: {}",
                io::Error::last_os_error()
            );
            errs += 1;
            continue;
        }

        let new_priority = niceness + current_priority;
        if unsafe { setpriority(which, int_id, new_priority) } < 0 {
            eprintln!(
                "Could not set priority for {int_id}: {}",
                io::Error::last_os_error()
            );
            errs += 1;
        }
    }

    if errs > 0 {
        Err(Error::new_nomsg(errs))
    } else {
        Ok(())
    }
}
