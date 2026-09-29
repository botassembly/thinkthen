---
flow: build
priority: 274
opens: sdlc/issues/2026-09-20-new-user-stumble-register.md
---

# 0274: Explain the question and evidence in an interactive dry run

Status: complete. Fresh independent Medium code review accepted `447adf0c9`; the coordinator integrated its unchanged implementation and focused proof. Preparation source pin: `60aec53e`; implementation merged approved main `1b1f09d70` first. [Preparation](../records/0274-dry-run-role-preparation.md) traces the plan, channels, and existing proof; the [build record](../records/0274-dry-run-role-build.md) gives focused results. This ticket addresses row 4 of the new-user stumble register; row4 is closed and the register remains open for its other criteria.

## Outcome and boundary

For an ordinary **one-document** `decide`, `choose`, `tag`, or `score` dry run whose standard output is a terminal, explain the roles before the JSON appears on screen. Write this exact one-line hint to standard error:

`thinkthen: dry-run: request.state is the evidence; request.questions holds what you asked about it.`

The JSON remains one compact, unchanged `PlanDocument` on standard output, including its field names, order, and key-free request body. If standard output is a pipe or file, standard error gains no hint, even if standard error is a terminal. A refused or empty plan gains no role hint. The existing standard-input-terminal waiting notice may appear first. No option, environment setting, or schema field is added. A dry run still needs no key and opens no backend connection.

**Adjustment to row 4's old remedy:** it asks for prose “above the JSON.” That is the intended interactive visual order, but prose on standard output would break the settled one-document JSON contract and current CLI consumers. Standard error is already the person-facing channel. Terminal-only guidance addresses the observed first-user command while preserving captured plans and scripts. The coordinator approved this channel choice in the [design review](../records/0274-dry-run-role-design-review.md); root should record the adjustment when closing row 4.

Do not apply the sentence to record-mode plans. `filter` and `rank` always read records: `cli/asking/reading.rs::read_by` selects lines or JSONL when no framing flag is given. They have no eligible one-document route. In a batched `decide` plan, `request.state` can say `Each question quotes the text it asks about.` and the actual record evidence is quoted inside each `request.questions` entry ([executable example](../../spec/decide.md)). Nor does this ticket change `find`, `annotate`, `recognize`, or `relate`, whose dry-run paths or schemas are separate. The current [channels reference](../../specification/channels.md) continues to explain their machine-readable plans.

## Implementation and smallest proof

1. In `crates/thinkthen/src/cli/asking/plan.rs`, after the one-document plan has been validated and serialized, use the real `stdout.is_terminal()` boundary to emit the exact fixed hint to stderr before the existing JSON write. Keep the emission local to the ordinary asking plan path and suppress it when `reading.streams()` is true. Do not inspect, interpolate, or echo question or evidence text. Reuse the existing stderr write/failure convention; do not add a second plan document or alter `PlanDocument`.
2. In `specification/channels.md`, state the one-document interactive behavior and the stdout/stderr rule beside `--dry-run`; retain the separate record-mode explanation. Update `sdlc/ratchet.json` to the measured source total if Rust source grows.
3. First show the missing hint with a focused **compiled CLI** PTY case in `crates/thinkthen/tests/dry_run_terminal.rs`, then green it. The existing `decide_edge.rs` harness pipes all three streams and cannot observe `stdout.is_terminal()`; no PTY fixture exists today. Invoke the compiled `CARGO_BIN_EXE_thinkthen` through Python's standard-library `pty` on Unix, with stdin piped to avoid the waiting notice and no key. Capture stdout and stderr on one raw PTY to assert the **exact hint before the JSON** and a parseable single plan. Capture the channels separately with stdout on a PTY and stderr piped to prove the hint is on stderr and stdout contains only JSON. Reverse them (stdout piped, stderr on a PTY) to prove no hint appears when stdout is not a terminal. Keep the process bounded and use no external dependency, network, test-only production hook, timing assertion, or new framework. A boolean-helper-only test is insufficient because it cannot prove actual `stdout.is_terminal()` wiring. Reuse `crates/thinkthen/tests/decide_edge.rs`'s exact piped stdout and empty-stderr assertion and `crates/thinkthen/tests/backend/exchange.rs`'s zero-loopback-request dry-run case; avoid adding a duplicate case to the nearly capped `decide_edge.rs`.
4. Run focused format, lint, relevant functional/spec checks, diff and policy checks. Name any changed source-size total and review the public channel change and ratchet under the repository's second-agent rule. The coordinator names any later broad-rung checkpoint.

Exact changed files: `crates/thinkthen/src/cli/asking/plan.rs`, `crates/thinkthen/tests/dry_run_terminal.rs`, `specification/channels.md`, `sdlc/ratchet.json`, this ticket, `sdlc/records/0274-dry-run-role-build.md`, and register row 4 in `sdlc/issues/2026-09-20-new-user-stumble-register.md`. Existing `spec/decide.md` already pins piped JSON, so it needs no edit. No other source path, adapter, key handling, or site file is in scope.

## Deferred gaps

Record and aggregate dry-run plans require their own accurate role explanations, since evidence placement differs. This ticket makes no claim about first-hour stumbles 9 or 18, nor about live answer wording. The root owns issue closure after reviewed implementation.

## Evidence

- Starts from: Register row 4's observed swapped-input command, the settled five-channel and four-field dry-run contract in `specification/channels.md`, and source pin `60aec53e`.
- Keeps: Compact JSON alone on stdout; exact `PlanDocument` schema; quiet captured stderr; key-free, no-send dry runs; the stdin-terminal waiting notice and separate aggregate schemas.
- Changes: Adds one fixed, terminal-only stderr role hint for one-document `decide`, `choose`, `tag`, and `score` plans, before the JSON is displayed.
- Proof: Compiled CLI PTY case for exact hint order, channel separation, and piped-stdout suppression; existing `decide_edge.rs` exact captured JSON/empty stderr and `backend/exchange.rs` zero-request dry-run case; focused page and policy checks.
- Defers: Record-mode and aggregate role guidance, unrelated register rows, provider work, stress, and full-suite checkpoints.

## What the build taught us

The preparation correction about `filter` and `rank` prevented a false one-document promise. The compiled CLI PTY test proved the actual stdout terminal decision, interactive order, stderr placement, piped suppression, and a record-only `filter` exclusion; no helper-only scaffold was added. The existing exact piped JSON and zero-loopback-request tests passed. Python's standard-library PTY made the Unix-only terminal proof possible without a new dependency. Fresh code review caught two test-process errors: cleaning the CLI grandchild did not clean Python's inherited environment, and `communicate(timeout=10)` alone could leave the CLI running after timeout. The correction clears Python's environment and bounds child kill/reap and PTY cleanup on all exits. The only source growth is the local output boundary; the new test owns the rest of the ratchet rise. Future tests must check every subprocess level and timeout path. Record and aggregate guidance remain outside this ticket. Fresh re-review and root's issue closure remain.
