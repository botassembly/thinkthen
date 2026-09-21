---
flow: build
priority: 35
opens: Cargo.toml Cargo.lock crates conformance sdlc/scripts sdlc/planning sdlc/ratchet.json
---

# 0055: Fold to one crate through the private engine

Status: landed

## Outcome

The repository contains one package named `thinkthen`, with a library and the existing command. The pure core, private engine, and command edge are modules in that package. The move changes no request, retry, recording, output, diagnostic, exit code, order, width bound, or input behavior.

## Current facts and decisions

Ticket 0051 settled the boundary after reviewing the accepted ADR 0017. Today the workspace still has two packages. Command files also mix input reading, request work, output, and command failures, so moving files whole would violate the boundary.

This ticket implements section 8 step 1 and nothing later. Ian can overturn these decisions.

1. `core` contains the current pure decision types and rules. It depends only on `serde`, `serde_json`, `sha2`, and `thiserror`, and cannot refer to `engine` or `cli`.
2. `cli` keeps arguments, environment and credential lookup, file and standard-input framing, CSV and TSV input, the detached input reader, output, diagnostics, and exit codes.
3. `engine` owns sending, retries, request workers, recording, and cache locks. It receives resolved settings and framed records and returns typed per-row events through a bounded private bridge. It never owns `clap`, a writer, `ExitCode`, or the command's `Failure`.
4. Engine workers and internal feeders are scoped and joined before the engine call returns. The command-owned reader stays detached because it may be blocked on standard input when a completed answer or closed pipe ends the run.
5. The `cli` feature is on by default and required by the binary. Command-only dependencies are optional. A default-features-off build contains the library without command dependencies. The private engine and core gain no public API in this ticket.
6. The library release path unwinds panics. The command release path keeps the current abort behavior.
7. The command becomes the first full runner of `conformance/cases.json`. Successful cases cross the private engine boundary through embedded exchanges. The `local`, `deadline`, and `defect` cases use deterministic in-process injection and make no network request.

## Scope

Move the current core into the `thinkthen` package, extract the private engine boundary from the mixed command files, remove the `thinkthen-core` package and path dependency, add the first command runner over the shared conformance file, and update repository architecture, policy, package, and build checks for the one-package layout.

Excluded: public engine functions or errors, backend profiles, returning each input record with its answer, engine settings, a default cache, cancellation, fork repair, faster refused connections, `recognize`, `relate`, the C interface, request or result shape changes, paid calls, and release publication.

## Acceptance

- Cargo has one workspace member and one package named `thinkthen`. The package has a library target and a binary that requires the default `cli` feature. `clap` and `csv-core` are optional, and the lock file has no `thinkthen-core` package.
- A default-features-off library build and dependency-graph check exclude command dependencies. Core doctests still run. A package built from Cargo's generated package tree succeeds with no sibling crate.
- The complete `core` tree retains its prohibited-API rules. Policy scans every core source for a reverse reference to `engine` or `cli` and for a dependency outside `serde`, `serde_json`, `sha2`, and `thiserror`. Self-tests plant and catch one prohibited standard API, one reverse reference, and one `ureq` reference.
- The private bridge passes typed framed inputs and typed per-row results under a fixed bound. Engine code has no command argument, writer, diagnostic, exit-code, or command-failure type. CLI maps structured engine failures back to the current sentences, counts, and codes.
- Existing scheduler tests prove stable input order, jobs bounds, flat read-ahead, interactive one-row progress, partial answers, early closed-pipe stopping, retries, replay, and recordings. Existing request and output fixtures remain unchanged.
- A surviving-process test calls the engine, returns from it, and proves that no engine worker or internal feeder remains. The test does not require the detached command input reader to finish.
- One offline command runner reads every case in `conformance/cases.json`. It runs every successful case through the private engine with its embedded exchange and deterministically injects `local`, `deadline`, and `defect` at that boundary. It checks the shared expected answer or failure kind and uses no key or network.
- Release checks prove unwind behavior for library use and abort behavior for the command build.
- Repository architecture and Rust standards describe the one-package layout. User behavior pages do not change.
- The exact size ratchet, all four repository rungs, and `git diff --check` pass with the key and base address unset and without a network request.

## Dependencies

Accepted ADR 0017, tickets 0051 through 0054, and `sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`.

## Complexity

- Contract: 1
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 7
- Minimum level floor: level 3 for concurrent scheduling and worker lifetime
- Final level: 3
- Reasons: the public behavior is already fixed, but the private split crosses both schedulers, retries, recordings, feature resolution, packaging, and thread lifetime. Compatibility and bounded execution require several independent proofs.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the move requires a public type, a changed command contract, a new dependency, or a new worker lifetime.

## Review

Independent design review rejected the first proposal because it omitted the shared conformance runner assigned to the merge by ticket 0052 and ADR 0017. The repair adds every successful shared case plus deterministic `local`, `deadline`, and `defect` injection through the private engine boundary. The same reviewer accepted the corrected design.

Independent code review rejected the first implementation because the command still owned both scheduling state machines, the core policy had lost accepted bans, successful conformance cases did not check their answers, the worker-lifetime test bypassed the real scheduler, and two behavioral doctests had become privacy checks. The repair moved scheduling state into the engine, restored and mechanically checked every core ban, checked complete successful results and all six fault kinds through the engine, exercised the production scheduler, and restored the two behavior assertions without exposing a normal public surface.

The reviewer then found import spellings that could evade the source policy. The final checker tokenizes Rust, skips comments and literals, understands use trees and raw identifiers, and rejects crate-root, ancestor, dependency, and wildcard paths that cross the core boundary. Its self-test plants each rejected form and allowed control. The same reviewer accepted the final repair and confirmed the earlier findings remained closed.
