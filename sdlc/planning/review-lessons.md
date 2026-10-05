# Lessons from thinkthen's code reviews

## 1. Mistakes reviewers actually found

- `assert!(choose.contains("3"))` passed on any help text with a 3 in it — `sdlc/records/0009-choose-and-score.md`
- The one secrecy test covered `decide` alone, not `choose` or `score` — `sdlc/records/0009-choose-and-score.md`
- Nine "before any request" list-refusal tests ran under `--dry-run`, which sends nothing regardless — `sdlc/records/0009-choose-and-score.md`
- Four backend test files each pasted the same ~20 lines of process-spawning setup — `sdlc/records/0009-choose-and-score.md`
- `score --threshold 0.5` drew a misleading clap usage tip naming `--record` — `sdlc/records/0009-choose-and-score.md`
- A key and a request body (carrying the evidence) printed through derived `Debug` — `sdlc/records/0003-answer-decide-if-from-the-shell.md`
- The response-body read had no byte cap — `sdlc/records/0003-answer-decide-if-from-the-shell.md`
- The HTTP client followed redirects by default, so evidence could cross to another host — `sdlc/records/0003-answer-decide-if-from-the-shell.md`
- Recording files and folders were created group/world readable under a common umask — `sdlc/records/0004-record-and-replay-backend-exchanges.md`
- Every read failure on a recording entry was reported as "no recording," hiding a corrupt file — `sdlc/records/0004-record-and-replay-backend-exchanges.md`
- A failed rename left a temp file holding evidence behind on disk — `sdlc/records/0004-record-and-replay-backend-exchanges.md`
- `sdlc/scripts/live` read `THINKTHEN_LIVE_LEDGER`, letting any caller redirect the spend limit to a throwaway file — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0006.md`
- The demos script captured an empty `--replay` folder name from backticked prose and silently skipped the missing-folder guard — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0006.md`
- A test harness file opened with `#![allow(dead_code)]`, a suppression the standards forbid — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0006.md`
- A landed record reported the wrong ratchet number and demo count — `sdlc/records/0006-the-two-variables-the-live-script-and-the-first-green-demo.md` (Correction section)
- `AGENTS.md` kept describing a key read from "its backend profile" after profiles were removed — `sdlc/issues/closed/2026-09-19-agents-md-still-describes-profiles.md`
- A "swept word" grep after removing profiles missed the bare words `profile` and `adapter`, leaving two stale sentences — `sdlc/records/0007-remove-profiles-and-the-configuration-surface.md`
- A `jq` transform's `//` operator read `false` and a missing value alike, turning unresolved into "no" — `sdlc/records/0008-metric-recipes-over-live-decide-rows.md`, `sdlc/issues/closed/2026-09-19-ideas-carried-from-the-design-captures.md`
- Duplicate case ids were silently counted twice by four of five metric recipes — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0008.md`
- A `jq` function parameter (`$name`) was evaluated once against the caller's input rather than per row, silently reporting zero coverage — `sdlc/records/0011-the-live-probe.md`
- `select($old | has(.))` parsed and ran but was wrong, only failing at run time on real data — `sdlc/records/0008-metric-recipes-over-live-decide-rows.md`
- A demo page printed a listing of `triage.sh` with no block asserting it, so it could rot silently — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0010.md`
- A block under `set +e` let an earlier failure inside it pass unnoticed — `sdlc/issues/closed/2026-09-19-review-leftovers-from-ticket-0010.md`
- `probes/replay-check.sh` rotted silently; no rung ran it, so a schema drift went undetected until run by hand — `sdlc/issues/closed/2026-09-19-replay-check-fails-on-meta-tool.md`
- `specification/choose.md` stated "Reversing the option order changed none of fifty picks," disproven at a larger sample — `sdlc/planning/adr/0014-what-the-live-probe-changed.md`
- `specification/score.md` and `rank.md` called rating "the weakest thing a decider model does," disproven by measurement — `sdlc/planning/adr/0014-what-the-live-probe-changed.md`
- A base URL carrying a password would print on success and get written into a committed recording and plan — `sdlc/records/0006-the-two-variables-the-live-script-and-the-first-green-demo.md`
- A base URL's trailing whitespace/newline (from `$(cat file)` or a `.env` line) was not trimmed before composing the address — `sdlc/records/0006-the-two-variables-the-live-script-and-the-first-green-demo.md`
- Scheme matching was case-sensitive, wrongly refusing `HTTP://host/v1` — `sdlc/records/0006-the-two-variables-the-live-script-and-the-first-green-demo.md`
- `unwrap_used` silently misses an unwrap whose error type is `Infallible` — `sdlc/issues/closed/2026-09-18-two-lint-rules-do-not-hold-as-written.md`
- `allow_attributes_without_reason` could not be `forbid`den workspace-wide as the standards claimed; clap's derive macro breaks under `forbid` — `sdlc/issues/closed/2026-09-18-two-lint-rules-do-not-hold-as-written.md`
- Demo pages used `--input FILE`, an unbuilt option, on every line, so a "green-in-spirit" page actually failed to parse — `sdlc/records/0005-reshape-decide-to-the-flat-surface.md`

## 2. Patterns

1. **Tests that prove less than they claim** (5): weak substring assertions, "before any request" claims proved only under `--dry-run`, wrong numbers reported in a record. Clearest: `assert!(choose.contains("3"))`.
2. **Checks that nothing runs, so they rot** (3): `replay-check.sh` unwired from any rung, `set +e` swallowing failures, the demos-script guard silently skipped on an empty name. Clearest: `probes/replay-check.sh` failing for a ticket cycle before anyone noticed.
3. **Secrecy/privacy proofs that cover one path** (6): one-verb-only secrecy test, key/body leaking through derived `Debug`, world-readable recordings, redirect crossing hosts, a leftover temp file, a password in a URL reaching a recording. Clearest: the `Debug` derive printing the key and the request body.
4. **Pages that drift from the binary** (6): stale `AGENTS.md` sentence, too-narrow word sweep, two disproven specification claims, demo pages built against an option that didn't exist yet, a record with wrong measured numbers. Clearest: `choose.md`'s "changed none of fifty picks," contradicted at a larger sample.
5. **Shell and `jq` traps** (4): `//` conflating `false` and missing, duplicate ids double-counted, a `jq` closure parameter evaluated once, `has()` misused on the wrong operand. Clearest: `//` turning an unresolved answer into a silent "no."
6. **A gate depending on the caller's environment** (2): `THINKTHEN_LIVE_LEDGER` letting a caller redirect the spend limit, two lint rules asserted absolute in `rust-standards.md` that didn't hold once real code (clap's derive) touched them.

## 3. Remedies

1. **Weak tests** — (a) mechanical: assert request counts on the loopback listener instead of `--dry-run`; pin exact strings, not substrings. Partly covered already (`harness::spawn` fixed the duplication half); the assertion discipline itself has no check and needs the written rule now in the proposed `AGENTS.md`.
2. **Rotting checks** — (a) mechanical: every check script under `sdlc/scripts/` or `probes/` runs from a rung (`spec` now does this for `replay-check.sh`). Already covered for that one script; the general rule is new and belongs in `AGENTS.md`.
3. **Narrow secrecy proofs** — (b) test helper: one shared "no secret in any output or `Debug` line" fixture exercised across every verb, both success and failure. Some of this landed as code fixes; the *rule* that a secrecy test must cover every verb is new and belongs in `AGENTS.md`.
4. **Pages that drift** — (c) rule: a page's measured claim names the record that measured it, and a behavior-changing ticket updates the page in the same commit. No mechanical check catches a false English sentence; this is a written rule.
5. **Shell/`jq` traps** — (c) rule, since jq offers no linter here: never use `//` for a three-way branch. This is now written into `AGENTS.md`; further traps are already logged in the "ideas carried" issue as craft knowledge, not universal rules.
6. **Environment leaking into a gate** — (a)/(d) mixed: the ledger bug is already fixed (script reads a fixed path, tests copy the script). The lint-table gap is already corrected in `rust-standards.md`'s "Not yet enforced" note; no further rule needed, listed as covered.

## 4. Proposed `AGENTS.md`

```markdown
# Agent instructions for thinkthen

The binary is `thinkthen`. The crates are `thinkthen-core` and `thinkthen`. Read `README.md` first, then `specification/README.md`, then `sdlc/planning/rust-standards.md`. The specification is the contract, and code follows it.

## Building

- Red-green test-driven development. Write the failing test, watch it fail for the stated reason, make it pass, then clean up.
- Build the simplest thing that works. YAGNI, DRY, locality of behavior, separation of concerns. A command or an option enters only when a demo cannot be written without it.
- The gate ladder is `sdlc/scripts/{install,lint,test,spec}`. Run the cheapest rung first and the whole ladder before handing back.
- `sdlc/ratchet.json` holds the source size ceiling, equal to the measured total. The commit that raises it says what grew, why it earns its lines, and where you looked for duplication to delete first.
- A second agent reviews any change that raises the ceiling, widens a public surface, or adds a dependency, and names what it checked.
- A ticket that turns a demo green writes the page in the how-to form of ADR 0011. `sdlc/scripts/demos` checks it.
- Commit as soon as a change is whole and push right away. Commit messages are imperative and active.
- Never add agent attribution to a commit or a pull request: no trailer, no co-author line, no "generated with".

## The pure core

`thinkthen-core` touches no file, no environment variable, no socket, no clock, and no process. Its `clippy.toml` bans them. The binary parses at the edge and hands typed values inward. Do not weaken either lint table; `lint` compares them against the accepted copies.

## The tool judges and never acts

`thinkthen` never runs a command, never treats free text as an instruction, and never writes a file the user did not name. A ticket that asks for any of those is wrong and stops.

## No network in a gate

Tests replay recorded responses. A live call to a paid backend runs only from `sdlc/scripts/live`, by hand, under a token cap, with Ian's authorization.

## Credentials

A key is read from `THINKTHEN_API_KEY`. It is never committed, logged, hashed, echoed in a plan, or written to a recording. It goes only to the address the user named. A recording stores request bodies and responses and never headers.

## Public hygiene

This repository will go public. Never name a private project or a customer. Describe a consumer generically.

## What reviewers keep finding

- A secrecy test covers every verb and every failure path, not one command, and checks every `Debug` line too.
- A test that claims "sends nothing" asserts the request count on the loopback listener. `--dry-run` proves nothing about a live path.
- Pin the exact sentence a test checks. `contains("3")` passes on any text with a 3 in it.
- A check script strips fenced code blocks before it reads a title or a status line out of prose.
- A block under `set -e` off pins the exit code it captured, or an earlier failure inside it passes unseen.
- A `jq` transform never uses `//` for a three-way rule; `false` and a missing value read alike under it.
- A page's measured number names the record that measured it.

## Where things are

`crates/thinkthen-core` and `crates/thinkthen` hold the code. `specification/` is the contract. `spec/*.md` are executable pages, run by `mustmatch`. `demos/` are how-tos in the form of ADR 0011: a task, a trap, and a test at once. `transforms/` holds `jq` recipes over recorded rows. `probes/` holds the live measurements behind a ruling. `sdlc/` is the record: `planning/adr/` decisions, `tickets/` authorized work, `records/` what landed and its review, `issues/` problems found, `scripts/` the gate ladder.

## Where decisions go

Every decision lands in `sdlc/`: an ADR, an issue, or a ticket. Tickets are numbered from 0001, and a ticket lands through a worktree. A decision Ian cannot find later was not made. Say which ones he can overturn.
```

(3,956 characters, under the 4,000 limit.)

## 5. What `rust-standards.md` should gain or lose

- **Gain**: a line noting `allow_attributes_without_reason` is denied at the workspace and forbidden only in the pure crate, since `forbid` workspace-wide breaks under clap's derive macro (issue 2026-09-18).
- **Gain**: a line noting `unwrap_used` does not fire on an `Infallible` error type, so "denies `unwrap`" is a risk-ban, not a token-ban.
- **Gain**: under "Not yet enforced," add that no tool runs the repository's own check scripts (`replay-check.sh` and similar) from a rung; each one needs its own wiring or it rots quietly.
- **Gain**: a line that a secrecy or privacy claim needs a test over every verb and both the success and failure path, not one representative case.
- **Lose**: nothing factual needs removing; both corrections above are additions to already-accurate text, not replacements.
- **Consider**: since `thinkthen` now ships `transforms/` (`jq`) as load-bearing, first-class artifacts, `rust-standards.md` (a Rust-only document) is the wrong home for the `jq` lessons — they belong in `AGENTS.md` or a `transforms/README.md`, which is where the proposed `AGENTS.md` above puts the one universal rule (never use `//` for a three-way branch).

## 2026-10-04: Audit host contracts separately from the common engine

0405 found that typed score descriptions and relation reads already existed despite an older product note. Trace the accepted parser and the complete executable path before turning a premise into a ticket. Keep metadata digests separate from request/cache identity. Record exact selected-case and packed-file proof scope; a published file count is not an individual test count. Name host adapters, generic JSON versus typed routes and changed-rule strict-replay proof separately. Recognition can introduce previously unasked downstream relations when its entity cut changes. Bound display snapshots in memory; a read-only audit does not authorize implicit spool files.


## 2026-10-04: Bound native setup and stop failed joins before teardown

0381 A's fresh High review rejected unbounded native build tools despite bounded consumer lifetimes, and found that recording join failure let a fixture read/free a live worker's state. Inspect the entire build/stage/inspection path when asserting deadlines. Drain outputs without blocking deadline enforcement; cleanup targets only owned handles or an owned PID tree. A failed join must terminate before shared-state access and teardown. Test the failure with a real held worker and an independent cleanup bomb. Portable runner/shared-fixture proof does not establish native Windows thread, compiler, loader or process-tree behavior.
## 2026-10-04: Count recognition chunks before choosing a held width

0400 C's historical recognition fixture claimed fourteen chunks but its seven-word text now produces seven BILOU questions. A default-eight rendezvous could never fill. Repeating that phrase on one input line supplies fourteen actual chunks while retaining the original canned reply. Keep scalar omission and explicit-four record mode as distinct guards; the record-mode jobs refusal stays unchanged. Reuse the owned signal helper through a bounded child module rather than exposing a test API. Record arrival counts on failures, and verify the plan's chunk count separately from actual sends. Source review, final integration, full checkpoints and SQLite stress proof remain pending.


Extracted setup helpers do not receive Clippy's test-function exception for `expect()`. Return `io::Result` and let the boundary test assert setup success. 0400 C's first frozen lint rejection and its renewed independent scalar/record proof remain in the build record.
