## Review — experiments lens: 211 (+ 207 jobs 2–3, + 225)

Snapshot: 211 at `0801a32` worktree state read as-is, 207 and 225 current on disk, date 2026-09-21, nothing running. Method: **read-only and static only — I have no execution tool in this role, so I reran nothing.** Every check below is source, notes, lock-file, `/proc` liveness, or build-artifact evidence. The exact commands to run are listed at the end.

## Verdict

The run is honest and its headline numbers trace to command output in the lane notes: answers 1–3 hold, the null/width/fork benches are internally consistent with the code, foyer and the cache arms are fairly stated (version pinned, capacity stated, fork smoke real), the PG and Python fork evidence is what it claims, and job 3's divergence table (12/3/5) is arithmetically exact against `conformance.json` and the runner code. Two things do not hold: **answer 4's "poll every tick" is not what the code does** (a fast backend starves the callback entirely — P1), and the "by construction" fork claim is sound for locks but leaves an unstated OnceLock precondition and an fd leak. Several smaller claims are inference stated as fact, and job 3's consolidation broke the 207 stand-in's own case runner. Accept the report with fixes; do not copy the interrupt shape into ADR 0017 unchanged.

## Findings, by consequence

**F1 — P1, likely defect (code-evident, not run): the poll callback (and the calling-thread cancel/deadline check) runs only when the wait *times out*, so a fast backend starves it — the exact deafness 205 recorded.**
Violated promise: FINDINGS answer 4, "each tick (50–100 ms) it checks the cancel token … and runs the callback", and the shim doc "raises `KeyboardInterrupt` … worst case one tick plus one in-flight request". Evidence: only poll call site is the `RecvTimeoutError::Timeout` arm, `engine/src/lib.rs:548-563` (TICK at `:75`); `recv_timeout` returns immediately while messages are queued, so when the channel never idles (`ENGINE_NULL=1`, or any backend faster than ~width×TICK per record, ≈1.5 ms/record at width 32) the loop consumes answers and never polls; the Python shim's only `check_signals` call is that poll (`python/src/lib.rs:254-263`), and the GIL is detached for the whole batch (`:251`), so no other path can raise. The interrupt tests only exercise the 300 ms stub (`python/tests/test_sigint.py:6-9`), so the gap is outside the tested lane. Repro (supervisor): 1,000,000 records, `ENGINE_NULL=1`, SIGINT one second in — expect `KeyboardInterrupt` inside ~0.4 s and (I predict) it returns only when the batch drains. Smallest fix: in the `Ok` arm, run the poll too when `Instant::now()` is more than one `TICK` past the last poll.

**F2 — P2, design gap/risk: "the fix is by construction" is right about locks, but the rebuild path has two unstated preconditions and one leak the record does not name.**
(a) `build_inner` reads `config()`, a `OnceLock` (`engine/src/lib.rs:752`, `:585-609`). If a fork lands while the first settings init is in flight, the child's first call blocks forever on that lock — narrow, but it is shared state on the "lock-free" path. (b) The retired box is leaked deliberately (`:797-820`), so the child keeps the *inherited* `ureq` pool and its duplicate socket fds for its lifetime, never closing them; a long-lived forked worker keeps the parent's connections half-open and doubles client-side fds. The fork tests' children exit immediately (`tests/fork_mid.rs:100-110`), so nothing measures this. (c) The new test is still 25 rounds (`tests/fork_mid.rs:54`) and cannot distinguish the fix from a lucky pass — exactly the trap the notes admit for the old code. Smallest fix: one sentence each in NOTES/FINDINGS (settings must be read before fork; the child leaks inherited fds) plus a long-lived-child fd check.

**F3 — P2, inconsistency: the double-billing record contradicts the crate's own comments, and "counts sends" is really "counts attempts".**
`src/lib.rs:344-348` and the `post()` comment (`:940-943`) say the wire-repeated send on a dead pooled connection counts twice, while NOTES/FINDINGS say that shape counts once ("ureq absorbs it inside one `send()`"); the code increments once per `send()` call (`:944`), so the notes are right and both comments are wrong. The counter increments before the attempt (`:846`, `:944`), so a refused connection with no bytes on the wire bills 1 per attempt (3 with default retries); the refused-port arm checked the exception, not the counter. The billing test itself is honest in shape — the responder does drop the first connection after reading it (`tests/billing.rs:38-52`) — but `let read` is unused (compile warning in `engine/target/.../output-test-integration-test-billing`), so the assertion proves two accepted connections, not "both requests read" as FINDINGS item 4 says (`tests/billing.rs:99`). Fix: correct the two comments, assert `read > 0` in the billing test, and write "attempts" where the counter is defined.

**F4 — P2, design gap: the retention numbers cannot separate glibc retention from a per-batch leak.**
One 100,000-record batch, RSS 2.5 MB → 16.9 MB and stays (`engine/examples/retention.rs:89-115` samples one batch at 50 ms), then FINDINGS concludes "not a leak the engine can release". A leak would look identical. Fix: run two more batches and show RSS flat, or say "retention and leak are indistinguishable in this run".

**F5 — P2, inconsistency: the thread attribution's measured half is solid; the "halves" half is arithmetic, and the 83-thread explanation is a hypothesis.**
Source-verified: a set timeout makes `DefaultResolver` spawn one thread per lookup even for numeric addresses (`ureq-3.4.2/src/unversioned/resolver.rs:112-145`), so the ramp half is real. But "it halves to width plus three once the pool is warm" and "a synchronous numeric resolver halves the spike" are derived from the same one-peak-per-width output (`examples/threads.rs` prints one peak, no custom-resolver arm), and the earlier 83 came from a 50 ms sampler (`examples/retention.rs:66-77`, "the exact split … is not attributed here") against a different run's 1 ms sampler. Fix: label the derived numbers as derived, or add the sync-resolver arm.

**F6 — P2, confirmed inconsistency: job 3's removal of the ten `cases2/NN-*.json` originals broke the 207 stand-in's own runner and left its README stale.**
`cases2/` now holds only `conformance.json` and `tools/`; `engine/examples/cases.rs:17-30` reads every `*.json` there and expects the old case shape, so `conformance.json` falls through to the catch-all and the run ends "1 cases, 1 failed", exit 1 (`:229-233`). `engine/README.md:39-43` and `:71` still document running it and call `cases2/` "ten cases as data". Fix: delete or retarget the example, correct the two README lines.

**F7 — P2, confirmed inconsistency: 225's check count is wrong.**
`225/README.md` says "`python3 rules/tests.py`, 20 checks, 0 failures"; `rules/tests.py` has 25 `check(` call sites (`:25-122`) and prints only a failure tally, and the rule-4 checks were deleted on 2026-09-21 (`:45-46`). Fix: print the check count in the script and update the README.

**F8 — P2, risk/housekeeping: leftovers the notes imply are gone.**
`/tmp/foyer-smoke/` still holds 256 `foyer-storage-direct-fs-*` region files (~40 MB) though the cache lane records deleting every layout (`cache/NOTES.md`, cleanup); ~20 `/tmp/stub211*.pid` files remain with dead pids (checked `/proc/<pid>`: 172035, 3593135, 833688 all gone) while the engine lane 2 and Python lane notes say "its pid files removed" (`engine/NOTES.md` lane-2 close; `python/NOTES.md` stub-down). The PG lane's pidfile *is* gone and its containers are claimed removed with `docker ps -a` evidence — unverifiable here without docker. Fix: `rm -rf /tmp/foyer-smoke /tmp/stub211*.pid` and drop the "removed" wording or make it true.

**F9 — P2, inconsistency: FINDINGS overstates the null-cost replication.**
"median, three passes each by builder and orchestrator": the builder's record shows three (`engine/NOTES.md` benches), the orchestrator's shows one (`NOTES.md`, "engine lane verified by the orchestrator": 1.593 µs / 2.314 µs). Also trivial: Python's measured range is 1.473–1.523, printed as 1.47–1.52.

**F10 — P2, dead code: `python/src/lib.rs:250` is a leftover of the cancel-token regression.**
`let options = options_of(deadline);` is shadowed by `:253` and never read; the compiler diagnostic sits in `python/target/.../output-lib-ttb`, so the "one warning fixed" line in `python/NOTES.md` is not the last word. Fix: delete the line.

## Strengths (verified, not assumed)

- Every headline number I traced has a command and output behind it: 9.656–9.666 s / 9.658 s / 9.673 s, null 1.212–1.593 µs, 100-thread width 32 on 33 connections, cancel at 303 ms with 8 drained, PG 0.22 s / 2.119 s freezes, Python 0.207–0.209 s, ten forks at ~353 ms.
- The atomic-slot reasoning is genuinely sound for the question it answers: read is one acquire load + Arc clone (`lib.rs:797-812`), the slot never frees, losers drop their own unpublished box, and the rebuild takes only fresh locks — no inherited lock on the child's path. `thinkthen-core` has no statics (`repos/thinkthen/crates/thinkthen-core/src`, no `OnceLock`/`Mutex`/`static`), so `Backend::resolve` cannot reintroduce one.
- The engine is what it claims: no tokio/reqwest/hyper/async anywhere in `engine/Cargo.lock`; pool sized to the width with defaults verified in ureq (`config.rs:961-963`, 10/3/15 s).
- The Python tests are honest where it matters: the sigint test asserts the stub counter does not move after return and latency < 2 s; the fork-mid test alarms at 10 s so a hang is never hidden.
- Cache arms: foyer 0.22.6 pinned by lock, capacities 4 GiB/12 GiB in code (`foyer-bench.rs:121-127`), `RecoverMode::Quiet` is the crate default (`foyer-storage` builder doc), no `pub fn blocking` anywhere in the foyer crates, and the fork smoke really does hang the child on a disk get (`foyer-bench.rs:402-409`).
- Job 3: the divergence table is exact — 12 pass (01–08, 10, 16, 17, 19), 3 fail (09, 18, 20), 5 skips (11–15); the validator re-derives every expectation and flags the shaped-to-contract five itself. Job 2's baseline script exists and its printed shape matches the number quoted (`duckdb/.tmp/job2_baseline.py`).
- 225: 40 cases in `cases.json` with exactly 10 divergence flags, 40 `expected/*.json`, C01's recordings present for the replay proof.

## What I could not run (and the commands to run)

Reran nothing; no exec tool. Not verified here: any live bench or test; the docker container removal; the 225 replay; the 8.89-cent ledger; the job-2 baseline's actual execution. Do not treat the absence of a rerun as a pass. Supervisor commands: `cd 211/engine && ENGINE_NULL=1 cargo test --release --lib` and `ENGINE_BASE_URL=http://127.0.0.1:8214/v1 cargo test --release`; `ENGINE_NULL=1 ./target/release/examples/bench cost`; `cd 211/cache && ./target/release/foyer-bench verify /tmp/foyer-smoke`; `cd 207/engine && ENGINE_NULL=1 cargo run --release --quiet --example conformance_check`; `cd 207/engine/cases2 && python3 tools/validate_conformance.py`; `cd 225 && python3 rules/tests.py`; `docker ps -a | grep -c pg211`; the F1 starvation probe above.

## Roadmap

1. **F1** — fix the poll in the tick loop, re-run the sigint test on `ENGINE_NULL=1`, and state the corrected shape in FINDINGS before ADR 0017 copies it.
2. **F2/F3** — add the two preconditions and the fd-leak sentence; fix the two `lib.rs` comments, the billing assertion, and the "counts sends" wording.
3. **F4/F5/F9** — mark derived numbers as derived, or measure them (second/third batch; sync-resolver arm); correct the "three passes" line.
4. **F6/F7/F10** — repair the 207 example and README, fix the 225 check count, delete the dead line.
5. **F8** — clean `/tmp` and either make the "removed" claims true or drop them.

## Open questions

- After the loop, a token set (or deadline passed) after the last answer returns `cancelled`/`deadline` and discards a complete, paid-for result set (`lib.rs:572-580`). Is that intended, or should a fully answered batch win?
- The harness asked about a Polars equality proof (33 vs 1 connections) and a pandas 3.0.6 object-column Arrow-door claim. Neither exists under the named scope; 211's only analogues are `bench conn` (2 connections for 100 sequential) and the width-33 comparisons, and the pandas/Arrow notes live in `205-thinkthen-libs/python/NOTES.md:18,66-68,76-98`, outside this lens. Do not attribute those claims to 211 or to 225.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Ten findings with file:line evidence (e.g. engine/src/lib.rs:548-563 poll only on timeout; python/src/lib.rs:250 dead options confirmed by python/target fingerprint diagnostic; cases.rs:17-30 vs cases2/ contents; 225/rules/tests.py 25 check() sites vs README '20 checks'), each with classification, violated promise, and smallest fix."
    }
  ],
  "changedFiles": [],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "read/grep/find over experiments/211, 207, 225, ureq-3.4.2, foyer-0.22.6, thinkthen-core, /proc/<pid>, cargo fingerprint diagnostics",
      "result": "not-run",
      "summary": "No execution tool exists in this role; all verification was static. Nothing was rerun."
    },
    {
      "command": "ENGINE_NULL=1 cargo test --release (211/engine) and the stub-backed suite",
      "result": "not-run",
      "summary": "Not run by me; listed for the supervisor. Static read shows lib 3 tests, billing 1, deadline 4, fork_mid 1, retryable 1, wire 3 as recorded."
    },
    {
      "command": "cd 207/engine/cases2 && python3 tools/validate_conformance.py",
      "result": "not-run",
      "summary": "Not run by me; validator read in full and its 12/3/5 arithmetic checked by hand against conformance.json."
    },
    {
      "command": "python3 rules/tests.py (225)",
      "result": "not-run",
      "summary": "Not run by me; file read in full: 25 check() call sites against a README claim of 20."
    },
    {
      "command": "docker ps -a | grep pg211",
      "result": "not-run",
      "summary": "Docker is out of bounds for this review; container removal rests on the PG lane's own paste."
    }
  ],
  "validationOutput": [
    "Static: engine/Cargo.lock has no tokio/reqwest/hyper/async; ureq config.rs:961-963 confirms 10/3/15s defaults; resolver.rs:112-145 confirms a thread per lookup when a timeout is set.",
    "Static: foyer Cargo.lock 0.22.6; foyer-bench.rs:121-127 states 4 GiB/12 GiB; no 'pub fn blocking' in any foyer crate; RecoverMode::Quiet is the builder default.",
    "Static: cases2/ now holds only conformance.json + tools/, so examples/cases.rs reads conformance.json and fails it.",
    "Static: /tmp/foyer-smoke (256 region files) and ~20 /tmp/stub211*.pid files remain; the three pid values read from those files are absent from /proc (no live stub).",
    "Static: build fingerprints carry 'unused variable: read' (tests/billing.rs:45) and 'unused variable: options' (python/src/lib.rs:250)."
  ],
  "residualRisks": [
    "F1 poll starvation is code-evident but not reproduced; a fast-backend interrupt probe is the confirming test.",
    "The atomic-slot rebuild path's OnceLock precondition and the child's inherited-fd leak are unmeasured.",
    "Retention, the 83-thread explanation, and the 'sync resolver halves it' claim remain unseparated measurement from inference.",
    "PG container removal, the 225 replay, and the 8.89-cent ledger are outside what static reading can confirm."
  ],
  "noStagedFiles": true,
  "diffSummary": "Read-only review; no files created, edited, or staged.",
  "reviewFindings": [
    "blocker-for-ADR: engine/src/lib.rs:548-563 - the poll callback runs only on recv_timeout timeouts, so fast/null backends starve it and Ctrl-C is deaf until the batch drains, contradicting FINDINGS answer 4",
    "non-blocker: engine/src/lib.rs:752,797-820 - 'by construction' ignores the CONFIG OnceLock precondition and leaks the child's inherited pool fds",
    "non-blocker: src/lib.rs:344-348,940-944 vs NOTES/FINDINGS - pooled-dead retry wording contradicts the code; counter counts attempts, not sends",
    "non-blocker: tests/billing.rs:45,99 - unused read; assertion proves two connections accepted, not both requests read",
    "non-blocker: engine/examples/retention.rs:89-115 - one batch cannot separate glibc retention from a leak",
    "non-blocker: examples/threads.rs - 'halves to width plus three' and the sync-resolver saving are derived, not measured",
    "non-blocker: 207 engine/examples/cases.rs:17-30,229-233 + README.md:39-43,71 - job 3's removals broke the runner and left the README stale",
    "non-blocker: 225/README.md - claims 20 checks; rules/tests.py:25-122 has 25",
    "non-blocker: /tmp/foyer-smoke and /tmp/stub211*.pid remain while notes say layouts and pid files were removed",
    "non-blocker: FINDINGS - 'three passes each by builder and orchestrator' overstates the orchestrator's single null pass",
    "non-blocker: python/src/lib.rs:250 - dead options binding, still warning in the build"
  ],
  "manualNotes": "Nothing rerun: this role has read/grep/find only, so all conclusions are static and the supervisor should run the listed commands, starting with the F1 fast-backend interrupt probe. No secret values were read or reported. The lens items on pandas and Polars belong to 205-thinkthen-libs, not to any named scope folder, and are reported as not found rather than inferred."
}
```