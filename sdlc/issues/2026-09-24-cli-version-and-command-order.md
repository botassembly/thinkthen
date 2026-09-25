# The CLI lists the functions first, and 0.1.0 is the launch version

Status: Open. Ian's ruling, 2026-09-24, filed by the marketing session.

## Version

Ian, later the same day: keeping `0.0.1` until launch is fine. He does not care much about the pre-release number.

- `0.1.0` is the official release, cut once everything is done and tested.
- `thinkthen --version` prints the version. The libraries and database extensions carry the same version as the Rust crate.

## Command order in the help

Ian: the functions and help come first, and "at the bottom are the admin stuff, like status, audit, diff, and things like that."

Today `thinkthen --help` lists `status` first and `cache`, `transform`, and `help` last (`crates/thinkthen/src/cli/args/command.rs`). The asked order:

1. The ten functions. Marketing teaches them as decide, choose, score, tag, recognize, filter, rank, find, annotate, relate (`repos/mktg/products/thinkthen/vocabulary.md`). The team may keep another order, as long as the ten sit together at the top.
2. `help`.
3. The admin commands: `audit`, `diff`, `status`, `cache`, and `transform`.

## Marketing's use

Marketing no longer stages anything as done or not done. It names 0.1 plainly as the launch version. Every recording in the decks uses the newest build from main.
