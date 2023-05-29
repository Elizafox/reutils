mod cat;
mod dirname;
mod false_;
mod head;
mod reutils;
mod sleep;
mod tail;
mod true_;

use crate::err::Result;

use crate::utils::cat::util_cat;
use crate::utils::dirname::util_dirname;
use crate::utils::false_::util_false;
use crate::utils::head::util_head;
use crate::utils::reutils::util_reutils;
use crate::utils::sleep::util_sleep;
use crate::utils::tail::util_tail;
use crate::utils::true_::util_true;

use phf::{phf_ordered_map, OrderedMap};

pub type DispatchFn = fn(Vec<String>) -> Result;

type MapValue = (&'static str, DispatchFn);

/** Utilities must be registered in this structure.
    Otherwise, reutils won't know about them!
*/
pub const DISPATCH_TABLE: OrderedMap<&'static str, MapValue> = phf_ordered_map! {
    "cat" => ("/bin/cat", util_cat),
    "dirname" => ("/usr/bin/dirname", util_dirname),
    "false" => ("/bin/false", util_false),
    "head" => ("/usr/bin/head", util_head),
    "reutils" => ("/usr/sbin/reutils", util_reutils),
    "sleep" => ("/bin/sleep", util_sleep),
    "tail" => ("/usr/bin/tail", util_tail),
    "true" => ("/bin/true", util_true)
};
