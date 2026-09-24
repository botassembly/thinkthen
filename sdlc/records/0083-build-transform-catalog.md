# 0083: Build the read-only transform catalog

Status: landed. A fresh Claude code review (`sdlc/records/0083-code-review.md`) returned two findings. Both are fixed: `specification/roadmap.md` now points to `transform.md`, and `spec/transform.md` checks empty output with `test -z` and the byte count exactly; each fixed line was shown to fail on wrong output (`11735`, `x`). The review's note on catalog help vocabulary is also taken: `sdlc/scripts/demos` scans `transform`, `transform list`, and `transform show` help, and a planted `row` in `demos-self-test` fails it. The coordinator accepted the fixes without another review. Owner: Claude.

## Result

`thinkthen transform list` prints the ten names in bytewise order. `thinkthen transform show NAME` prints one embedded `.jq` file byte for byte. An unknown name exits 2 with the fixed sentence and no echo. `cli/mod.rs::entry` routes `transform` right after the `--version` return, before `Environment::read`, interrupt activation, usage counters, standard input, and the engine. The catalog module sees only the parsed subcommand and the locked standard-output writer.

The branch was rebased onto main at `897401fd` (0082 landed). The ticket's review route now says a fresh Claude session reviews it and Codex does not.

## Red then green

- `tests/transform.rs`: all five tests failed first with `error: unrecognized subcommand 'transform'` (list printed nothing, show exited 2, help lacked the row). The guarded test failed on the missing packaged file.
- `tests/version.rs` root inventory assertion (edited, not duplicated) failed with `root Commands order is [..., "cache", "help"]`.
- The guarded test was proved able to fail twice by temporary edits that were then reverted. Routing after `Environment::read` failed with `thinkthen: the configuration file could not be read`. An early `std::fs::read("band")` hung on the named-pipe decoy and failed with `["transform", "list"] blocked on input or a decoy`.
- `catalog.py` was proved able to fail on the tree: one appended byte in `crates/thinkthen/transforms/cost.jq` and one extra `extra.jq` gave `cost.jq differs from the repository copy` and `extra is not a listed member`. Its table regex first missed the row rustfmt wraps, and the check failed on the table names until the regex allowed whitespace.

## Acceptance

| Criterion | Proof |
| --- | --- |
| Exact list bytes, final newline, empty stderr, exit 0, repeated, locales, directories; table strictly ascending and duplicate-free | `transform::list_prints_the_closed_names_in_byte_order_everywhere` (two directories, four locales, two runs each). Unit test `cli::transform::tests::the_catalog_is_strictly_ascending_bytewise_and_free_of_duplicates` |
| Edit 0082's single root inventory to insert `transform`; pin the three descriptions in short and long help | `version.rs` `ORDER` now has 14 rows ending `cache`, `transform`, `help`. `transform::transform_help_pins_its_three_introductions` pins the root row after the unchanged `cache` row and the `-h` and `--help` openings of `transform`, `transform list`, and `transform show` |
| All ten `show` results against byte fixtures, byte counts, and SHA-256 | `transform::show_prints_each_member_byte_for_byte` compares each with the ticket's count and digest, the packaged copy, and `transforms/NAME/NAME.jq`, and checks one final newline, no CR, and no BOM |
| Exact-name lookup and refusal | `transform::an_unknown_name_is_refused_without_echoing_it` covers case variants, `.jq`, `./`, traversal, absolute, prefix, suffix, empty, leading space, trailing newline, and two Unicode names: empty stdout, exit 2, the exact sentence. Missing and surplus arguments and `--input` get Clap's exit 2 with empty stdout. Unit test `lookup_takes_only_an_exact_name` |
| No key or configuration read; no file, cache, counter, lock, or folder created | `transform::the_catalog_reads_no_setting_input_or_file_and_sends_and_runs_nothing` sets every variable the source reads (`HOME`, `XDG_CONFIG_HOME`, `XDG_CACHE_HOME`, `THINKTHEN_CACHE`, `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and the six `THINKTHEN_SIGINT_*` and `THINKTHEN_TEST_*` names) to canaries under a mode-000 folder. It snapshots every path and size under the test root before and after |
| Zero network requests; no canary in any output | The same test binds a nonblocking loopback listener named by `THINKTHEN_BASE_URL` and asserts `accept` returns `WouldBlock` after eleven successful runs and one refusal. The canary key and the canary name appear in no stdout or stderr. The catalog writes no file, so no temporary file can hold it. The command has no `Debug` output path; `RUST_BACKTRACE=full` was set |
| Zero process execution | The same test puts only trap `jq`, `sh`, `bash`, `python3`, `cargo`, `env`, and `cat` on `PATH`; the marker stays absent. `policy.py` `check_catalog_policy` refuses process, `Command`, `exec`, `nix`, engine, edge, cache, interrupt, usage, and recording references with 25 plants and keeps 7 controls |
| No standard-input or user-file access | The same test holds standard input open and runs from a folder of named pipes called `NAME` and `NAME.jq` for all ten. Opening one or reading input would block, and the test fails after 20 seconds. The grammar has no input option or path. Policy forbids `fs`, `stdin`, `File`, `include_str`, and `env` in the module |
| `cargo package`, inspect the `.crate`, unpack outside the checkout, build offline, remove the source, run from an empty folder | `sdlc/scripts/package` runs `catalog.py --crate target/package/thinkthen-0.0.1.crate`, unpacks it under `mktemp -d`, builds with `--locked --offline` and a temporary target folder, copies the binary, deletes the source and target, then runs `list` and every `show` from an empty folder. It compares the list, all ten files, empty stderr, and an unchanged empty folder |
| Planted package faults | `catalog.py` self-test (lint rung) builds in-memory archives with one changed byte, one deleted member, and one unlisted `extra.jq`. Each yields exactly its own failure line, and the clean archive yields none |
| Settled specification page, executable page, help, and transform index | `specification/transform.md` (Settled) and its index row; `spec/transform.md` pins the list, `show counts` against the repository file and 1,735 bytes, and the refusal with exit 2; `transforms/README.md` names the catalog and says the tool runs no transform |
| Focused tests, policy self-test, package rung, ratchet, format, Clippy, `git diff --check`, four gates, no live call | See Gates below. `sdlc/scripts/live` never ran |

## Budget

| Bound | Limit | Measured |
| --- | ---: | ---: |
| Existing production Rust files touched | 4 | 2 (`cli/mod.rs`, `cli/args/command.rs`) |
| New production Rust files | 1 | 1 (`cli/transform.rs`) |
| Nonblank production Rust lines added | 180 | 74 (61 in `transform.rs` outside its test module, 13 in the two edited files) |
| Nonblank Rust lines added, tests included | 500 | 444 added, 6 removed |
| Packaged `.jq` files, bytes, lines | 10, 66,328, 1,418 | 10, 66,328, 1,418 |
| Largest new Rust file | 500 nonblank | `tests/transform.rs` 349 |
| Dependencies, build scripts | 0 | 0 |

The ratchet moves from 43,897 to 44,335, the measured total (+438), in two commits. The first adds the catalog (+433). The second adds the file-level Clippy allow the test helpers need (+5), after lint's first run refused their `expect` and `panic` calls. Non-Rust files changed: `sdlc/scripts/policy.py`, `package`, `lint`, `README.md`, and the new `catalog.py`.

Duplication checked first: the nested `cache` grammar (followed its shape), `edge::write_line` (adds a newline, so raw bytes cannot use it), `Failure::Usage` (reused for the refusal), the `help()` and `command()` helpers in `version.rs` and `status.rs` (integration test files share no module), and the policy scanner (reused `rust_tokens`, `rust_use_paths`, and the alias and glob checks). Nothing was found to delete.

## Departures

- Clap strips a single-sentence doc comment's final period, as it does for every other root row. The root row and the three help openings therefore read without the ticket's trailing period. The tests pin the text as printed.
- The ticket lists `sdlc/scripts/policy.py`, `package`, and `lint` as the changed scripts. The build also adds `sdlc/scripts/catalog.py` and a row in `sdlc/scripts/README.md`, both inside `opens`. A separate script keeps one byte comparison shared by the tree check, the archive check, and the planted faults.
- The table membership check lives in `catalog.py`, which parses the `include_bytes!` rows. The Rust tests pin the same ten names independently.

## Gates

Run on Beelink at `8cb3faa9`, one rung at a time, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset:

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint`: exit 0. It runs policy with the catalog plants, `catalog.py` with its planted faults, the package rung with the archive check and the unpacked offline build, the ratchet at 44,335/44,335, fmt, Clippy, and docs. The first lint run at `f82ada36` failed on Clippy in `tests/transform.rs`, and `8cb3faa9` fixes it.
- `sdlc/scripts/test`: exit 0, 736 Rust tests passed and 0 failed across all test binaries.
- `sdlc/scripts/spec`: exit 0, including `spec/transform.md` (3 blocks).
- `git diff --check 897401fd..HEAD`: clean.

`sdlc/scripts/live` did not run. No paid or network call ran. The review is still pending. A fresh read-only Claude session reviews the final diff under the ticket's route.
