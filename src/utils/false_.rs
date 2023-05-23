/* utils/false_.rs - Implementation of false
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::err::AppletError;

pub fn util_false(_args: Vec<String>) -> Result<(), AppletError>
{
    Err(AppletError::new_nomsg(1))
}
