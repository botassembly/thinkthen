# 0731: Restore batch-one pacing for native session feeds

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0639

Reviews: revision b2b8488, accept

## Outcome

- Changes: `crates/thinkthen/src/public/pull.rs`, `crates/thinkthen/src/public/pull/`, `crates/thinkthen/src/public/options/readiness.rs`, `crates/thinkthen/src/public/request/`, `crates/thinkthen/tests/request_session.rs`, `crates/thinkthen/tests/request_session/`, `crates/thinkthen/tests/public_batches.rs`, `crates/thinkthen/tests/public_batches/`. Private readiness carries the pacing distinction so session construction preserves the command-line parallel path without changing public controls.
- Restore the documented batch-one session behavior: finish each row before requesting the next native input. Supplying readiness must not silently enable speculative batch-one input pulls.
- Keep the command-line tool's deliberate readiness-based parallel path and the existing batch-max behavior. Keep readiness, cancellation wakeups, parked unread answers, scheduler limits, public signatures and packet formats.
- Keep the existing Go `feed-full` assertion and cancellation checks unchanged. Fix the private pacing/readiness distinction, not the fixture's expected intake count.

### Proof

- An outside-in native session regression combines readiness, batch one and a held first provider response. It fails before the repair and passes afterward, preserving the existing bounded-input contract. Reuse the repository's loopback and feed helpers.
- The existing Go owned installed `feed-full` case passes unchanged against a freshly built native package, including its later cancellation assertion. Preserve the original failing receipt.
- Existing direct batch-one, session-readiness, parked-answer and command-line parallel behavior checks pass. The configured Linux and native macOS gate passes on the ending revision through pm job. Do not add a runner, raise a deadline or weaken an assertion.

## Evidence

The serial Go diagnostic on source `f4cd9171c` in Yellow job `1792472604853-604246` returned child exit 1, not a timeout. `libraries/go/fixtures/owned_consumer.go:393` rejected the intake count after the call returned; `owned_installed.py:123` retained the panic. Peak memory was 1,570,594,816 bytes with no memory events. The log does not expose the actual pull count or prove the later cancellation error value.

Read-only investigation traced commit `0eca6c567`: `public/request/session.rs:630–636` supplies readiness to ordinary sessions, while `public/pull.rs:145` makes batch-one pacing conditional on no readiness. The normal scheduler window then applies. ADR 0111:259, ADR 0053:72 and ticket 0554:61 preserve sequential batch-one pacing; 0598 explicitly leaves the scheduler unchanged. Existing session-readiness tests use batch max and miss this combination.
