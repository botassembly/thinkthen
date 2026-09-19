# Rust standards for thinkn

Written 2026-09-18 from a survey of the ten Rust repositories in Ian's workspace and from his Rust ideal state. Ian writes no Rust and cannot review it. Gates judge the code. Every rule below names the tool that enforces it. A rule with no tool is listed under "Not yet enforced" and counts as a gap.

## What the survey found

- Every repository runs the same ladder: `sdlc/scripts/install`, `lint`, `test`, `spec`. No repository has a `check` that wraps the three.
- One repository is strict. It denies `unwrap`, `expect`, `panic`, indexing, and printing. It bans whole families of methods in library code. It pins an exact toolchain, caps every file at 500 non-blank lines, and keeps a size ceiling equal to the measured total. Its own lint script verifies that the lint table was not weakened.
- The others share a weaker copied lint block. Several call the size ratchet and never declared a ceiling, so it passes quietly. Denying `unwrap` alone moved the panics into `expect`.
- No repository uses property tests, snapshot tests, or a mock HTTP crate. The Rust ideal state asks for property tests on parsers.

`thinkn` starts from the strict repository's posture on the first commit. Adopting strictness later costs more than starting with it.

## Layout

- One Cargo workspace, resolver 3, edition 2024, an exact `rust-version`. The lint table sits at the workspace root and every crate inherits it. Enforced by: `lint` verifies that each crate sets `lints.workspace = true`.
- Two crates, split on a real dependency direction.
  - `crates/thinkn-core` is a pure library. It holds the question types, the wire format, the plan, the acceptance policy, the framing parsers, and the question-file grammar. It touches no file, no environment variable, no socket, no clock, and no process. Enforced by: a `clippy.toml` in that crate bans those methods and types, and the crate root forbids the two lints.
  - `crates/thinkn` is the binary. It owns arguments, files, the environment, HTTP, time, and exit codes. It parses at the edge and hands typed values inward.
- `rust-toolchain.toml` pins one exact release with the `minimal` profile and exactly `clippy` and `rustfmt`. Enforced by: `lint` parses the file.
- `rustfmt.toml` holds `style_edition = "2024"` and nothing else.

## Lint table

Rust lints: `unsafe_code`, `unreachable_pub`, and `missing_debug_implementations` are forbidden. `missing_docs` warns, and warnings fail the build.

Clippy lints: `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout`, and `print_stderr` are denied. `allow_attributes_without_reason` is forbidden. `cognitive_complexity`, `excessive_nesting`, `too_many_lines`, and `wildcard_imports` warn.

Clippy settings: functions up to 90 lines, 6 arguments, cognitive complexity 20, nesting depth 4. Tests may use `unwrap`, `expect`, `panic`, and indexing through the settings file. No test file pastes a suppression at its top.

The binary writes to standard output through one locked writer passed down as a value. It never calls the print macros.

Enforced by: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and `lint` compares the tables in `Cargo.toml` and `clippy.toml` against the accepted copies so nobody weakens them quietly.

## Size

- No source or test file passes 500 non-blank lines. Enforced by: `lint`.
- `sdlc/ratchet.json` holds one ceiling over all Rust source. The ceiling equals the measured total. Adding code means editing the number. The commit that raises it says what grew and why it earns the lines, and where the author looked for duplication to delete first. Enforced by: `sdlc/scripts/ratchet.mjs`, called by `lint`.
- A repository with nothing to lint fails. Enforced by: `lint` fails when it finds no Rust source.

## Errors and types

- The core crate defines its own error enums with `thiserror`. No public signature returns a string error or a boxed unknown error. The binary adds context at the top and maps each error to one exit code in one place.
- A fixed set of choices is an enum. A validated value gets its own type: a probability, a pass mark, a JSON Pointer, an option name, a model name. A constructor that can fail returns a result.
- No dynamic JSON inside the core. Wire bodies decode into typed structs. Enforced by: the core's `clippy.toml` bans `serde_json::Value` access methods and the `json!` macro.

## Dependencies

- Few, and each one argued. The starting set: `serde` and `serde_json` for the wire format, `thiserror` for errors, `clap` with derive for the command line, `ureq` for blocking HTTP. Parallel requests use threads and a bounded channel. No async runtime enters until a measurement asks for one.
- Every dependency resolves from crates.io with a checksum, under MIT, Apache-2.0, Unicode-3.0, or Unlicense. Enforced by: `lint` reads `cargo metadata` and `Cargo.lock`.
- Adding a dependency takes a second reviewing agent and a line in the commit message saying why the standard library would not do.

## Tests

- Red first. A test fails for the stated reason before the code exists.
- Unit tests sit beside pure code in the core and are table-driven.
- Integration tests under `crates/thinkn/tests/` run the compiled binary and see only arguments, standard input, standard output, standard error, and the exit code.
- No gate touches the network. Tests replay recorded responses from a fixture directory. One test helper serves canned responses from a loopback listener to prove the request bytes. It uses the standard library only.
- Property tests cover every parser and round trip: JSON Pointer, the question file, record framing, the wire format. `proptest` arrives as a development dependency with the first parser.
- `spec/*.md` files are executable examples of the command line, run by `mustmatch`. They are the top rung, and they double as the user documentation.
- Live calls to a paid backend sit outside the ladder in `sdlc/scripts/live`. They run by hand, under a token cap, with Ian's authorization.

## The ladder

| Rung | Script | What it runs |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | Verifies the pinned toolchain and the tools the other rungs need |
| 1 | `sdlc/scripts/lint` | Policy checks, the ratchet, the file ceiling, `cargo fmt --check`, clippy with warnings denied, `cargo doc` with warnings denied |
| 2 | `sdlc/scripts/test` | `cargo test --locked --workspace --all-targets --all-features` |
| 3 | `sdlc/scripts/spec` | The compiled binary against `spec/*.md` |

Cheapest rung first. The whole ladder runs before any hand-back.

## When the repository goes public

Add `LICENSE`, `CHANGELOG.md`, `deny.toml` with `cargo deny` in `lint`, and one CI workflow that runs the three rungs. No private repository in the workspace carries these, and neither does this one until then.

## Not yet enforced

- No tool checks that parsing happens only at the edge or that a validated value has its own type. The reviewing agent checks both and says so in its review.
- No tool requires a second reviewer before the ceiling rises, the public surface widens, or a dependency lands. The rule is written in `AGENTS.md` only.
- The five factory scripts under `sdlc/project/` are absent. Factory 2 runs one pilot repository and this is not it. Copy them when this repository registers.
