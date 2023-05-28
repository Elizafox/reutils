/* build.rs: reutils build script.
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

fn main() {
    crate_git_revision::init();
}
