mod cat;
mod false_;
mod reutils;
mod true_;

use crate::err::AppletError;

use crate::utils::cat::util_cat;
use crate::utils::false_::util_false;
use crate::utils::reutils::util_reutils;
use crate::utils::true_::util_true;

use phf::{OrderedMap, phf_ordered_map};

pub type DispatchFn = fn(Vec<String>) -> Result<(), AppletError>;

type MapValue = (&'static str, DispatchFn);

/* Utilities must be registered in this structure.
 * Otherwise, reutils won't know about them!
 */
pub const DISPATCH_TABLE : OrderedMap<&'static str, MapValue> = phf_ordered_map!
{
    "cat" => ("/bin/cat", util_cat),
    "false" => ("/bin/false", util_false),
    "reutils" => ("/usr/bin/reutils", util_reutils),
    "true" => ("/bin/true", util_true)
};
