# Confirmation, 2026-09-24

Checked read-only. For each ticket I diffed the reviewed commit against its revision, and I checked ADR 0041 at tag `surfaces-wave7-final`, the 0085 ticket at `d5be12cc`, and the port guide on `origin/main` at `fdb7c535`.

- ACCEPT 0077 (083bb5e6): both blocking findings are resolved. The `Width` type covers 1 through 32, and 0 and 33 fail as `Usage`. A new R2-9 bullet adds the two-engine test. Each row has a planted bug. All four follow-ups are resolved: the stale text, the `policy.py` rule with a planted failure, the in-crate command proofs, and the 1,000-line test cap with 0092 as a prerequisite. Nothing new broke.
- ACCEPT 0078 (fc9a8712): all three blocking findings are resolved. `nix` changes to a non-optional Unix dependency, with `Cargo.lock` and `deny.toml` in scope and a second dependency reviewer. The promise now covers worker threads only, and 0085 `d5be12cc` sends every live attempt on a worker with a planted-bug thread-ID test. R5-3 moved to 0096 in the ticket and in the port guide on main. The follow-ups are resolved: the retry-wait phase is dropped, `SIGXFSZ` is excluded from the mask, the default-action child is killed, the child proofs run in the all-features binary, and `cargo tree` runs in `lint`. Nothing new broke.
- ACCEPT 0092 (71cd757e): the crate has a library target and a thin binary. `spawn` and `process_has_file` stay in the harness, with a reviewed path dev-dependency. The loopback cases are named, and the injection cases 20 through 25 stay in-process. The unknown-body test has a planted fall-through. Nothing new broke.
- ACCEPT 0091 (7af36977): the counter step for case 17 is added. The pending mark sits in `provenance` only, and each schema refusal has a planted bad case.
- ACCEPT 0099 (c0dfef20): the 0041 amendment records the upper bound above 4,294,967,295 seconds, conditional on 0095. That matches the tag's own 2026-09-23 amendment of ADR 0041, which supersedes the no-deadline sentence.

# Re-review: 0077, 0078, 0091, 0092, 0099

Reviewer: fresh read-only Claude session, 2026-09-24. Read: workspace `CLAUDE.md` Tickets section, repo `CLAUDE.md`, `one-line-plan-2026-09-24.md` and `surfaces-port-guide.md` on main `860086d2`, the prior review `spine-review-controls.md`, the error index rows R2-9, R4-12, R5-3, R6-3, and on main: `Cargo.toml` (root and crate), `tests/backend/harness/mod.rs`, `core/reply.rs`, `cli/interrupt.rs`, `sdlc/scripts/{test,lint}`, engine thread spawn sites. Also ureq 3.4.2 source in the cargo registry and tag `surfaces-wave7-final` ADRs 0041–0046. Nothing was built, run, or edited.

## Verdicts

- REJECT 0077 (912ee3ce): all five prior findings are resolved. Two engine-row proofs are incomplete: the width half of R4-12 is missing, and the R2-9 planted bug points at a test no bullet defines.
- REJECT 0078 (c5895e17): the split and the R6-3 test answer the prior findings. The worker mask needs `nix` in the library build, which contradicts "no new dependency". The outcome still fails for calls that run on the host's calling thread. R5-3 is orphaned by the split.
- ACCEPT 0091 (fcf10b58): all seven prior findings are resolved. Make one in-place edit (finding 0091-1) before the build starts. It needs no re-review.
- REJECT 0092 (c8fd2de5): the digests, arm choice, command-runnable arms, order, count line, and bind rule are resolved. The harness cannot move whole into a binary crate as written.
- ACCEPT 0099 (f420a2d8): not in the prior review. It is sound. Make one in-place edit (finding 0099-1) before the build starts.

## 0077

1. **The width half of R4-12 is missing (blocking).** The port guide says 0077 carries R2-9 and the width half of R4-12. The ticket names only R2-9. Width 0 in a permit gate blocks every caller forever, and the private `Option<Width>` seam is the door 0084–0086 consume. Smallest change: add one bullet. The seam's `Width` type exists only for 1 through 32. Width 0 and width 33 fail as `Usage` before any request. The planted bug accepts 0, and the red test shows the refusal missing.
2. **The R2-9 planted bug names a test that no bullet defines (blocking).** "Shows the two-engine test turning red" has no matching acceptance bullet. The loopback bullet mixes paths, and the state bullet checks registration only. Smallest change: add one bullet. Two engines that differ in a setting other than width (for example the model or the cache folder) send at once through the held-reply listener, and the listener's peak equals the one cap. A per-settings gate doubles that peak.
3. Follow-up (non-blocking): two stale sentences remain. "that 0078 cannot replace" should read 0096. In Excluded, "Do not change 0087's implementation position … before 0087 lands" should be deleted, because 0087 has landed.
4. Follow-up (non-blocking): a runtime test cannot pin "one accessor is the only door." Make that a `policy.py` rule with a planted failure, the same way the `ureq` ban works.
5. Follow-up (non-blocking): after 0077, the command's width activation is not visible from outside its process. The two `--jobs` activation proofs must be in-crate tests of the command setup function. Say so.
6. Follow-up (non-blocking): 750 test lines is tight for this matrix. Prior finding 0076-6 said the same about a smaller matrix. Allow 1,000 lines. Name 0092 as a test prerequisite, since the plan puts the moved harness before the control tickets' tests.

## 0078

1. **The worker mask needs a library dependency (blocking).** `nix` is `optional` and enabled only by `cli` (`crates/thinkthen/Cargo.toml`). Blocking signals in a library build without `unsafe` needs `nix` (`thread_swap_mask`) as a normal Unix dependency. That contradicts "No new dependency," and the repo rule sends a dependency change to a second reviewer. Smallest change: say that `nix` (feature `signal`) becomes a non-optional Unix dependency while `signal-hook` leaves the library, and put the `cargo tree` check on both.
2. **The outcome fails on the calling thread (blocking).** Direct judgments and `find` run on the host's calling thread (0077's own current facts). The design keeps the host's mask on that thread. The ureq 3.4.2 source has no `Interrupted` handling (grep: zero hits), so a host signal during a timed socket read still fails a single call as `Transport(Other)`. That is R6-3 again, and Python's main thread is the common case. Smallest change: add a calling-thread row to the R6-3 test, then pick one of two fixes and record the choice:
   - (a) Run each live attempt on a masked engine thread at 0077's one send boundary.
   - (b) Narrow the outcome to engine-created threads, and file an issue that 0085 or 0097 owns.
3. **R5-3 is orphaned (blocking but small).** The port guide assigns R5-3 (a lock taken on a child's first call after fork) to 0078. After the split it belongs to 0096. Add one line to the ticket and one to the port guide's "Engine rows by spine ticket."
4. Follow-up (non-blocking): the "during a retry wait" phase cannot go red, because `thread::sleep` resumes after EINTR. Keep the held send as the planted-bug proof. Pin the default-action child's outcome: killed by SIGXFSZ, or EFBIG if SIGXFSZ is in the worker mask. Say whether the mask includes SIGXFSZ.
5. Follow-up (non-blocking): every rung runs `--all-features`, so no rung builds "a library build without `cli`." Run that child from the all-features test binary without CLI setup, and put the `cargo tree --no-default-features` check in `lint`. A check that no rung runs rots.

## 0091

1. **Fix in place:** "Runner arms … Nothing else" contradicts the case-17 bullet. Case 17 needs a counter-difference step around the call. Add "and one counter step for case 17."
2. Follow-up (non-blocking): say that cases marked pending on 0095 still run and must pass, and that the mark sits in `provenance` only. Otherwise the branch skip table this ticket refuses comes back.
3. Follow-up (non-blocking): each of the three schema refusals needs a planted bad case that turns its test red.

## 0092

1. **The harness cannot move whole (blocking).** `harness/mod.rs:449` spawns `env!("CARGO_BIN_EXE_thinkthen")`, which only the `thinkthen` package's own integration tests can see. Its items are `pub(crate)`. A binary crate cannot be the target of a `pub use` shim. Smallest change:
   - `conformance/backend` gets a library target (the listener and canned replies, made `pub`) plus a thin binary.
   - `spawn` and `process_has_file` stay in `tests/backend/harness`, which re-exports the rest.
   - `thinkthen` gains a path dev-dependency on the new crate, reviewed under the repo's dependency rule. The 0086 review's F5 already expects that dev-dependency.
   - The 49 importers still do not churn.
2. **Name the loopback cases (blocking but small).** The acceptance says "The acceptance lists which cases run on loopback" and lists none. Name them: every success case and each wire-fault ID. The injection cases 20–25 stay in-process.
3. Follow-up (non-blocking): a test that the unknown-body 5xx fires turns red if the case arm falls through to the generic arm. Keep the two arms on separate paths and test that.

## 0099

1. **Fix in place:** ADR 0041 says "an unrepresentable budget is treated as no deadline." The 0095 draft refuses a budget above 4,294,967,295 seconds as `Usage`. The port amendment must record that change at the public door, conditional on 0095's acceptance, so the ported ADR does not contradict its new owner.
2. Follow-up (non-blocking): the status line says "revised after design review," but its Review section says the review is pending. Fix the line.

## Lanes and dependencies

Lanes A, B, and C match the plan. 0077 and 0078 follow 0076. 0078 builds beside 0077. 0091 follows 0090 and lands before 0085 and 0092. 0092 follows 0089 (landed) and 0091. 0099 can land any time before 0086. None of the five tickets names yellow, and the gates run on this machine.

Ian or the queue owner can overturn the choice between fix (a) and fix (b) in 0078 finding 2, and the width-type range in 0077 finding 1.
