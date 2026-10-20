# 0639: Move the command-line tool and MCP server out of the core crate

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0625, 0629

## Outcome

- The library crate holds only the library. The command-line tool and the MCP server live in their own crate (or crates) and use only the library's public API. Any private access they need becomes public API or moves with them.
- The `cli` feature and the `cfg(feature = "cli")` blocks in the library crate are deleted.
- The MCP server executes tools through the Rust run function from 0629, with no session loop of its own.
- The released binary name, install path and behavior do not change.

### Proof

- On the M5 through pm job, the packaged `thinkthen` binary from the release suite (`package`) passes `conformance/check-cli.sh` and the spec page replays (`sdlc/scripts/spec`). The MCP stdio checks pass against the installed binary.
- The installed C library and the Rust library consumer pass their routine checks.

## Evidence

`crates/thinkthen` holds the library, the command-line tool (`src/cli`, `src/main.rs`) and the MCP server (`src/mcp`), switched by the `cli` feature (`crates/thinkthen/Cargo.toml`, `src/lib.rs:52-67`). The crate is about 107,000 source lines and exports about 216 public type names. Every binding builds it with default features off and depends on the feature switch to leave the CLI out. `lib.rs:61-64` pulls CLI modules into the crate root (`args`, `asking`, `edge`, `failure`, `judge`, `profile`), so CLI and library code can reach into each other.

The CLI is among the most churned code since 2026-09-15: `cli/args.rs` 68 commits, `cli/args/command.rs` 62, `cli/asking.rs` 59, `cli/mod.rs` 57, `cli/failure.rs` 51. Each of those changes rebuilds and re-reviews the library crate that every binding uses.

## Progress

- 2026-10-19 started
- 2026-10-19 next: M5 public-boundary job1792439651280-32898 at878dcd9b4 passed261 tests with6 existing ignored, and the separately pushed environment/status port647531555 now returns to the main developer for integration with reviewed warning-order bridges, while CLI compilation, complete command and package proof and whole review remain unfinished.
- 2026-10-19 next: All four command-port handoffs are merged and their temporary worktrees removed, and the developer is completing ordinary-session execution with reviewed native find-count, unrounded plan-combination and narrow annotation-pointer boundaries before the first integrated command build and whole-ticket proof.
- 2026-10-19 next: Whole-workspace all-target compilation passed in M5 job1792447159242-78030; the first complete runtime batch1792448553033-85000 at8d1dd71f2 ran every group with186 passes13 failures13 ignored, and the developer is repairing the collected admission, partial-result and fixture failures before one targeted rerun.
- 2026-10-19 next: Targeted M5 job1792449818876-26860 atc820d33e5 passed68 session and stdio tests with one ignored case, preserving refusal send counts and completed prefixes; the original rank late-refusal assertion stays unchanged pending the PM public-boundary ruling, and full closing gates remain outstanding.
- 2026-10-19 next: M5 policy-only job1792452043826-57992 passed at0aad6c0b1, preserving prior formatting, API, compile and runtime passes; rank boundary ruling remains pending before fresh whole review and package proof.
- 2026-10-19 next: PM approved the default-off Rust feed incremental_ranking option for ordinary rank only;356cdd141 preserves default zero sends and the original CLI prior-batch send, with M5job1792452495684-32759 passing58 session tests and the CLI regression while fresh whole review owns remaining lint and package proof.
- 2026-10-19 next: Fresh review of 7c0b23bd6 found no remaining source issue; M5 job1792459538740-56567 passed workspace checks, installed Rust,243 CLI cases,24 spec demos,MCP and archive checks, and job1792461829556-55532 passed fresh installed C with the c tool group; the configured closing gate remains before acceptance.
- 2026-10-19 next: Configured closing M5 gate1792462660817-73865 at119309745 failed children isolation checks on three environment.rs constructors hidden behind a configuration closure; reviewer is moving shared child clearing to each constructor and preserving evidence stdin. Reviewed Rust RequestSessionFeedOptions.interrupt owns an optional Arc callback, defaults to None and may run concurrently on workers; borrowed CallOptions interruption remains on its caller thread. incremental_ranking defaults to false and admits only ordinary rank feeds, with no wire, C, MCP or host field. Installed C proof1792461829556-55532 and prior package proofs remain passing; exact-source closing gate follows the isolation fix.
- 2026-10-19 next: Full configured gate1792463268691-7147 at3c9eba4a8 passed Linux lint including generation, formatting, clippy, docs and strict public inventory2439 items with4 hostile plants; workspace runtime ran2158 tests with2121 passing,36 failing,1 certificate-case timeout and76 skipped. Earlier focused session/command checks and243 shared installed CLI cases omitted the whole moved backend suite, so they missed command wording, metadata, physical ordinal, fixture-path and settlement regressions. Reviewer remains sole writer and is batching repairs with unchanged assertions before another full run; installed C proof remains queued after generated carrier changes. Earlier setup1792462851127-79133 failed because an extra launcher hid host Docker, not source; direct invocation fixed setup. Generation ordering proof1792463225071-93391 passes after C producers moved ahead of host ABI projections.
- 2026-10-19 next: Batched Linux focus1792464629090-27264 atff40722ad ran all37 previously failed cases:30 passed and7 failed, with1035 unselected; CA now passes with unchanged assertions and no deadline changes. Formatting, children57/57 self-checks and0 findings, docs and strict public inventory2440 declarations with4 hostile plants passed. Reviewer pushed79697744e to retain saved Local causes, complete streamed rank presentation, known early command time, native Relate grammar ownership and command-specific model diagnostics; all original assertions remain. No full gate or installed C rerun until all37 focus cases pass. The only additional public bridge is reviewed AttemptObservation.transport_failed, which exposes the retained precise transport flag because the broader outcome also includes reply-byte refusals; canonical attempt serialization stays unchanged.
- 2026-10-19 next: Configured gate1792465446868-33727 at0940b021a passed Linux lint and2155 of2158 workspace cases with76 skips, exposing three native-kind/admission-order regressions missed by the green37-case CLI focus; pushed6c56d07c3 restores library kinds and schema-first Find count with a typed diagnostic, and the next focus includes complete native_named_questions,request_contract and native_command_boundaries binaries before the full gate and final package/C checks.
