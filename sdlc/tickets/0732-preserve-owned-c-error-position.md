# 0732: Preserve the native failing record position in owned C errors

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

- Changes: `libraries/c/src/failures.rs`, `libraries/c/src/failures/`, `libraries/c/src/ffi/run_call/`, `libraries/c/include/thinkthen.h`, `libraries/c/tests/`.
- Preserve the native failing record position when `thinkthen_run_call` returns an owned error, and expose it through the surviving owned C error interface before 0688 freezes that interface.
- Represent an absent position distinctly from every valid native position; never infer the position from the message or input.
- Keep error kind, message, retryability, facts, admission, send counts and ownership unchanged. Add no legacy session or thread-local error path.
- Ask the PM for any required public accessor name or type decision before implementing that public shape. Search the generated header ownership before editing it; update its source rather than a generated copy.

### Proof

- Through an installed C archive outside the repository, reproduce an owned admission failure at record positions one, two and three and verify its position agrees with the native failure. Cover absent position, independently owned errors, freeing and zero sends.
- Demonstrate the new regression fails against the old conversion and passes with the repair, without changing diagnostics or the shared oracle.
- Run the relevant existing C ownership and error checks, followed by the configured integrated gate before landing. Use admitted build-machine jobs; Linux installed proof joins the checkpoint.

## Evidence

- The 0640 developer found that `Failure::from(Error)` in `libraries/c/src/failures.rs` preserves code, message, retryability and facts but drops native `stopped.at`; both immediate and blocking-call owned errors use this conversion.
- The current packet corpus has twenty-seven case IDs with native failing positions: twenty-four at one, two at two, and one at three. The cache and replay declaration cases fail on their second invocation. Native admission attaches the position before creating the session.
- 0690 deletes the session packet and thread-local error paths, leaving `thinkthen_run_call` and its owned error as the public C failure path used by host libraries.
- PM approved this pre-freeze ticket in ask two of the 2026-10-20 0640 C boundary ruling and requires 0688 to depend on it. The 0640 installed-call checks retain their current observable error contract until this repair lands.

## Progress

- 2026-10-20 started
