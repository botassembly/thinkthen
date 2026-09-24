REJECT

Reviewer: fresh read-only Claude (Opus) session, 2026-09-24. Checked: ticket at origin/ticket/0083-transform-catalog (b8f01f5c), origin/main at 989d5b0c (0088 landed), 0089 ticket branch, main's `cli/mod.rs` entry path, `cli/args/command.rs`, `sdlc/scripts/{lint,package}`, `crates/thinkthen/Cargo.toml`, and the ten `transforms/*/*.jq`.

The design is sound and still correct. All ten names, byte counts, and SHA-256 prefixes match current main: no transform changed since b02db985, and no eleventh `.jq` exists. The root order (status, ten functions, cache, transform, help) matches main's landed enum. `cli/mod.rs::entry` has a clean spot for early routing after the `--version` return and before `Environment::read` and `interrupt::activate`. `sdlc/scripts/package` already runs `cargo package --locked --offline --allow-dirty` from the lint rung. `Cargo.toml` has no `include` list or build script. The fixes below are small text changes.

## Findings

F1. The dependencies are stale. "Waits for tickets 0080, 0081, and 0082" and "Ticket 0082's post-0081 baseline" predate 0088 (public relate, now landed) and 0089 (queue item 2, which edits `cli/args.rs`, `find.rs`, `specification/result.md`, and the ratchet). Smallest change: implementation starts from main containing the 0088, 0089, and 0082 landing records. "No earlier command row changes" is measured against that tree.

F2. One acceptance clause cannot be proved. "Deliberately shuffled internal fixture input" has no meaning for a closed static table. Replace it with: a unit test asserts the table is strictly ascending bytewise and duplicate-free, and the list bytes are pinned. The locale, current-directory, and repeat checks stay.

F3. Name the 0082 test that must change. 0082 adds an exact root inventory assertion that ends `cache`, `help`. 0083 must edit that assertion (it cannot only add a new one), or both tests fail. State it in the Acceptance bullet on root help.

F4. The owner line is stale. "Luna Extra High ... Sol High ... three-ticket trial" predates the one-line plan. Name Claude as the owner and give the review route the plan uses.

F5 (minor). Budgets say "at most four existing files". The likely set is `cli/mod.rs` (routing), `cli/args/command.rs` (variant), `cli/args.rs` or a new args submodule (nested grammar), and `sdlc/scripts/policy.py` (not Rust). That fits. Say whether a new `cli/args/transform.rs` counts as the one added module or as a second new file.

## Acceptance quality

Strong. The exact list bytes, exact refusal sentence, exit 2, per-member SHA-256, counted loopback listener, trap `jq` on PATH, canary key, planted package faults, and the isolated unpacked build are all red-first and exact.

## Scope, split, order

No split is needed. 180 production and 500 total Rust lines are realistic for a static table plus nested grammar and routing. The unpacked-crate offline build adds a full compile to the lint rung. Run it on the gate host the repo uses for Rust builds. It must not run in parallel with 0082. Shared files: `cli/args/command.rs`, the root-help inventory test, `sdlc/ratchet.json`, `sdlc/scripts/lint`, and possibly `specification/README.md`. The ticket's serial order (0082 first) is correct.
