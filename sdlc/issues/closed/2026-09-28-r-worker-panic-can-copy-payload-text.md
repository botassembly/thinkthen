# R worker panic diagnostics can copy payload text

Status: closed 2026-09-30. Fixed by ticket 0306 (`4de0f8116`): the R worker calls `thinkthen::contained`. Target-package proofs move to the release issue.

Filed 2026-09-28. Confirmed by source inspection on main `cd395d3f` during the bounded 0226 native-guard preparation. This records a diagnostic path that can copy a panic payload. It does not claim that a real credential or user evidence has appeared in a production panic.

## Evidence and retained behavior

`libraries/r/thinkthen/src/rust/src/calls.rs::on_worker` catches the worker unwind and creates a defect containing `words(&*panic)`. The `words` helper copies either a string slice or owned string from the payload. The existing worker test deliberately panics with `boom` and expects `the call panicked: boom`, then proves a later call succeeds. A Rust panic hook runs before the catch, so replacing the returned sentence alone does not establish standard-error secrecy. The engine's landed guarded scopes do not cover every possible panic in the R wrapper's worker body.

Keep the six error kinds, fixed non-retryable defect classification, cancellation and caller-thread R behavior, joined or detached worker lifetimes, and subsequent successful calls. A reviewed fix should use a fixed diagnostic and preserve unrelated host panic handlers. Check the accepted private scope pattern from ADR 0081 and the forthcoming 0226 design rather than introducing per-call process-global hook swaps or a public fault switch. R's scope and package proof remain separate from C, SQLite and DuckDB; no all-port closure follows from those fixes.

## Closure proof

Use one small child-process regression with a synthetic payload marker at the actual private R worker boundary. Check stdout, stderr and the returned error for absence of that marker; retain the defect kind, non-retryable behavior and a later successful call. Show that an unrelated host panic still reaches its intended handler. Keep injection private to the test boundary, preserve R's normal interrupt and shutdown contract, and run only the relevant installed-package or equivalent host boundary proof. Source inspection alone does not close this issue.

## Reviewed implementation and remaining proof

Ticket 0227 source and the evidenced Linux package subset landed from corrected candidate `e52c1604` after fresh High code review. The source children prove fixed non-retryable Defect, both synthetic payload forms, later success and unrelated prior-hook delivery. Installed Linux packages separately prove loading and no-fault later use. Python's reviewed correction also covers implicit reference cleanup. See [the build evidence](../../records/0227-language-panic-build.md) and [code acceptance](../../records/0227-code-review.md). Other target packages and the later 0212 Call conversion integration remain open; this issue is not counted complete from the Linux subset alone.

## Ticket 0306

Ticket 0306 removes this binding's own panic hook. Its guard now calls the shared `thinkthen::contained`, which forgets the payload inside the marked scope. The existing child test passes unchanged. The target-package proofs above remain open.
