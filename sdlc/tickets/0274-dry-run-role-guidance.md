---
flow: build
priority: 274
opens: sdlc/issues/2026-09-20-new-user-stumble-register.md
---

# 0274: Explain the question and evidence in an interactive dry run

Status: design candidate for independent review. Source pin: `60aec53e` (`origin/main` at preparation). [Preparation](../records/0274-dry-run-role-preparation.md) traces the plan, channels, and existing proof. This ticket addresses row 4 of the new-user stumble register; it does not close the register.

## Outcome and boundary

For an ordinary **one-document** `decide`, `filter`, `rank`, `choose`, `tag`, or `score` dry run whose standard output is a terminal, explain the roles before the JSON appears on screen. Write this exact one-line hint to standard error:

`thinkthen: dry-run: request.state is the evidence; request.questions holds what you asked about it.`

The JSON remains one compact, unchanged `PlanDocument` on standard output, including its field names, order, and key-free request body. If standard output is a pipe or file, standard error gains no hint, even if standard error is a terminal. A refused or empty plan gains no role hint. The existing standard-input-terminal waiting notice may appear first. No option, environment setting, or schema field is added. A dry run still needs no key and opens no backend connection.

**Adjustment to row 4's old remedy:** it asks for prose “above the JSON.” That is the intended interactive visual order, but prose on standard output would break the settled one-document JSON contract and current CLI consumers. Standard error is already the person-facing channel. Terminal-only guidance addresses the observed first-user command while preserving captured plans and scripts. The coordinator can approve this routine channel choice and should record the adjustment when closing row 4.

Do not apply the sentence to record-mode plans. In a batched `decide` plan, `request.state` can say `Each question quotes the text it asks about.` and the actual record evidence is quoted inside each `request.questions` entry ([executable example](../../spec/decide.md)). Nor does this ticket change `find`, `annotate`, `recognize`, or `relate`, whose dry-run paths or schemas are separate. The current [channels reference](../../specification/channels.md) continues to explain their machine-readable plans.

## Implementation and smallest proof

1. In `crates/thinkthen/src/cli/asking/plan.rs`, after the one-document plan has been validated and serialized, use the real `stdout.is_terminal()` boundary to emit the exact fixed hint to stderr before the existing JSON write. Keep the emission local to the ordinary asking plan path and suppress it when `reading.streams()` is true. Do not inspect, interpolate, or echo question or evidence text. Reuse the existing stderr write/failure convention; do not add a second plan document or alter `PlanDocument`.
2. In `specification/channels.md`, state the one-document interactive behavior and the stdout/stderr rule beside `--dry-run`; retain the separate record-mode explanation. In `spec/decide.md`, add one focused executable channel assertion only if its existing piped JSON checks do not already pin the preserved behavior. Update `sdlc/ratchet.json` to the measured source total if Rust source grows.
3. First show the missing terminal hint with a focused red check, then green it. A small test at the terminal/output boundary may pass a terminal boolean because that is the production input, as `edge::waiting_on_terminal` already does; it must assert the exact user-visible bytes for terminal and nonterminal cases and have no test-only hook. Reuse `crates/thinkthen/tests/decide_edge.rs`'s exact piped stdout and empty-stderr assertion and `crates/thinkthen/tests/backend/exchange.rs`'s zero-loopback-request dry-run case as outside-in contract proof. Avoid adding a duplicate scaffold case to the nearly capped `decide_edge.rs`. A focused PTY CLI assertion is preferable if an existing portable fixture supports it; do not add timing, stress, provider, or full-suite work for this message.
4. Run focused format, lint, relevant functional/spec checks, diff and policy checks. Name any changed source-size total and review the public channel change and ratchet under the repository's second-agent rule. The coordinator names any later broad-rung checkpoint.

Exact intended files: `crates/thinkthen/src/cli/asking/plan.rs`, `specification/channels.md`, this ticket, and `sdlc/records/0274-dry-run-role-build.md`; `sdlc/ratchet.json` only for measured Rust growth. `spec/decide.md` and a focused test file are conditional on the smallest proof actually needed. No other source path, adapter, key handling, or site file is in scope.

## Deferred gaps

Record and aggregate dry-run plans require their own accurate role explanations, since evidence placement differs. This ticket makes no claim about first-hour stumbles 9 or 18, nor about live answer wording. The root owns issue closure after reviewed implementation.

## Evidence

- Starts from: Register row 4's observed swapped-input command, the settled five-channel and four-field dry-run contract in `specification/channels.md`, and source pin `60aec53e`.
- Keeps: Compact JSON alone on stdout; exact `PlanDocument` schema; quiet captured stderr; key-free, no-send dry runs; the stdin-terminal waiting notice and separate aggregate schemas.
- Changes: Adds one fixed, terminal-only stderr role hint for ordinary one-document asking plans, before the JSON is displayed.
- Proof: Exact terminal/nonterminal hint bytes at the real TTY boundary; existing `decide_edge.rs` exact captured JSON/empty stderr and `backend/exchange.rs` zero-request dry-run case; focused page and policy checks.
- Defers: Record-mode and aggregate role guidance, unrelated register rows, provider work, stress, and full-suite checkpoints.

## What the build taught us

Pending implementation, focused proof, and fresh code review. Record assumptions corrected, proof adjustments, and remaining gaps here before landing.
