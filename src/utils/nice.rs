/* utils/nice.rs - implementation of nice
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

#[cfg(unix)]
fn set_priority(niceness: i32) {
    use errno::{errno, set_errno, Errno};
    use libc::{c_int, getpriority, setpriority, PRIO_PROCESS};
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
    let ret = unsafe { setpriority(PRIO_PROCESS, 0, new_priority as c_int) };
    if ret < 0 {
        eprintln!(
            "Could not set process priority: {}",
            io::Error::last_os_error()
        );
    }
}

#[cfg(unix)]
fn spawn_process(niceness: i32, command: &str, args: &[String]) -> Result {
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
    match status.code() {
        Some(code) => {
            if code == 0 {
                Ok(())
            } else {
                Err(Error::new_nomsg(code))
            }
        }
        None => Ok(()),
    }
}

#[cfg(windows)]
fn spawn_process(niceness: i32, command: &str, args: &[String]) -> Result {
    use std::os::windows::process::CommandExt;
    use std::process::Command;
    use windows::Win32::System::Threading::{
        REALTIME_PRIORITY_CLASS,
        HIGH_PRIORITY_CLASS,
        ABOVE_NORMAL_PRIORITY_CLASS,
        NORMAL_PRIORITY_CLASS,
        BELOW_NORMAL_PRIORITY_CLASS,
        IDLE_PRIORITY_CLASS};

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
        } else if niceness >= -19 && niceness <= -10 {
            HIGH_PRIORITY_CLASS.0
        } else if niceness <= -1 && niceness >= -9 {
            ABOVE_NORMAL_PRIORITY_CLASS.0
        } else if niceness == 0 {
            NORMAL_PRIORITY_CLASS.0
        } else if niceness >= 1 && niceness <= 18 {
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
    match status.code() {
        Some(code) => {
            if code == 0 {
                Ok(())
            } else {
                Err(Error::new_nomsg(code))
            }
        }
        None => Ok(()),
    }
}

pub fn util(args: &[String]) -> Result {
    spawn_process(19, &args[1], &args[2..])
}
