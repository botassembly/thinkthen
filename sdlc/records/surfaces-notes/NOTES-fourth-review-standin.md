# Fourth review: stand-in and contract lane — probes, rulings, and evidence

Date: 2026-09-23. Items from `sdlc/issues/2026-09-23-surfaces-branch-fourth-review.md`, numbers carried. Every closing probe below ran against the reviewed tip (old code, `git stash` of the fix) and against the fixed tree, in that order, with both outputs recorded. Probes live in the tree as committed tests (`standin/tests/churn_probe.rs`, `standin/tests/scale_probe.rs`) plus the in-crate unit tests.

## Item 1 — the state-table use-after-free (HIGH)

The defect: eviction and retraction freed the slot's box (`*Box::from_raw`) the instant the compare-and-swap won, while other threads held loaded pointers to it; the grace protected the retired `Arc`, never the box. The comment claiming "a stored pointer is never freed" was false, and freed addresses could return as new boxes, arming the compare-and-swaps' address-reuse hazard.

The fix — safe by construction, no grace reasoning required: a slot's box is never freed. Eviction writes the tombstone into the box in place (`ptr::replace`) and retires the `Arc` as before; a straggler reader clones the tombstone, matches nothing (its pid is zero), and looks the state up again. Because no box is ever freed, no address is ever reused, and the CAS hazard is closed by construction rather than by timing. The cost is eight bytes per eviction, counted by `retired_boxes()` and listed in a registry so the never-freed memory reads as retained, not leaked, to the sanitizer. The false comments are replaced with the true invariant.

The reviewer's probe shape (`lens1r4/c/churn.c`): seventy engines with distinct timeouts, thirty-two threads, repeated create/evict/retract cycles. Rebuilt as `standin/tests/churn_probe.rs` — fresh engines with wide-stride timeouts every call, so publishing, eviction, and retraction overlap the lookups continuously.

Old code, ASan (`RUSTFLAGS=-Zsanitizer=address cargo +nightly test -Zbuild-std --test churn_probe`), three runs: heap-use-after-free in runs one and two, clean in run three — the reviewer's five-of-six rate, reproduced. Fixed code, same command, six runs: `test result: ok. 1 passed; 0 failed` six times, no ASan summary. Fixed code in release (`cargo test --release --test churn_probe`), three runs: ok. The C-door variant of the probe drives the same public path (`from_settings` to the same table through the C door's connector), so the door-level crash shares this fix; noted as residual below that the C-door binary itself was not rebuilt for this evidence.

## Item 18 — the stand-in's tests that could not fail

`settings_state.rs` used two settings values, which never evict. New: `standin/tests/scale_probe.rs` drives two hundred distinct values (and, with a live listener, one hundred against real idle sockets), measuring `/proc/self/fd` and `VmRSS` before and after a post-grace sweep.

Old code, socket probe as first written against a dead port: passed — honestly recorded, because a refused connect leaves no idle socket to leak; the probe was rebuilt with a live listener to make the leak reachable. With the live listener the old code also passed its post-sweep assertion: the wave-four retire/grace/sweep machinery already closed pools once any later lookup swept. What the old code could not survive was the box free (item 1) and the reviewer's during-churn accounting (4 to 194 open files while the grace was still running), which is a bounded transient the design accepts. The new probes now pin both quiesced shapes — descriptors flat within eight, residency flat within 8 MB — and fail on any regression to the leak.

The `settings_state.rs` flake (two failures in fifty under load) was the listener counting accept-to-close: a finished response's tail could overlap the next connection. The count now covers only the delayed window, which a width-1 permit spans exactly.

The digest flake (four of fifty) and the `THINKTHEN_BASE_URL` leak were one bug: `the_testkit_reports_a_visible_skip` set the variable inside its scenario and `switches()` restored only `ENGINE_NULL`, so the digest test read the wrong base whenever the order lined up. `switches()` now restores the whole removed set.

## Item 22 — the early return under the settled spelling

The stand-in's wire suite read only `ENGINE_BASE_URL` and `ENGINE_WIDTH`, so a suite running under `THINKTHEN_BASE_URL` decided its skip against the default port and returned early although a backend was reachable. Both helpers now read the settled spelling first with the deprecated one beside it.

## Leftovers — ruled and enforced

- Width ceiling: `MAX_WIDTH = 4096`, refused with the usage kind and the ceiling named, before any thread or allocation (`build_inner`). Fail-then-pass: old code answered width 100,000 with a wire error (`backend: Connection refused`) — and panics at 322 MB with bulk records, per the review — while the new code answers `usage: width 100000 exceeds the ceiling 4096` (test `an_absurd_width_is_refused_not_panicked`).
- `Options::deadline_in` checked add: a budget the clock cannot name is treated as no deadline, documented and tested (`an_unrepresentable_budget_is_no_deadline_not_a_panic`); the panic is gone. Rule recorded: an unrepresentable wall is not a spendable budget and never a host panic. Superseded in review 5: ADR 0041's amendment refuses a budget too large with the usage kind, tested by `an_oversized_budget_is_refused_on_every_door`.
- DNS, failed connects, and TLS-handshake failures are now non-retryable (`classify_transport`), each with its reason named; a mid-flight reset keeps its retry. Four unit tests pin the split.
- `request_digest_for`: the engine digests against its own configured base, the environment only answering for helpers with no engine in hand. Test: two addresses are two digests, and the engine's trail agrees with the helper named for its base.
- The JSON door key-presence rule (`"rank": false`): the enforcement site is the C door's dispatch (`libraries/c/src/lib.rs:864` acts on `contains_key`), outside this lane's folders. The rule, recorded for the C lane: a flag key on a JSON door accepts only its documented spelling — `"rank": true` to rank, absent to filter — and any other value is a usage error naming the key and the accepted spelling, because silently ignoring `false` hides a mistyped intent and honouring it by presence ranks a caller who asked not to rank. The contract's question and set doors were probed and already refuse `"rank": false` (`usage: a question file holds one of...`), so no contract-side change is needed.

## What ran at the tip (rule 3)

`cargo test` in `contract/` (29 passed) and `standin/` (39 passed), zero warnings. Clippy `-D warnings --all-features` in both crates fails on pre-existing findings only — five in the stand-in (lines 268, 291, 1243, 1761, 1808 of the old code), four in the contract (205, 758, 783, 1764) — none in this lane's lines; the workspace-wide lint sweep is the gate lane's item. The repo-wide surfaces ratchet needs its ceiling re-raised for this lane's added lines, the gate lane's mechanism; the count at this tip is recorded there. The full surfaces gate (Docker, npm) did not run in this lane; its rung is the verifier's.
