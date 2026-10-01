# Mainline readiness and ticket 0074 completion

Date: 2026-09-23. Budget: 12,000 characters. Ian requested this assessment and plan. It serves the reliable engine, ten functions, library, and installation gaps. Ian can overturn the order. ADR 0017 and the build queue govern accepted behavior and ownership.

## Assessment

Current update: tickets 0074, 0087, 0079, 0080, and 0081 are landed on remote main. Ctrl-C, documentation corrections, deterministic request splitting, recognition, and the shared relation foundation pass independent review and the full local ladder. The remaining authorized launch-first queue is 0088 public `relate` command, 0082–0083, 0076–0078, then 0084–0086. The Luna 0081 trial stopped after two design-remediation passes. Sol Medium now drives accepted ticket 0088 with independent Sol review.

Eight commands work, Ctrl-C and request splitting are on main, and local checks are strong. Two new functions, command completion, the remaining private controls, the public Rust API, adapter integration, and installation remain unfinished. The source-package gate does not establish an installable 0.1 release.

## Repository snapshot

The review started on clean main `2b63c872925c7237ded0491dc13f69247cf6f142`, equal to remote main by `git ls-remote`. Rust and Cargo report 1.93.1. Plan edits use `thinkthen-mainline-review`.

| Checkout | Observed state | Meaning |
| --- | --- | --- |
| `repos/thinkthen` | Clean main at `2b63c87` | Production core, engine, and CLI live under `crates/thinkthen/src/{core,engine,cli}` |
| `thinkthen-0074` | `ticket/0074-cli-sigint` at `b3d9133`; 11 tracked modifications and 4 untracked paths | Active draft, including its untracked ticket. No 0074 code commit or remote branch existed. Main was three documentation commits ahead |
| `thinkthen-surfaces` | Clean `surfaces` at `f942e06`, equal to its remote | Language/database adapters, contract, and stand-in engine remain separate. Comparison with main found 266 main-only and 246 surfaces-only commits |

Git listed 87 worktrees. `surfaces` was the only local branch with commits outside main. Existing worktrees and unrelated site edits were preserved.

## What is already on main

- All eight judgment commands: `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `annotate`, and `find`. Record framing, CSV/TSV input, detailed results, transforms, recording, replay, and nineteen executable how-tos are present.
- Tickets 0053–0065 added request identity, preserved good partial answers, the one-package boundary, corrected filtering, backend limits, records beside answers, durable recording, the bounded default cache, status/counts, bounded retries, and backend-bound recording folders.
- Tickets 0066–0069 improved help and structured questions/evidence. `meta.cached` is canonical; historical readers remain supported.
- Ticket 0072 makes a refused connection fail after one attempt. Ticket 0073 adds the private cancellation token and checks in schedulers, requests, retry waits, and recording-lock waits. It does not bind Ctrl-C.

Quality checks enforce the pure core, worker lifetimes, the 500-nonblank-line file limit, and an exact total ceiling. Gates exercise replay, local listeners, failures, documentation, and package boundaries. Preserve independent review and coordinator-run gates.

## Complete ticket 0074 next

The accepted ticket is `sdlc/tickets/0074-bind-sigint-to-cooperative-cancellation.md` in the 0074 worktree, absent from main. It keeps process-lifetime signal state, uses the engine token, and routes Unix SIGINT through a CLI carrier to protect sent HTTP attempts. Preserve the reviewed protocol and dependency justification.

Observed gaps in the draft:

- `cli/interrupt/tests.rs` exists but is empty. Tests remain inline in `cli/interrupt.rs`, currently 481 nonblank lines. Creating the empty file did not implement the accepted split.
- The record subprocess test uses default width and releases its reply immediately after SIGINT. It lacks `--jobs 1` and the accepted acknowledgment. A reviewer ran `cargo test --locked -p thinkthen --test backend interrupt::`: two passed and one failed with empty stdout and stopped-at record 1. Two exact reruns passed. This is insufficient completion evidence.
- The carrier blocks on its stop channel. The named acknowledgment seam in the amended ticket is absent. The three backend tests do not cover all accepted input-wait, mask, initialization, and cleanup-failure cases.
- The sticky-initialization test does not drive repeated failed initialization through the production initialization path. The accepted failure-position proofs remain work to do.

Completion order:

1. Preserve the existing draft and accepted ticket. Review the three intervening main commits before integrating them. Keep the library worktree separately owned. Commit the ticket with its completed implementation; do not confuse it with main's unrelated `sdlc/records/0074-the-r-interrupt-window.md`.
2. Move the interrupt-state tests into the opened `cli/interrupt/tests.rs` module. Keep every source/test file within 500 nonblank lines. Measure the accepted production limit of five Rust files and 360 added nonblank lines, and the total limit of 900 added nonblank Rust lines. Do not quietly relax a budget if the remaining proof exceeds it; reduce duplication or obtain independent acceptance of a bounded amendment.
3. Implement the already-reviewed hidden `THINKTHEN_TEST_SIGINT_ACK` seam in the normal carrier thread. Acknowledgment follows observed cancellation and precedes an already-queued stop. Exclusively create a regular file, write one byte once, refuse existing files/symlinks without truncation, record seam failure without changing product behavior, and clean up every subprocess fixture path. Keep help and normal output free of the seam. Document it with the other hidden test settings.
4. Make the record proof deterministic with `--jobs 1`, a listener event, SIGINT, acknowledgment, then reply release. Assert the exact completed row, stopped-at record 2, one POST, a complete cached entry, and Unix signal status. Apply the same acknowledgment ordering to sent `decide`, aggregate `find`, and retry-wait cases. A sleep or a successful `kill` call cannot establish handler completion.
5. Complete the accepted state and routing proofs: four registration actions and their interleavings; active-entry exclusion; two sequential entries sharing the reset token; sticky failure at each registration position; actual worker mask inheritance; ordinary and annotate input waits; later-signal forced termination; and injected mask, spawn, readiness, restoration, and emulation failures. Assert exact diagnostic and exit precedence, carrier join where cleanup returns, mask restoration, and active-claim release.
6. Have the independent reviewer reconcile the cleanup assertions with the accepted termination contract. Successful default emulation terminates the process; the design expressly gives cancellation precedence. Do not introduce a cleanup-before-emulation redesign merely from an ambiguous test description. Prove all returning normal, error, and unwind paths, including emulation failure.
7. Run focused tests, policy, exact ratchet, formatting, Clippy, and `git diff --check`. Independent code review must cover signal ordering, the safe Unix-only optional `nix` dependency, target/feature gating, license/lockfile, added lines, and all amended acceptance criteria. Preserve red-then-green evidence for the race and missing proofs.
8. After acceptance, run `sdlc/scripts/{install,lint,test,spec}` sequentially on the integrated current-main tree. Record the exact revision and results, close the SIGINT issue, update the ticket/queue, commit, land, and push. Verify remote main contains the implementation. Keep Actions disabled under Ian's ruling.

Done means the actual Unix subprocess dies from SIGINT after completed output is flushed; a shell therefore reports 130. Merely returning integer 130 is not the signal proof. A first signal prevents new attempts that observe cancellation and lets started attempts finish. The later signal remains the escape from blocked input or shutdown. Ticket 0074 does not complete deadlines, process-wide width, fork recovery, or host ownership of the recorder's SIGXFSZ handler.

## After 0074

1. Complete the remaining private controls in bounded reviewed tickets: process-wide width, whole-call deadlines, fork recovery, and host signal ownership. Fast refused-connection failure and the private cancellation token are already landed.
2. Land ticket 0088's public `relate` command over the completed shared foundation, with the complete `@entities` grammar, saved calibration identity, exact dry-run provenance and backend-profile name, Option A schema, settled empty-input behavior, partial output at exit 6, documentation, and secrecy proof. The settled hybrid method needs no paid measurement.
3. Open the real Rust API over all ten functions after controls and result shapes are stable. The crate currently exposes `entry()` only; its public judgment API is unfinished. Add C and then integrate the separately owned surfaces through a reviewed merge. Replace the stand-in and duplicate parser/scheduler behavior; prove request counts, cancellation, partial results, and ownership against the real engine.
4. Prepare installable artifacts and rehearse them locally: release archives/checksums, the agreed Homebrew and download-script paths, clean-prefix installation, Linux/macOS checks, manual/help/skill/catalog work, missing package metadata, and release documentation. The current version is `0.0.1` with `publish = false`; the source-package check is not a release installer.
5. Re-run the remaining quality work against the installed real artifacts. The accepted first release remains simultaneous 0.1 across the agreed surfaces. No publication or registry-name action follows from this planning update.

## Surfaces evidence and limits

Main contains review documents about `surfaces`; it does not contain that branch. The [third review](../issues/closed/2026-09-22-surfaces-branch-third-review-the-unheld-fixes.md) closes most tested findings at `f942e06` and names the remaining proof gaps. Stand-in passes do not prove real-engine integration.

This review does not rerun the surface runtime suites, macOS checks, website deployment, or live model quality tests. No paid call ran. GitHub Actions remains paused; local results supply current gate evidence.

## Verification

On main `2b63c87`, the coordinator ran `env -u THINKTHEN_API_KEY -u THINKTHEN_BASE_URL sh sdlc/scripts/install`, then `lint`, `test`, and `spec` sequentially with `CARGO_NET_OFFLINE=true`. All exited 0. Install may fetch dependencies/advisories; later rungs use offline dependencies and local fixtures. Lint passed package, policy, pages, dependency audit, formatting, Clippy, docs, and ratchet `36701/36701`. Rust tests reported 633 passed and one intentional ignore. Doctests, schema/probe/transform/live-wrapper fixtures, executable specs, replay checks, and nineteen green how-tos passed. Documentation changes passed `git diff --check`.

Independent readers checked worktrees, interruption, packaging, and ticket status. The coordinator rejected one initial teardown finding: pre-cleanup default emulation follows the accepted cancellation precedence.

Independent Sol review accepted the planning changes. Pages, relative links, character budgets, ratchet, and whitespace checks passed.
