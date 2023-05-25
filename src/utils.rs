mod cat;
mod false_;
mod head;
mod reutils;
mod tail;
mod true_;

use crate::err::AppletResult;

use crate::utils::cat::util_cat;
use crate::utils::false_::util_false;
use crate::utils::head::util_head;
use crate::utils::reutils::util_reutils;
use crate::utils::tail::util_tail;
use crate::utils::true_::util_true;

use phf::{phf_ordered_map, OrderedMap};

pub type DispatchFn = fn(Vec<String>) -> AppletResult;

type MapValue = (&'static str, DispatchFn);

/** Utilities must be registered in this structure.
    Otherwise, reutils won't know about them!
*/
pub const DISPATCH_TABLE: OrderedMap<&'static str, MapValue> = phf_ordered_map! {
    "cat" => ("/bin/cat", util_cat),
    "false" => ("/bin/false", util_false),
    "head" => ("/usr/bin/head", util_head),
    "reutils" => ("/usr/sbin/reutils", util_reutils),
    "tail" => ("/usr/bin/tail", util_tail),
    "true" => ("/bin/true", util_true)
};
