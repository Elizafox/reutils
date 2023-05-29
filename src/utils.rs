mod basename;
mod cat;
mod dirname;
mod echo;
mod false_;
mod head;
mod reutils;
mod sleep;
mod tail;
mod tee;
mod true_;

use crate::err::Result;

use phf::{phf_ordered_map, OrderedMap};

pub type DispatchFn = fn(&[String]) -> Result;

type MapValue = (&'static str, DispatchFn);

/** Utilities must be registered in this structure.
    Otherwise, reutils won't know about them!
*/
pub const DISPATCH_TABLE: OrderedMap<&'static str, MapValue> = phf_ordered_map! {
    "basename" => ("/usr/bin/basename", basename::util),
    "cat" => ("/bin/cat", cat::util),
    "dirname" => ("/usr/bin/dirname", dirname::util),
    "echo" => ("/bin/echo", echo::util),
    "false" => ("/bin/false", false_::util),
    "head" => ("/usr/bin/head", head::util),
    "reutils" => ("/usr/sbin/reutils", reutils::util),
    "sleep" => ("/bin/sleep", sleep::util),
    "tail" => ("/usr/bin/tail", tail::util),
    "tee" => ("/usr/bin/tee", tee::util),
    "true" => ("/bin/true", true_::util)
};
