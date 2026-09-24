# Spine review: engine controls and test infrastructure

Reviewer: fresh read-only Claude session, 2026-09-24. Read on main at `ab72203c`: workspace `CLAUDE.md` Tickets section, repo `CLAUDE.md`, `one-line-plan-2026-09-24.md`, `surfaces-port-guide.md`, `specification/{find,result,question-file,recognize}.md`, ADR 0017 section 6, `conformance/cases.json` and its README, the conformance runner under `src/cli/conformance_tests`, `tests/backend/harness/mod.rs`, `engine/{mod,http,request}.rs`, and the ticket branches for 0076, 0077, 0078, 0089, 0091, 0092, 0093 (ADR 0047), and 0095. Branch source: tag `surfaces-wave7-final` `conformance/conformance.json` and `standin/`. Nothing was built, run, or edited.

## Verdicts

- **0076 whole-call deadlines (9ddf5ecf): ACCEPT with three small fixes.** The design is still correct on main. The fixes cover routing, 0089 as a prerequisite, and one shared runner file.
- **0077 one process width cap (7fe9adc6): findings.** One acceptance bullet is missing (a deadline while waiting for width). One bullet belongs to 0078. The `opens` list misses the recognize and relate paths.
- **0078 fork recovery and host signals (01d82781): findings, blocking.** As written, the fork proofs cannot be built: the workspace forbids `unsafe`, and `fork()` needs it. The pools the design rebuilds do not exist until 0085. The R6-3 test accepts two outcomes, so it cannot fail. Split the ticket.
- **0091 conformance union (b6997dcd): findings.** The word rulings match ADR 0017 and the specification. The case mechanics have six gaps: duplicates, runner arms, IDs, the find fix, digests, and the size.
- **0092 loopback backend (788d3758): findings.** The design is sound and adds no product code. It misses request-identity digests, the deadline arm cannot run through the command, and it must follow 0089.

## 0076: findings

1. **The routing is stale.** The `Selected route` section sends the work to Luna Max with Sol reviews under the three-ticket trial. The one-line plan makes Claude the owner of the queue. The workspace rule says "each agent reviews with its own models." Smallest change: rewrite the routing line to Opus builds and a fresh Claude reviewer, unless Ian routes Codex.
2. **0089 is a prerequisite that the ticket does not name.** Both tickets write `engine/http.rs`, and 0089 changes `is_retried` and the close-before-reply message. The deadline rule also depends on it: a timeout that comes from the deadline after the body left must return `Deadline` with one send and never retry. Smallest change: add 0089 to Dependencies and add "no second connection" to the held-send bullet. The bullet already counts one send.
3. **One file is shared with the case runner.** Giving `Error::Deadline` its budget changes `inject(Injection::Deadline)` in `engine/request.rs`, and `src/cli/conformance_tests/runner.rs` matches on it. 0091 edits that runner. Name both files in `opens`.
4. **Guard against a test hook in product code (follow-up, no ticket change needed).** The command has no deadline option. Every command-path proof (recognize, relate, annotate, find) must therefore be an in-crate unit test that calls the private engine. Main already has an in-crate `TcpListener` in `engine/http.rs`. Do not add a `THINKTHEN_TEST_DEADLINE` environment hook. Main already carries five `THINKTHEN_TEST_*` hooks.
5. **The R6-4 amendment is sound.** Store the budget as a `Duration` and format it with integers only. Say which private `Display` the "prints exactly" test pins. Also say that an instant that cannot be represented (`checked_add` returns `None`) means no deadline, never a panic. 0095 owns the public refusal of large budgets.
6. **Sizes.** The limits of 12 production files and 500 lines are realistic. 750 test lines is tight for this matrix: 7 paths spent before the call, 2 schedulers, 3 lock waits, retry, held send, and the barrier race. For comparison, 0089's narrower `resend.rs` alone is 210 lines. Smallest change: raise the test cap to 1,000 lines, or make the spent-deadline row one table test at the prepared-request choke point with one case per scheduler.
7. **Follow-up for 0095.** 0095's interrupt check runs "at every existing 50 ms poll." Keep the deadline and cancel poll in one function so 0095 hooks in at one place.

## 0077: findings

1. **One proof is missing.** No bullet covers a spent deadline while waiting for width. 0076 hands this proof to 0077 by name. Smallest change: add one bullet. A waiter held behind a full gate whose deadline expires returns `Deadline` and sends nothing, the listener's count stays frozen, and no permit is used up.
2. **One bullet belongs to 0078.** "PID inspection precedes access to every replaceable width wait state" cannot be built before 0078 exists. Smallest change: 0077 routes every access through one accessor function, and a test pins that accessor as the only door. 0078 then guards the accessor.
3. **The `opens` list is incomplete.** `cli/recognize.rs`, `cli/relate.rs`, and `cli/relate/config.rs` build clients or workers today, and the width plumbing must reach them. Add them, or open `crates/thinkthen/src/cli` whole as 0076 does. With them added, 10 production files is likely too few. Allow 12.
4. **Stale text.** The ticket still cites the old queue order and says not to begin before 0087, which has landed. Its routing still reads "gpt-5.6-sol." Cite the one-line plan and route to Claude, as in 0076 finding 1.
5. **Still correct against main.** `jobs` is already `Option<u8>` at `cli/args.rs:162`, so keeping explicit and omitted width apart costs little. After 0089 the gate wraps status retries only. The ADR 0047 citation on 0093 matches item 5: one cap per loaded copy.

## 0078: findings (blocking; this design was never reviewed)

1. **The fork proofs cannot be built.** `unsafe_code = "forbid"` is a workspace lint (`Cargo.toml:20`), and the crate roots add `#![forbid(unsafe_code)]`. `nix::unistd::fork` is `unsafe`, and no crate test forks today. Every warm-parent, busy-parent, child-race, cache, counter, and queue proof needs `fork()` inside a process that holds private engine state. An outside crate cannot reach private code. The repo `CLAUDE.md` bars weakening the lint table. Smallest change: add a private PID-source seam (a `cfg(test)` fake PID). With it, red-first unit tests hold the inherited mutex, rebuilding marker, and gate in another thread, fake a PID change, and prove that replacement never touches them. If the code locks first, the test hangs and the watchdog fails it. Move the real-fork proofs to 0086's external consumer crate, which gets its own reviewed allowance for one fork call, or to the Python surface's `os.fork` check. Record that choice in the ticket.
2. **The pools the design rebuilds do not exist before 0085.** Main builds one `http::Client` per command path per call (`cli/asking.rs:384`, `find.rs:104`, `annotate.rs:92`). No engine value holds a pool across calls. The only process statics are `RECORDING_SIGNAL` and the CLI `STATE`. The "warm parent keeps the engine value alive" proof has no value to keep. Smallest change: split the ticket.
   - **0078a, host signals, now.** Move `SIGXFSZ` to the command edge, make `signal-hook` CLI-only, and settle R6-3. This part is small and correct as written.
   - **0078b, fork recovery, after 0085.** 0085 builds its retained state under the replaceable slot from the start. 0078b lands before 0086.
   Record the plan change in `one-line-plan-2026-09-24.md`.
3. **The R6-3 test cannot fail.** It accepts either "normal answer" or "pin the interruption and file an issue." It also cannot go red on main: `Cancelled` comes only from the token, so "never returns Cancelled" holds by construction. The live risk is a different one. A socket with a timeout returns EINTR even under `SA_RESTART` (signal(7)). After 0089, EINTR becomes `Transport(Other)`, which fails the call with exit 4 and is never resent. Smallest change:
   - Pin one expected outcome: the normal answer and exactly one send on the listener.
   - Install the handler with the safe `signal_hook::flag::register`.
   - Deliver the signal with nix's safe `pthread_kill` to a worker thread ID published by a `cfg(test)` seam.
   - If the test goes red, fix it here: engine-created worker threads block asynchronous signals, as the CLI already does for SIGINT with `thread_swap_mask`.
   The host `SIGXFSZ` harness also needs `setrlimit` in a child process. Reuse the existing `ulimit -f 1` subprocess pattern in `recording_durability.rs:188`.
4. **Stale facts.** "Current facts" still says post-0074. Rewrite them against the tree after 0088 and 0089. Route to Claude, not `sol-implementer`.
5. **Sizes.** 0078a is small: about 4 files and 150 lines. 0078b's limits of 14 files, 650 production lines, and 950 test lines are plausible once the real-fork proofs move out.

## 0091: findings

Word rulings, checked against main:

- **"unsure" in cases.** Consistent. ADR 0017 section 6 item 4 makes "unsure" the public word and keeps "unresolved" in the specification's grammar. Main's case IDs already say "unsure" (03, 07), with bare `null`.
- **find with no pick is `null`.** Consistent with `find.md:33,41` and `result.md:20`. The claimed contradiction in main's cases is not real. In `answers[]`, each entry is the wire-level choice decode, so `bare: "none"` and `kind: "choice"` are correct there. `operation.selected` is already `null`. Smallest change: do not edit case 19. Add one README sentence: for `find`, `answers[]` is the wire choice, and `operation` holds the find value. Editing `answers[]` would turn the runner red for no reason.
- **Broken JSON text is `usage`, a broken named file is `local`.** Consistent with `question-file.md:33` for files, and with 0084 and 0095 (`from_json` gives Usage, `load` gives Local). The specification does not state the JSON-text half, and the specification is the contract. Smallest change: add one sentence to `specification/question-file.md` and put that file in `opens` (`find.md` needs no change). Branch cases 09 (filter with a band) and 22 (rank with a threshold) are verb mismatches. Those are `usage` for a file too, so they do not test the split. Keep 08 and 10 as the pair.

Mechanics:

1. **Duplicates.** Cases 18 and 27 "become main's injections." Main already has `23-cancelled-fault` and `24-deadline-fault`, so 27 is a duplicate. Branch 18 is a mid-batch cancel that keeps finished results, and an injection cannot express that. The ticket also leaves 18 to the surfaces. Smallest change: drop 27 and leave 18 to the surfaces only.
2. **Runner arms.** The command runner hands the private engine questions parsed from the case. It has no file-versus-text input, no `usage` or counter verb, and no recognize, relate, or decide-many arms. The ticket must name:
   - one `question_form: text|file` field for the usage/local pair;
   - where case 17's counter difference runs (it needs a cache folder, so it is not a pure-core case);
   - the three new success arms.
   Without these, "the command runner passes on the grown file" cannot hold.
3. **Case IDs.** Branch 06, 08, 09, 10, 19, 22, and 23 collide with main's IDs, and main already repeats 17 and 18. Smallest change: new cases continue at 26 onward and record their branch ID in `provenance`.
4. **Recognize.**
   - "Refresh main's recognize fixture digests" is backwards. Main's 40 replay files pass today (`all_forty_harvest_cases_replay_without_a_key_or_network`). The branch digests are the ones that fail to match.
   - Main's fixture uses `MISC` as a real kind, and the branch uses `other`. Map branch→main, not the other way.
   - The main test counts 10 divergences, and the ticket names 5.
   - Main's fixture is a command replay folder, but the plan wants one file for 0085 and the surfaces.
   Smallest change: recognize stays in the fixture folder, and `cases.json` gains only 68 (the offsets case) plus the 9 relation cases as `synthetic_contract`.
5. **Relate 69–71.**
   - Building them from main's planner bytes is the right call. The ticket should also say that the entities gain concrete kinds (method H used `*` over bare text).
   - The expected edges follow from the stated synthetic probabilities, not from the branch's answers. Case 36-C09 splits "Karst and Vellum" under main's rule.
   - Case 70, with 24 records, adds bulk and no new contract. Cut it to one same-kind case (69) and one cross-kind case (71).
   - The dependency line should say that 0088 landed at `71841025` and that the target-side issue is closed.
6. **Size and shared files.**
   - Three new runner arms plus a schema check will not fit in 400 lines. Allow 650, or move the relate arm to 0085.
   - Shared files: `conformance/README.md` with 0090 (its "calibration" wording), `src/cli/conformance_tests/runner.rs` with 0076, and `specification/` with 0082.
   - Narrow `opens` from `crates/thinkthen/tests` to `tests/fixtures/recognize-225` and `tests/backend/recognize.rs`.
7. **0095 citation.** The one-question `annotate` cases cite 0095 as "the ruled bulk form," but 0095 is an unreviewed draft. Cite ADR 0017 or mark the cases as pending 0095's review.

## 0092: findings

The design is sound and minimal. It adds no second engine: it serves replies and never plans or judges. The `thinkthen` crate gets no product change, which meets the repo rule. The findings:

1. **Request digests break.** `details.requests` digests hash the URL (`Exchange::new(backend.url(), bytes)`). Every case is recorded against `https://api.typesafe.ai`. With `base_url` pointed at loopback, every success case fails its digest check. Smallest change: the runner recomputes identities for the served URL, the way `tests/backend/recognize.rs:338` swaps in `$URL`, and says so in the acceptance.
2. **Choosing a fault arm.** "Per connection by case id" gives no way for a client to name the case. Smallest change: choose the arm by URL path prefix. The engine appends `/systemone` to any base (`core/backend.rs:289`), so `http://127.0.0.1:P/arm/reset/v1` works for every binding with no header.
3. **Some arms cannot run through the command.**
   - The command has no deadline option, so the deadline arm can run only through the private engine or through 0086.
   - The six injection cases (20–25) have no wire form except backend.
   Smallest change: the acceptance lists which cases run on loopback (success cases plus wire faults) and moves the deadline arm's proof to 0086.
4. **Order and shared files.**
   - 0089 adds 97 lines to `harness/mod.rs`, and 49 test files import the harness. 0092 must follow 0089.
   - Keep a one-line `mod harness { pub use …; }` shim so that 49 files do not churn.
   - The root `Cargo.toml` gains a second workspace member. That needs ADR 0047 (0093) accepted, or a one-line ruling in 0092 that a test-only unpublished member is allowed. The member must also come under policy, deny, and ratchet.
5. **Two gaps (follow-ups).**
   - The count arrives only on exit. A test that must prove nothing was sent before a phase needs a `count` line on standard input.
   - Branch suites that ran arbitrary questions on the null backend (`every_verb_answers_on_null`, the 3-million-record batch) lose coverage. An exact-body server 5xxs on every body it does not know. Each surface ticket must rewrite those suites onto cases, or 0092 adds one generic arm that answers any well-formed request with fixed probabilities. The test that "proves it opens no socket beyond loopback" is better stated as "binds 127.0.0.1 only." The binary has no client code.
6. **Size.** 450 net lines is realistic once the harness has moved.

## Build order and parallel lanes

Shared files that force serial order:

- `engine/http.rs`: 0089, 0076, 0077.
- `engine/{request,schedule,annotate_schedule,workers}.rs`: 0076, 0077, 0078b.
- `engine/recorder.rs`: 0076 (folder waits), 0078a.
- `cli/{asking,annotate,find,recognize,relate}.rs`: 0082, 0083, 0076, 0077.
- `crates/thinkthen/Cargo.toml`: 0078a, 0092.
- `tests/backend/harness/mod.rs`: 0089, 0092, and the tests of 0076 and 0077.
- `src/cli/conformance_tests/runner.rs`: 0076, 0091.
- `conformance/README.md`: 0090, 0091.
- `specification/`: 0082, 0091.
- `sdlc/scripts/policy.py`: 0077, 0078a, 0092.
- `sdlc/ratchet.json`: every code ticket. Re-measure on each rebase.

Lanes:

- **Lane A (engine controls, serial):** 0089 (building) → 0082 → 0083 → 0076 → 0077 → 0078a → (0084, 0095) → 0085 → 0078b → 0086.
  - 0078a may run beside 0077 once 0076 lands. Their write sets are disjoint except `policy.py` and `ratchet.json`.
- **Lane B (cases):** 0090 → 0091. It runs beside Lane A from now on, and it must land before 0085.
  - Merge conflicts in `specification/` and `conformance/README.md` are rebase-level. Land 0091 after 0082, or rebase it then.
- **Lane C (test backend):** 0092 after 0089 and 0091. Land it before 0076 starts, so that the tests of 0076–0078 are written against the moved harness once. Otherwise land it after 0078a.

Overturnable by Ian or the queue owner: the 0078 split and the move of 0078b after 0085 (a plan change), where the real-fork proof lives (0086's crate or the Python surface), and 0092's generic arm.
