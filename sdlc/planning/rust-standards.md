# Rust standards for thinkthen

Written 2026-09-18 from a survey of the ten Rust repositories in Ian's workspace and from his Rust ideal state. Ian writes no Rust and cannot review it. Gates judge the code. Every rule below names the tool that enforces it. A rule with no tool is listed under "Not yet enforced" and counts as a gap.

## What the survey found

- Every repository runs the same ladder: `sdlc/scripts/install`, `lint`, `test`, `spec`. No repository has a `check` that wraps the three.
- One repository is strict. It denies `unwrap`, `expect`, `panic`, indexing, and printing. It bans whole families of methods in library code. It pins an exact toolchain, caps every file at 500 non-blank lines, and keeps a size ceiling equal to the measured total. Its own lint script verifies that the lint table was not weakened.
- The others share a weaker copied lint block. Several call the size ratchet and never declared a ceiling, so it passes quietly. Denying `unwrap` alone moved the panics into `expect`.
- No repository uses property tests, snapshot tests, or a mock HTTP crate. The Rust ideal state asks for property tests on parsers.

`thinkthen` starts from the strict repository's posture on the first commit. Adopting strictness later costs more than starting with it.

## Layout

- One Cargo workspace, resolver 3, edition 2024, an exact `rust-version`, and one package named `thinkthen`. The package has a library and a binary. The binary requires the default `cli` feature. Enforced by: `policy.py` checks the workspace, targets, and features.
- Three private modules follow one dependency direction.
  - `core` holds the question types, wire format, plan, acceptance policy, framing parsers, and question-file grammar. It touches no file, environment variable, socket, clock, or process. Its module attributes forbid the accepted lint groups. `policy.py` scans every core source, refuses references to `engine`, `cli`, and outer-only dependencies, and plants one failure of each kind in its self-check.
  - `engine` owns prepared request identity, transport, retries, recording, cache locks, both scheduling state machines, bounded work queues, stop metadata, and scoped request workers. It accepts typed framed-input events and returns typed ordered results and structured private errors. Its worker scope joins every request worker before returning.
  - `cli` owns arguments, credential and environment lookup, input framing, CSV and TSV, the detached input reader, output, diagnostics, `Failure`, and exit codes. It maps private engine errors to the existing command contract.
- `rust-toolchain.toml` pins one exact release with the `minimal` profile and exactly `clippy` and `rustfmt`. Enforced by: `lint` parses the file.
- `rustfmt.toml` holds `style_edition = "2024"` and nothing else.

## Lint table

Rust lints: `unsafe_code`, `unreachable_pub`, and `missing_debug_implementations` are forbidden. `missing_docs` warns, and warnings fail the build.

Clippy lints: `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout`, and `print_stderr` are denied. `allow_attributes_without_reason` is denied across the workspace and forbidden in the core. Clap's derive macro expands to an `allow` attribute, and `forbid` would stop the binary from compiling. `cognitive_complexity`, `excessive_nesting`, `too_many_lines`, and `wildcard_imports` warn.

Clippy settings: functions up to 90 lines, 6 arguments, cognitive complexity 20, nesting depth 4. Tests may use `unwrap`, `expect`, `panic`, and indexing through the settings file. No test file pastes a suppression at its top.

The binary writes to standard output through one locked writer passed down as a value. It never calls the print macros.

Enforced by: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and `lint` compares the tables in `Cargo.toml` and `clippy.toml` against the accepted copies so nobody weakens them quietly. The package allows the disallowed-path lint groups at its root and the private core forbids them, because Clippy reads one configuration for the whole package. `policy.py` also checks every core source and dependency root.

## Size

- No source or test file passes 500 non-blank lines. Enforced by: `lint`.
- `sdlc/ratchet.json` holds one ceiling over everything under `crates`, test files included. The ceiling equals the measured total. Adding code means editing the number. The commit that raises it says what grew and why it earns the lines, and where the author looked for duplication to delete first. Enforced by: `sdlc/scripts/ratchet.mjs`, called by `lint`.
- A repository with nothing to lint fails. Enforced by: `lint` fails when it finds no Rust source.

## Errors and types

- The core module defines its own error enums with `thiserror`. No public signature returns a string error or a boxed unknown error. The private engine error carries one of six accepted kinds with its structured cause. The command adds context at the top and maps each error to one exit code in one place.
- A fixed set of choices is an enum. A validated value gets its own type: a probability, a threshold, a JSON Pointer, an option name, a model name. A constructor that can fail returns a result.
- No dynamic JSON inside the core. Wire bodies decode into typed structs. Enforced by: the core module attributes keep the lint group forbidden, and `policy.py` holds the accepted purity table. Ticket 0001 planted each dynamic JSON kind and watched `lint` refuse it.

## Dependencies

- Few, and each one argued. `serde`, `serde_json`, `sha2`, `thiserror`, and the engine's `ureq` form the library-only graph. The default `cli` feature adds optional `clap` and `csv-core`. Parallel requests use threads and bounded channels. No async runtime enters until a measurement asks for one. The package rung builds and inspects the default-features-off graph.
- Every dependency resolves from crates.io with a checksum, under MIT, Apache-2.0, Unicode-3.0, or Unlicense. HTTPS forced three more: ISC, BSD-3-Clause, and CDLA-Permissive-2.0, each tied in `policy.py` to the crates that need it. No TLS stack exists in Rust without them. Enforced by: `lint` reads `cargo metadata` and `Cargo.lock`, and `cargo deny` reads the same seven licenses from `deny.toml` and fails on an advisory or on a license nothing in the tree offers.
- Adding a dependency takes a second reviewing agent and a line in the commit message saying why the standard library would not do.

## Tests

- Red first. A test fails for the stated reason before the code exists.
- Unit tests sit beside pure code in the core and are table-driven.
- Integration tests under `crates/thinkthen/tests/` run the compiled binary and see only arguments, standard input, standard output, standard error, and the exit code.
- No gate touches the network. Tests replay recorded responses from a fixture directory. One test helper serves canned responses from a loopback listener to prove the request bytes. It uses the standard library only.
- Property tests cover every parser and round trip: JSON Pointer, the question set, record framing, the wire format. Property tests also cover any total function over a numeric range, such as the threshold rule. `proptest` is a development dependency of the package and exercises core parsers.
- `spec/*.md` files are executable examples of the command line, run by `mustmatch`. They are the top rung, and they double as the user documentation.
- The binary reads one hidden, test-only variable, `THINKTHEN_TEST_RETRY_WAIT_MS`, so a retry test never sleeps for real seconds. Help never shows it.
- Live calls to a paid backend sit outside the ladder in `sdlc/scripts/live`. They run by hand, under a token cap, with Ian's authorization.
- A claim of secrecy is tested over every command, on the path that succeeds and on each path that fails, and it reads every `Debug` line. One command does not stand for the rest.
- A test that claims nothing was sent counts the requests on the loopback listener. An exit code and a `--dry-run` prove nothing about a live path.
- A script that checks something runs from a rung. `probes/replay-check.sh` rotted for a day because no rung ran it, and `spec` runs it now.

## The ladder

| Rung | Script | What it runs |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | Verifies the pinned toolchain and the tools the other rungs need |
| 1 | `sdlc/scripts/lint` | Policy and package checks, the ratchet, the file ceiling, `cargo deny`, `cargo fmt --check`, clippy with warnings denied, `cargo doc` with warnings denied |
| 2 | `sdlc/scripts/test` | `cargo test --locked --workspace --all-targets --all-features` |
| 3 | `sdlc/scripts/spec` | The compiled binary against `spec/*.md` |

Cheapest rung first. The whole ladder runs before any hand-back.

## When the repository goes public

`LICENSE` is here. Ian ruled MIT in ADR 0015, each package declares `license = "MIT"`, and `policy.py` checks both. `.github/workflows/gate.yml` runs the four rungs on every push and every pull request, and ADR 0015 item 6 rules it. Each action in it is pinned to a commit SHA and the downloaded `jq` is checked against a recorded SHA-256. `deny.toml` allows exactly the licenses the tree carries, and `lint` runs `cargo deny check advisories bans licenses` over it. `CHANGELOG.md` still waits.

## Not yet enforced

- `unwrap_used` skips a result whose error type cannot occur. The lint bans a risk and leaves the token legal.
- Clippy ignores a ban-list path it cannot resolve, and a typo in any ban passes quietly. The dynamic JSON bans were proved in ticket 0001. Every ban added later has to be planted and refused the same way.
- No tool checks that parsing happens only at the edge or that a validated value has its own type. The reviewing agent checks both and says so in its review.
- No tool requires a second reviewer before the ceiling rises, the public surface widens, or a dependency lands. The rule is written in `AGENTS.md` only.
- The five factory scripts under `sdlc/project/` are absent. Factory 2 runs one pilot repository and this is not it. Copy them when this repository registers.
