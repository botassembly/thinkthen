# 0092: Serve the shared cases from a loopback backend

Status: landed. Built at `ea2d01ef` on `ticket/0092-loopback-conformance-backend`. The code review (`sdlc/records/0092-code-review.md`) was also the second-agent review the repo requires for the new dependency, the raised ceiling, and the crate's new public surface. Its fixes are listed under "Review follow-up", and the coordinator accepted them without another review.

## Result

- `conformance/backend` is the package `conformance-backend`: a library and a thin binary. It is a second root workspace member, unpublished and test-only, as the ticket rules.
- The library holds the loopback listener moved from `crates/thinkthen/tests/backend/harness`, with its items made `pub`. The listener gains `routing` (answer from the whole request and keep no copy of it), `count`, `origin`, and a held-reply gate. `tests/backend/harness/mod.rs` keeps `spawn` and `process_has_file` and re-exports `Canned`, `Listener`, and `Observed`, so the 49 importing files did not change.
- `Backend` binds 127.0.0.1 on an ephemeral port. The binary prints the port, answers `count` and `release` lines on standard input, and prints the final count when standard input closes.
- The base path picks the arm. `/case/ID/v1` answers a case's exact bodies. `/generic/v1` answers any well-formed request: the first option, level, or yes gets 0.9 and the rest share the remainder in declared order. `/arm/reset`, `/arm/429`, `/arm/503`, `/arm/refuse`, `/arm/held`, and `/arm/malformed/CAUSE` are the fault arms. An unknown body, arm, or request gets status 500 and a line on standard error.
- `conformance/README.md` documents the arms and the standard-input lines.
- Gate changes, all inside `opens`: `policy.py` admits the member and holds it to the lint, license, publish, file-size, and dependency tables. `package` packages `thinkthen` alone. `ratchet.mjs` accepts a list of folders, and `ratchet.json` counts `crates` and `conformance`.
- Dependencies: `thinkthen` gains the path dev-dependency `conformance-backend`. The backend uses `serde` and `serde_json`, which were already in the lock file and in the test harness's build. `Cargo.lock` gains only the member's own entry. `deny.toml` needed no change. `cargo deny` reports advisories, bans, and licenses ok.

## Acceptance

| Criterion | Proof |
| --- | --- |
| The command runner passes every success case and every wire fault case against this backend, with recomputed digests | `tests/backend/loopback_cases.rs` runs 46 cases through the compiled command: every success case (01 to 19, 26 to 28, 32 to 52) on its case arm, and `21-backend-fault` on the refusal arm. It swaps each canonical request digest for the digest of the served URL before comparing. Cases 20, 22 to 25, and 29 to 31 stay in-process. Planted: keeping the canonical digests failed 43 of the 46 cases on `requests`. The other three print no row |
| One case per fault arm: reset, 429, 503, held, and each of the six malformed replies | `loopback_arms.rs`: `each_wire_fault_arm_yields_its_kind_and_sentence` pins the exact sentence, exit 4, and request count for reset, 429, 503, and refuse. `each_malformed_arm_fails_the_last_question_with_its_cause` pins the exact bare row and exit 6 for all six causes. `a_held_reply_answers_only_after_its_release` proves the command is still waiting 300 ms after the request arrives and answers `true` after release. In the binary, `a_held_reply_waits_for_a_release_line` does the same through a `release` line |
| An unknown body on the case arm returns the distinct 5xx, and a planted fall-through turns that test red | `an_unknown_body_on_the_case_arm_earns_the_drift_status_and_never_a_generic_answer` pins the status-500 sentence, exit 4, and 3 requests. Planted: a case-arm miss that fell through to the generic arm printed `true` and the test failed on `left: "true\n"` |
| Each fault arm yields its expected kind and cause through the command | See the two fault rows above. Every arm fails as a backend failure. Exit 4 means a whole-call failure, and exit 6 with the named cause means a failed question |
| The generic arm answers every verb | `the_generic_arm_answers_every_verb` pins the exact output and exit 0 for decide, choose, tag, score, filter, rank, find, annotate, recognize, and relate |
| It binds 127.0.0.1 only and leaves no process behind after standard input closes | `conformance/backend/tests/binary.rs`. `it_binds_127_0_0_1_only` connects on 127.0.0.1 and is refused on 127.0.0.2. Planted: a wildcard bind answered on 127.0.0.2 and the test failed. Each binary test closes standard input, reads the final count, and fails if the process has not exited within 5 seconds |
| Production code in `thinkthen` is unchanged | `git diff 18c0dc10 ea2d01ef -- crates/thinkthen/src` is empty |
| The crate stays under 450 nonblank Rust lines net of the moved harness | The backend's source is 378 nonblank lines net of the 361 moved listener lines: `arms.rs` 272, `main.rs` 16, `lib.rs` 9, and 81 new listener lines. Its own binary tests add 131 more, for 509. See the departures |
| No dependency beyond what the harness uses | `serde` and `serde_json` only. Both already built into the harness's test binary |

Red first: before any arm existed, a stub that sent every request to the drift status failed all three binary tests and four of the five arm tests. The unknown-body test passed under that stub, so its red is the planted fall-through above.

## Departures

- Case `18-annotate-two-groups` cannot run through the command as written. Its pointers `/summary` and `/body` equal its question names, and the command refuses to append an answer over an existing field. The runner nests the record's parts under `record` and moves each pointer there. No pointer reaches the request, so the bytes and digests stay the case's own. Renaming the case's questions would remove this step. That is a question for the case's owner.
- `21-backend-fault` is both an injection case and the one backend fault case. It still runs in-process in the engine runner. The loopback runner also runs it on a new `refuse` arm, status 422, which is what its injection `response_refusal` maps to. The ticket names no refusal arm, and this arm is the smallest way to carry the case on the wire.
- The 450-line cap holds for the backend's source, 378 lines. With its own tests the crate is 509 lines net of the moved harness. The ticket does not say whether tests count.
- `ratchet.mjs` is the shared reader other repos copy. It now accepts a list of folders. Without that change, the new member would sit outside the ceiling the ticket puts it under.
- The in-crate runner `src/cli/conformance_tests/command.rs` keeps its own one-shot loopback listener, `serve_once`. Moving it onto the backend would remove a duplicate, but `src/cli` is out of bounds while 0076 builds there. `sdlc/issues/2026-09-24-hand-rolled-loopback-listeners-duplicate-the-test-backend.md` tracks it.

## Review follow-up

- `DRIFT`, `Reply`, and `Listener::routing` are now private to the crate. The public surface is `Backend`, `run`, `Canned`, `Listener`, `Observed`, and `Recorded`. Clippy on the workspace compiles clean, which proves nothing outside the crate names them.
- The duplicate loopback listeners are filed as an issue: `serve_once` and the two that 0076 adds.

## Ceiling

Built at 44778 to 45905. After the rebase onto `c53b6f78` it is 44867 to 45994, the same +1127: the backend +870, of which 361 lines moved; the harness -371; `loopback_cases.rs` +430; `loopback_arms.rs` +192; `main.rs` +6.

## Checks

Run once at `ea2d01ef` with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, the one-minute load under 4:

- `sdlc/scripts/install`: exit 0.
- `sdlc/scripts/lint`: exit 0. Policy, ratchet `45905/45905`, deny, format, Clippy, and docs pass.
- `sdlc/scripts/test`: exit 0. The backend binary ran 358 tests and the conformance backend ran 3. `live-test: all cases passed`.
- `sdlc/scripts/spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

Ian or the queue owner can overturn the `refuse` arm, the status-500 choice for drift, the ratchet reader change, and the way the runner nests case 18's record.

## Landing checks

Run once at `987b09ba`, rebased onto `c53b6f78`, with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. The one-minute load was under 4.

- `sdlc/scripts/lint`: exit 0, ratchet `crates + conformance 45994/45994`.
- `sdlc/scripts/test`: exit 0 on the first run. The global-queue test did not flake. `live-test: all cases passed`.
