reutils
-------
This is reutils, a clone of busybox in Rust, specifically aimed at containers and small embedded systems.

Some utilities are implemented, but it's not a complete system; notably, there is no shell yet.

This software is provided under:

    SPDX-License-Identifier: GPL-2.0-only

See the LICENSE file for the actual license.

Portability
===========
We aim to support the following platforms:

Tier 1:
- Linux musl
- Linux glibc
- Windows

Tier 2:
- macOS
- BSD's
- Linux uClibc

Tier 3:
- Any other Unix workalike

Note that although we try to polyfill for Windows where appropriate, the mapping is not 1 to 1. Not every utility works, as Windows often does not have the needed functionality. In addition, some utilities need elevated privileges to run (notably, `ln`). Of course, you can always use WSL, and it should work.

Contributing
============
Contributions are always welcome. Please read the [Code of Conduct](/CODE_OF_CONDUCT.md) before contributing.

Please ensure that your code at least doesn't break a tier 1 platform, and preferably not lower tiers. If you don't know how to port something, ask one of us and we will try to assist in any way we can.

Note that the binary size is intended to be as small as possible; therefore, try to minimise the amount of bloat. Ensure all new dependencies are small and compact. Try to keep the `release` binary size below 2-4MB.

Make sure to run `cargo fmt` and `cargo clippy` before submitting pull requests!

TODO
====
See the [TODO](./TODO.md) file.
