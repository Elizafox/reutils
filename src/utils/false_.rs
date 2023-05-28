/* utils/false_.rs - Implementation of false
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use crate::err::{Error, Result};

pub fn util_false(_args: Vec<String>) -> Result {
    Err(Error::new_nomsg(1))
}
