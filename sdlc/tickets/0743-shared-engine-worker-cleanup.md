# 0743 — Prove shared-engine worker cleanup in the package check

Status: OPEN.

Milestone: 0.2.1

## Outcome

The shared-engine concurrency check proves that its engine workers leave after use in the existing package and routine checks. Diagnose the observed failure before choosing a repair, distinguishing a real engine leak from interference in process-thread accounting.

Changes: `crates/thinkthen/tests/public_batches.rs`. Add a necessary related source claim only after the reproduction identifies its owner.

- Preserve the shared-engine, throttle, two-thread and worker-cleanup assertions.
- Keep public names, types and documented behavior unchanged; do not raise deadlines, skip cases or add tooling.
- Check sibling worker-cleanup assertions for the same proven cause and repair the class.

## Evidence

- Arm64 M5 checkpoint 0688/1792518200923-4409 on 52a2e2c48a377315b095e4bef6d4fe2e0b48c6ee failed with exit 101 inside the package command's library-only cargo test, before routine lint/test ran: public_batches::one_engine_serves_two_threads_under_one_throttle_and_leaves_no_worker at public_batches.rs:282 reported “an engine worker stayed”; 62 passed, one failed and three were ignored.
- The completed log records no actual worker count or identity, so it establishes a failed assertion and does not decide the cause.
- Preserve that failed receipt, reproduce through the existing job and container commands, then prove the repaired class through the same package path and the configured gate before landing.

## Progress

- 2026-10-20 started
