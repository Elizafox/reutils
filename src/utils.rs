mod basename;
mod cal;
mod cat;
mod dirname;
mod echo;
mod false_;
mod head;
mod ln;
mod nice;
mod pwd;
mod reutils;
mod sleep;
mod tail;
mod tee;
mod true_;
mod tty;
mod uname;

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
    "dirname" => ("usr/bin/dirname", dirname::util),
    "echo" => ("bin/echo", echo::util),
    "false" => ("bin/false", false_::util),
    "head" => ("usr/bin/head", head::util),
    "link" => ("bin/link", ln::util_link),
    "ln" => ("bin/ln", ln::util_ln),
    "nice" => ("usr/bin/nice", nice::util),
    "pwd" => ("bin/pwd", pwd::util),
    "reutils" => ("usr/sbin/reutils", reutils::util),
    "sleep" => ("bin/sleep", sleep::util),
    "tail" => ("usr/bin/tail", tail::util),
    "tee" => ("usr/bin/tee", tee::util),
    "true" => ("bin/true", true_::util),
    "tty" => ("usr/bin/tty", tty::util),
    "uname" => ("usr/bin/uname", uname::util),
};

pub fn paths() -> Vec<(&'static str, &'static str)> {
    let mut utils: Vec<(&'static str, &'static str)> = Vec::new();
    for (util_name, (util_path, _)) in &DISPATCH_TABLE {
        utils.push((util_name, util_path));
    }
    utils
}
