reutils
-------
This is reutils, a clone of busybox in Rust.

Some utilities are implemented, but it's not a complete system; notably, there is no shell yet.

This software is provided under:

    SPDX-License-Identifier: GPL-2.0-only

See the LICENSE file for the actual license.

Portability
===========
We aim to support the following platforms:

Tier 1:
- Linux
- Windows

Tier 2:
- macOS
- BSD's

Tier 3:
- Any other Unix workalike

Note that although we try to polyfill for Windows where appropriate, the mapping is not 1 to 1. Not every utility works, as Windows often does not have the needed functionality. In addition, some utilities need elevated privileges to run (notably, `ln`). Of course, you can always use WSL, and it should work.

Contributing
============
Contributions are always welcome. Please read the [Code of Conduct](/CODE_OF_CONDUCT.md) before contributing.

TODO
====
- [ ] Implement whatever utilities busybox does (including a shell, etc)
- [ ] Proper version support (build script?)
- [ ] We could probably handle errors better, in a more Rust-like way
- [ ] `--install` flag
- [ ] Way to list the path of all utilities
