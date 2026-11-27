# Agent rules

- Port upstream `symfony/deprecation-contracts` behaviour exactly; check it against PHP (in docker only).
- No PHP source in this repository. Each module doc has an `//! Upstream:` line.
- No `unwrap`/`expect` on input, no `unsafe`. `publish = false`; no crates.io.
- This repository depends on no other php2rust repository.
- Run the three checks in README.md before a merge. No CI workflows.
