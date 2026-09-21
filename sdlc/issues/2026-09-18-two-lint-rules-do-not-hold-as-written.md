# Issue 0001: Two lint rules in rust-standards.md do not hold as written

- Status: Closed on 2026-09-21. `sdlc/planning/rust-standards.md` now says the lint is denied across the workspace and forbidden in the core, and it records the `unwrap_used` gap.
- Found: 2026-09-18, building slice 0
- Affects: `sdlc/planning/rust-standards.md`, "Lint table"

Two rules in the standards failed when slice 0 tried to enforce them. Both are recorded here so the document can be corrected. Ian can overturn either call.

## `allow_attributes_without_reason` cannot be forbidden workspace-wide

The standards say the lint is forbidden. Clap's `Parser` derive expands to `#[allow(clippy::restriction)]`, and `allow_attributes_without_reason` belongs to that group. A `forbid` level turns every such expansion into `error[E0453]: allow(clippy::restriction) incompatible with previous forbid`, so the binary crate does not compile at all.

The workspace table now denies the lint instead. `thinkthen-core` takes no derive from clap, so its crate root forbids the lint and the pure crate keeps the stronger level. The gate still bites: an `#[allow(dead_code)]` with no reason fails `lint` in either crate.

## `unwrap_used` misses an unwrap whose error type is `Infallible`

`version.parse::<String>().unwrap()` passed a full `lint` run. The same call spelled `version.parse::<u8>().unwrap()` failed as expected. Clippy does not report `unwrap_used` when the `Result` cannot carry an error, which is sound but worth knowing: the lint is not a blanket ban on the token.

## What to do

Change the standards to say `allow_attributes_without_reason` is denied at the workspace and forbidden in the pure crate. Leave `unwrap_used` as it is and drop any claim that the word `unwrap` never appears.
