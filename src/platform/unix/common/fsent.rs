/* platform/unix/common/fsent.rs - Generic Unix filesystem entry routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

// Unlike the other routines, this one is fairly simple: We just iterate through the parents until
// we find that its parent has a different mountpoint; then we've found it.
#[allow(clippy::missing_errors_doc)]
pub fn get_path_mountpoint(path: &str) -> io::Result<String> {
    let mut child = Path::new(path).canonicalize()?;
    let mut child_metadata = child.metadata()?;

    // Preserve original child since we overwrite it
    let orig_child = child.clone();
    for parent in orig_child.ancestors().skip(1) {
        let parent_metadata = parent.metadata()?;

        if parent_metadata.dev() != child_metadata.dev() {
            // Parent is on a different device; therefore we are the mountpoint.
            break;
        }

        child = parent.to_path_buf();
        child_metadata = parent_metadata;
    }

    Ok(child.to_string_lossy().to_string())
}
