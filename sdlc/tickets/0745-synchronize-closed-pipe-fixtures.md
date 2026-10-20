# 0745 — Synchronize closed-output-pipe cancellation fixtures

Status: OPEN.

Milestone: 0.2.1

## Outcome

The closed-pipe checks establish when the reader closes before asserting bounded request counts, independently of process scheduling, while preserving the product cancellation contract.

Changes: `crates/thinkthen-cli/tests/backend/annotate/scheduling.rs`, `crates/thinkthen-cli/tests/backend/parallel.rs`, `crates/thinkthen-cli/tests/backend/table.rs`, `crates/thinkthen-cli/tests/backend/tag/matrix.rs`, `crates/thinkthen-cli/tests/backend/closed_pipe.rs`, `sdlc/records/0745-closed-pipe-fixture-race.md`.

- Repair all five timer-dependent fixtures using the existing bounded rendezvous helper and explicit reader-closure ordering, including filter batch sizes one and ten.
- Keep all current request ceilings, deadlines, exit expectations, quiet-output and cancellation assertions; change no shipped code, names, types or documented behavior, and add no fixture machinery.
- Preserve safe release and child cleanup on a failed assertion.
- Record the repeated fixture cause and why the earlier 0352 repair left this ordering assumption intact; evidence only, with status kept in pm.

## Evidence

- M5 landing gate 0740/1792523048499-73469 failed at annotate/scheduling.rs:354, “broken pipe sent 9 requests”; 2164 of 2165 routine cases passed, and Linux lint passed; retained log is gate-runs/thinkthen/0740/m5/9134147e9680f2492c7224b471cda6315c44924c.
- The parent reads one buffered row after prefilling stdin, while all replies become available on a timer, so the child can emit additional rows and advance its window before the parent closes stdout.
- The same pattern occurs in parallel.rs, table.rs, tag/matrix.rs and closed_pipe.rs; adjacent annotate failure-after-close already holds replies with a rendezvous.
- Earlier 0352 commit 7b3f4cc4d synchronized the adjacent case but only lengthened this case's exit guard, leaving the ordering race.
- Exact proof command, replacing SHA with the pushed repair revision: `pm job run --machine m5 0745 SHA --disk 20 --memory 8 -- sdlc/scripts/gate`; this configured gate covers the routine backend cases and mechanical checks without a hand invocation; coordinator landing uses `pm ticket land 0745 --merge-branch --disk 20 --memory 8`.
- Before the configured gate, the focused existing backend entrypoint is `pm job run --machine m5 0745 SHA --disk 20 --memory 8 -- sdlc/scripts/job cargo test --locked -p thinkthen-cli --test backend pipe -- --nocapture`; the pipe selector includes all five repaired fixtures and existing pipe siblings.
- Classify failed-run reasons as product, setup or invocation and retain every failed receipt; do not rerun an unchanged failure until green.

## Progress

- 2026-10-20 started
- 2026-10-20 next: product: gate 1792524577676-83762 stopped at the known unpatched TLS advisory prerequisite before lint or tests; predecessor focused proof 1792524391492-24102 passed fifteen pipe cases, and final 440d71942 preserves original assertions while allowing legitimate cancellation before reply release, awaiting 0744 integration before final checks.
