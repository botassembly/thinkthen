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
  - `core` holds the question types, wire format, plan, acceptance policy, framing parsers, and question-file grammar. It touches no file, environment variable, socket, clock, or process. Its module attributes forbid the accepted lint groups. `policy.py` scans every core source, refuses references to `engine`, `cli`, `public`, `windows`, and outer-only dependencies, including target dependencies, and plants one failure of each kind in its self-check.
  - `engine` owns prepared request identity, transport, retries, recording, cache locks, both scheduling state machines, bounded work queues, stop metadata, and scoped request workers. It accepts typed framed-input events and returns typed ordered results and structured private errors. Its worker scope joins every request worker before returning.
  - `cli` owns arguments, credential and environment lookup, input framing, CSV and TSV, the detached input reader, output, diagnostics, `Failure`, and exit codes. It maps private engine errors to the existing command contract.
- `rust-toolchain.toml` pins one exact release with the `minimal` profile and exactly `clippy` and `rustfmt`. Enforced by: `lint` parses the file.
- `rustfmt.toml` holds `style_edition = "2024"` and nothing else.

## Lint table

Rust lints: the workspace forbids `unsafe_code`, `unreachable_pub`, and `missing_debug_implementations`. Ticket 0380 C gives the ThinkThen package the identical root lint tables with only `unsafe_code` changed to `deny`. The library forbids unsafe outside Windows and denies it on Windows. The binary and core forbid unsafe on every target. Only `src/windows/security/ffi.rs`, `src/windows/files/ffi.rs`, `src/windows/console/ffi.rs`, and the Windows test injector `tests/windows/ffi.rs` may use reasoned unsafe allowances. Each leaf denies `unsafe_op_in_unsafe_fn` and carries its own target guard. The injector requires both Windows and test builds. `policy.py` uses its comment and literal aware Rust scanner to check exact paths, allowances, target guards and lint tables. Its negative plants exercise each boundary. `missing_docs` warns, and warnings fail the build.

Clippy lints: `unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout`, and `print_stderr` are denied. `allow_attributes_without_reason` is denied across the workspace and forbidden in the core. Clap's derive macro expands to an `allow` attribute, and `forbid` would stop the binary from compiling. `cognitive_complexity`, `excessive_nesting`, `too_many_lines`, and `wildcard_imports` warn.

Clippy settings: functions up to 90 lines, 6 arguments, cognitive complexity 20, nesting depth 4. Tests may use `unwrap`, `expect`, `panic`, and indexing through the settings file. No test file pastes a suppression at its top.

The binary writes to standard output through one locked writer passed down as a value. It never calls the print macros.

Enforced by: `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and `lint` compares the tables in `Cargo.toml` and `clippy.toml` against the accepted copies so nobody weakens them quietly. The package allows the disallowed-path lint groups at its root and the private core forbids them, because Clippy reads one configuration for the whole package. Unit-test builds lift the three dynamic-JSON lints, `disallowed_methods`, `disallowed_types`, and `disallowed_macros`, in core so the derived schema code compiles. Clippy on the non-test library still enforces them. `policy.py` also checks every core source and dependency root.

## Size

- No source or test file passes 500 non-blank lines. Enforced by: `lint`.
- `sdlc/ratchet.json` holds one ceiling over everything under `crates`, test files included. The ceiling equals the measured total. Adding code means editing the number. The commit that raises it says what grew and why it earns the lines, and where the author looked for duplication to delete first. Enforced by: `sdlc/scripts/ratchet.mjs`, called by `lint`.
- A repository with nothing to lint fails. Enforced by: `lint` fails when it finds no Rust source.

## Errors and types

- The core module defines its own error enums with `thiserror`. No public signature returns a string error or a boxed unknown error. The private engine error carries one of six accepted kinds with its structured cause. The command adds context at the top and maps each error to one exit code in one place.
- A fixed set of choices is an enum. A validated value gets its own type: a probability, a threshold, a JSON Pointer, an option name, a model name. A constructor that can fail returns a result.
- No dynamic JSON inside the core. Wire bodies decode into typed structs. Enforced by: the core module attributes keep the lint group forbidden, and `policy.py` holds the accepted purity table. Ticket 0001 planted each dynamic JSON kind and watched `lint` refuse it.

## Dependencies

- Few, and each one argued. `serde`, `serde_json`, `sha2`, `thiserror`, the engine's `ureq`, and `signal-hook` form the library-only graph. `signal-hook` installs the Unix `SIGXFSZ` handler without unsafe code in this repository; the standard library exposes no safe signal-installation interface. The default `cli` feature adds optional `clap` and `csv-core`. Parallel requests use threads and bounded channels. No async runtime enters until a measurement asks for one. The package rung builds and inspects the default-features-off graph.
- Every dependency resolves from crates.io with a checksum, under MIT, Apache-2.0, Unicode-3.0, or Unlicense. HTTPS forced three more: ISC, BSD-3-Clause, and CDLA-Permissive-2.0, each tied in `policy.py` to the crates that need it. No TLS stack exists in Rust without them. Enforced by: `lint` reads `cargo metadata` and `Cargo.lock`, and `cargo deny` reads the same seven licenses from `deny.toml` and fails on an advisory or on a license nothing in the tree offers.
- Ticket 0380 C admits exactly `windows-sys = "=0.61.2"` with defaults disabled as a nonoptional `cfg(windows)` normal dependency. Production enables only `Win32_Foundation`, `Win32_Security`, `Win32_Security_Authorization`, `Win32_Storage_FileSystem`, `Win32_System_Threading`, and `Win32_System_Console`. The command alone restores inherited Ctrl-C delivery through `GetConsoleCP` and `SetConsoleCtrlHandler(None, false)` before CRT signal registration. Library engine construction installs no process handler. The Windows dev dependency adds only `Win32_System_Console` for the event injector. These library privacy checks use native token, security descriptor and handle APIs absent from the standard library. The existing MIT and Apache-2.0 allowances cover this dependency. `policy.py` checks the complete specifications, plants broadened features, versions, targets and optionality, and scans target normal dependencies from core. The library graph admits the native privacy API and excludes command signal initialization.
- Adding a dependency takes a second reviewing agent and a line in the commit message saying why the standard library would not do.

## Tests

- Red first. A test fails for the stated reason before the code exists.
- Unit tests sit beside pure code in the core and are table-driven.
- Integration tests under `crates/thinkthen/tests/` run the compiled binary and see only arguments, standard input, standard output, standard error, and the exit code.
- No gate touches the network. Tests replay recorded responses from a fixture directory. One test helper serves canned responses from a loopback listener to prove the request bytes. It uses the standard library only.
- Property tests cover every parser and round trip: JSON Pointer, the question set, record framing, the wire format. Property tests also cover any total function over a numeric range, such as the threshold rule. `proptest` is a development dependency of the package and exercises core parsers.
- `spec/*.md` files are executable examples of the command line, run by `mustmatch`. They are the top rung, and they double as the user documentation.
- The binary reads three hidden, test-only variables. `THINKTHEN_TEST_RETRY_WAIT_MS` keeps retry tests short. `THINKTHEN_TEST_SIGINT_ACK` names an exclusive one-byte carrier acknowledgment used to order SIGINT subprocess tests. `THINKTHEN_TEST_INPUT_PAUSE_MS` replaces the 50 ms input pause for piped input, so a stalled reader thread never closes a batch early; the test helpers that end their input set it to 10 seconds. Help never shows these settings. Only a build with debug assertions reads them, so a release binary ignores all three.
- Live calls to a paid backend sit outside the ladder in `sdlc/scripts/live`. They run by hand, under a token cap, with Ian's authorization.
- A claim of secrecy is tested over every command, on the path that succeeds and on each path that fails, and it reads every `Debug` line. One command does not stand for the rest.
- A test that claims nothing was sent counts the requests on the loopback listener. An exit code and `--plan` prove nothing about a live path. Cache prune's `--dry-run` is a separate preview contract.
- A script that checks something runs from a rung. `probes/replay-check.sh` rotted for a day because no rung ran it, and `spec` runs it now.

Package checks compare exported names with an independent declared public contract. Derive a count from that contract when useful; do not maintain a second literal cardinality or derive the expectation from the implementation under test. Keep exact ABI inventories, request bodies and arrival multisets where they prove behavior. The reviewer checks the oracle and any change to it.

A planted negative must prove the intended rejection, including its cause, rather than merely a nonzero exit. Prefer a stable error kind, code or named assertion marker over incidental prose. Keep exact text when wording, secrecy or a migration message is the contract. A changed result envelope requires checking diagnostic expressions as well as positive assertions.

Run package gates and their children with a minimal explicit environment, using `env -i` or an equivalent allowlist. Declare the host's UTF-8 locale, pinned tool paths, required compiler overrides and owned scratch home/cache directories. Fixtures use fake keys and owned loopback servers. Keep dependency locks unchanged; a missing cached dependency is a prerequisite failure. Parent Cargo configuration and child variable forwarding need separate checks. These rules apply to CI and local proof. The package-environment follow-up below tracks incomplete enforcement; a rule on this page is not a passing receipt.

## The ladder

| Rung | Script | What it runs |
| --- | --- | --- |
| 0 | `sdlc/scripts/install` | Verifies the pinned toolchain and the tools the other rungs need |
| 1 | `sdlc/scripts/lint` | Policy and bounded source-package/workflow routing checks, the ratchet, the file ceiling, `cargo deny`, `cargo fmt --check`, clippy with warnings denied, `cargo doc` with warnings denied |
| 2 | `sdlc/scripts/test` | Selected routine functional cases, parser and secrecy regressions, consumer checks and supporting script checks |
| 3 | `sdlc/scripts/spec` | The compiled binary against `spec/*.md` |
| 4 | `sdlc/scripts/surfaces` | Each landed binding's `check.sh` against one loopback backend (ADR 0047) |

Run the smallest relevant format, lint and functional checks during each change. Run full tests and lint on the landing commit. Run `spec` and affected surface checks when the change needs them. Load, churn, timing and contention belong only in explicit `test-stress --run` jobs. Additional functional cases use `test-full-cases --run`. Neither runs automatically at each handback.

The complete `sdlc/scripts/package` validation is a separate explicit packaging checkpoint and a required step of the manually dispatched release `crate` job before artifact upload. It includes library-only tests, internal doctests, private export probes in both feature profiles, stale archive cleanup, a fresh unpacked source build, transform bytes, and release panic modes; routine lint does not run it.

## When the repository goes public

`LICENSE` is here. Ian ruled MIT in ADR 0015, each package declares `license = "MIT"`, and `policy.py` checks both. `.github/workflows/gate.yml` runs the four rungs on every push and every pull request, and ADR 0015 item 6 rules it. Each action in it is pinned to a commit SHA and the downloaded `jq` is checked against a recorded SHA-256. `deny.toml` allows exactly the licenses the tree carries, and `lint` runs `cargo deny check advisories bans licenses` over it. `CHANGELOG.md` still waits.

## Not yet enforced

- `unwrap_used` skips a result whose error type cannot occur. The lint bans a risk and leaves the token legal.
- Clippy ignores a ban-list path it cannot resolve, and a typo in any ban passes quietly. The dynamic JSON bans were proved in ticket 0001. Every ban added later has to be planted and refused the same way.
- No tool checks that parsing happens only at the edge or that a validated value has its own type. The reviewing agent checks both and says so in its review.
- No tool requires the second reviewer for a new dependency. The dependency rule above and the review rule in `AGENTS.md` govern review.
- Minimal environments are not yet enforced for every package gate and child. The [package verification issue](../issues/closed/2026-09-29-nine-package-gates-fail-from-clean-checkouts.md) owns the remaining runner checks, including the release families introduced by 0269–0272. Each correction must prove the intended case executes under the declared environment.
- The five factory scripts under `sdlc/project/` are absent. Copy them when this repository registers with the factory.
