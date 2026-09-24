FINDINGS

# 0092 code review: loopback conformance backend

Reviewer: a fresh, read-only Claude session. It did not write the work. This is also the second-agent review that the repo's `CLAUDE.md` requires for the new dependency, the ceiling raise, and the new public surface. Reviewed code `ea2d01ef` and record `d6a76226` on `ticket/0092-loopback-conformance-backend`. I read the repo `CLAUDE.md`, `sdlc/README.md`, the ticket, the build record, the commit message, and the whole diff from `024e0d1c`.

## Findings

1. `conformance/backend/src/lib.rs:10-11`, `conformance/backend/src/listener.rs:118` and `:204`: the public interface is wider than the tests use. `DRIFT` and `Reply` are re-exported, but nothing outside the crate names them (checked with grep over `crates/thinkthen/tests` and `conformance/backend/tests`). `Listener::routing` is `pub`, but only `arms.rs` calls it. Fix: drop `DRIFT` and `Reply` from the `pub use` line. Make `DRIFT`, `Reply`, and `routing` `pub(crate)`. The surface then holds only `Backend`, `run`, and the three listener types the harness re-exports, plus `Recorded`, which `Listener::requests` returns.
2. `sdlc/records/0092-build-loopback-conformance-backend.md` ("Departures", last item): the `serve_once` duplicate in `crates/thinkthen/src/cli/conformance_tests/command.rs:209` is named as a follow-up only inside the record. No issue tracks it (`grep -rl serve_once sdlc/` finds only the record). Ticket 0076 adds two more hand-rolled loopback listeners in its uncommitted `src/engine/deadline_tests.rs:117` and `:182`. Fix: file one `sdlc/issues/` entry that names `serve_once` and asks 0076 to use the backend's held arm or `Listener` rather than a new listener. It should not block landing. `serve_once` is 16 lines, and moving it now would collide with 0076.
3. Landing: the merge onto `origin/main` `c53b6f78` conflicts only in `sdlc/ratchet.json`. After the merge, `ratchet.mjs` measures 45994, which is main's 44867 plus the same 1127. Fix: resolve the conflict to 45994, keep the list form of `directory`, and re-run the ladder on the merge commit before landing.

## What I checked, by command

- **Dependency.** In `crates/thinkthen/Cargo.toml` the new entry sits only under `[dev-dependencies]`. `conformance/backend/Cargo.toml` has `publish = false` and depends only on `serde` and `serde_json`. The `Cargo.lock` diff adds the member's own entry, which lists only `serde` and `serde_json`, and adds that member to `thinkthen`'s list. It adds no new third-party package. `deny.toml` is unchanged, and `allow-wildcard-paths = true` covers the versionless path. `policy.py` puts the member under the same lint, license, publish, and dependency tables. It also requires the member to declare no feature table and no target table. In the lint rung, `cargo deny` printed "advisories ok, bans ok, licenses ok". The packaged `thinkthen-0.0.1.crate` has no `conformance-backend` dev-dependency; only `proptest` is left.
- **Ceiling.** I counted non-blank `.rs` lines with `git show`, separately from `ratchet.mjs`. At `024e0d1c`: `crates` 44778, `conformance` 0. At `ea2d01ef`: `crates` 45035, `conformance` 870, total 45905. Before this change `conformance/` held no `.rs` file, so widening the count hides nothing. It brings the new member under the ceiling. Every existing copy of `ratchet.mjs` still reads a single string. The harness went from 431 to 60 lines. `listener.rs` is 442, of which 81 are new. Net new backend source is 378 (272 + 9 + 16 + 81), and 509 with `tests/binary.rs` (131). The commit message states what grew and where duplication was checked. The growth earns its lines: 430 lines of `loopback_cases.rs` run 46 cases through the real transport. `serve_once` should not block (finding 2).
- **Loopback-only.** Both bind sites in `listener.rs` (lines 175 and 228) use `127.0.0.1:0`. The backend makes no outbound connection. I ran the built binary: `[::1]` was refused, and when standard input closed the process exited and printed its final count. The wildcard-bind plant turned `it_binds_127_0_0_1_only` red.
- **Red-green.** I ran 11 plants in a scratch copy against `--test backend loopback` and `-p conformance-backend`. Ten went red:
  - wrong response byte: `loopback_cases`
  - backend drops case 40: `loopback_cases`
  - runner adds case 40 to the skip list: `loopback_cases:70`, the check that 46 cases ran
  - generic share 0.9 changed to 0.8: `the_generic_arm_answers_every_verb`
  - `invalid_probability` 1.5 changed to 0.95: the malformed test
  - gate never holds: both held tests, in `loopback_arms` and in `binary.rs`
  - reset arm serves 503: the fault-arm test
  - refuse arm serves 400: the fault-arm test and `loopback_cases`
  - wildcard bind: the bind test
  - a case miss falls through to the generic arm: the unknown-body test

  One plant survived. An early `return Ok(())` for one case inside `check()` stayed green. That plant disables the test itself, so it is not a gap in the backend.
- **Departures.** All four are defensible under the ticket.
  - Case 18 nesting: `annotating()` rewrites only the `on` pointers and the input records. Request bytes and digests are still compared, and they pass.
  - The `refuse` arm (422): `21-backend-fault` injects `response_refusal`, so 422 is the smallest status that carries it on the wire. The runner pins the exact sentence and exit 4.
  - Line count: 378 lines is under 450. The 131 test lines are the bind and exit proof that the acceptance asks for, so counting them separately is a fair reading.
  - Packaging `thinkthen` alone: the backend is `publish = false`, and the package rung passed.
- **Secrecy.** The backend's router reads only the request line and the body. `routing` keeps no copy of a request. Its only log lines are the drift lines on standard error, which carry the case id or the path. I sent `Authorization: Bearer sk-SECRETMARK` and `x-api-key` to a case miss, an unknown arm, the generic arm, and a bad cause. Standard error had 3 drift lines and 0 lines with the marker. No loopback test records. The existing `secrecy.rs` sweep still covers the command's failure paths, and it passed.
- **Merge and 0076.** Only `sdlc/ratchet.json` conflicts (finding 3). 0101 and 0102 touch `src/cli`, `src/core`, and `sdlc/scripts/live`, and they do not overlap this ticket's files. The 0076 worktree (`372277ff` plus uncommitted edits) changes only `src/engine` and `src/cli`. It has not touched `tests/backend` yet, so the only certain conflict is `ratchet.json`. It changes no failure sentence that the new tests pin; its `Deadline(Budget)` change stays inside the engine. The risk to watch: a 0076 test that needs a new `Canned` builder must now edit `conformance/backend`, which is outside 0076's `opens`. Name that in 0076's ticket or use the held arm.
- **Ladder.** One run at `d6a76226` in a scratch clone, with the `THINKTHEN_` variables unset. The one-minute load was 4 to 8. Results:
  - install: exit 0.
  - lint: exit 0; the ratchet printed `crates + conformance 45905/45905`.
  - test: exit 101. `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` failed at `scheduling.rs:102` ("document at 4"). That is the known flake in the open issue `sdlc/issues/2026-09-24-the-global-queue-concurrency-check-fails-under-load.md`. `serve_kept`'s peak accounting is unchanged by the move, which I checked by diff. Alone, the test passed 5 of 5, and a rerun of the whole `backend` binary passed 358 of 358. All the new tests passed, as did the 3 binary tests and `secrecy`.
  - spec: exit 0.
  - `sdlc/scripts/live` was not run. The scratch copies were deleted.

## Non-blocking notes

- The flake above may fire more often now, because the 46-case runner adds many process spawns to the same test binary. The issue's fix is the right one: hold both requests at the `after_release` barrier.
- `loopback_cases.rs:305` (`annotated`) compares answers, failures, model, and `meta.requests`. It does not compare the set's question digest or each answer's `request` digest. The in-process runner covers those, so the loopback run checks less for annotate cases than the cases hold.
- `loopback_cases.rs:230` accepts exit 0, 1, or 3 for every success case rather than the exit each case implies. Pinning the exit from the bare value would be stricter.
- The packaged crate's `tests/backend` now imports `conformance_backend`, which the package drops. The package's integration tests could not build from the tarball. That was already true of files included from `conformance/`, and nothing builds them today.
- `ratchet.mjs` now differs from the copy other repos share. It still reads a single string, so other repos are unaffected. Say so when the shared reader is next synced.
