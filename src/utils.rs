/* utils.rs - utils table for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

mod basename;
mod cal;
mod cat;
#[cfg(unix)] // Not working on Windows
mod df;
mod dirname;
mod echo;
mod false_;
mod head;
mod ln_link;
mod nice;
mod pwd;
#[cfg(unix)] // Not working on Windows
mod renice;
mod reutils;
mod sleep;
mod strings;
mod tail;
mod tee;
mod true_;
mod tty;
mod uname;
mod wc;

use crate::err::Result;

use phf::{phf_ordered_map, OrderedMap};

pub type DispatchFn = fn(&[String]) -> Result;

type MapValue = (&'static str, DispatchFn);

/** Utilities must be registered in this structure.
    Otherwise, reutils won't know about them!
*/
pub const DISPATCH_TABLE: OrderedMap<&'static str, MapValue> = phf_ordered_map! {
    "basename" => ("usr/bin/basename", basename::util),
    "cal" => ("usr/bin/cal", cal::util),
    "cat" => ("bin/cat", cat::util),
    #[cfg(unix)] // Broken on Windows
    "df" => ("bin/df", df::util),
    "dirname" => ("usr/bin/dirname", dirname::util),
    "echo" => ("bin/echo", echo::util),
    "false" => ("bin/false", false_::util),
    "head" => ("usr/bin/head", head::util),
    "link" => ("bin/link", ln_link::util_link),
    "ln" => ("bin/ln", ln_link::util_ln),
    "nice" => ("usr/bin/nice", nice::util),
    "pwd" => ("bin/pwd", pwd::util),
    #[cfg(unix)] // Not working on Windows
    "renice" => ("usr/bin/renice", renice::util),
    "reutils" => ("usr/sbin/reutils", reutils::util),
    "sleep" => ("bin/sleep", sleep::util),
    "strings" => ("usr/bin/strings", strings::util),
    "tail" => ("usr/bin/tail", tail::util),
    "tee" => ("usr/bin/tee", tee::util),
    "true" => ("bin/true", true_::util),
    "tty" => ("usr/bin/tty", tty::util),
    "uname" => ("usr/bin/uname", uname::util),
    "wc" => ("usr/bin/wc", wc::util),
};

pub fn paths() -> Vec<(&'static str, &'static str)> {
    let mut utils: Vec<(&'static str, &'static str)> = Vec::new();
    for (util_name, (util_path, _)) in &DISPATCH_TABLE {
        utils.push((util_name, util_path));
    }
    utils
}
