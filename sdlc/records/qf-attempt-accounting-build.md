# Quick Fix: Count only admitted HTTP attempts

Status: implementation candidate on `qf/attempt-accounting`; fresh code review and landing remain. The accepted [boundary investigation](2026-09-27-attempt-accounting-preflight.md) and SQL settings ticket 0149 own the contract. This slice leaves the later `SendBudget` reservation to 0149.

Before the change, HTTP called the attempt-count hook and then checked the deadline again. The existing slow-hook test showed zero connections after expiry, but did not read usage. I extended that test with real counters. It failed red with `requests_sent: 1` and zero connections. The final code prepares counter bookkeeping before a callback-free token/deadline check, then marks one request and its retry bit before transport. It performs no second stop check. A held-response witness reads one in-flight attempt before releasing the reply. Overflow now returns the fixed existing `Defect` before mutating counts or starting transport.

The prepared guard reserves a pending slot and a 32-byte month buffer before the final check. It samples the month at admission. Its idle writer cannot create a ledger until a real delta is queued. The abandoned-preparation witness sees zero requests and retries, no usage folder and no persistence failure. A normal host callback still runs outside the usage lock. The private final check reads the fired flag, caller token and deadline without calling user code. Existing token, persistence and old-row behavior remain separate from this correction.

Focused proof passed: slow-hook deadline, held-response visibility, overflow refusal, old `thinkthen.usage/1` row with defaulted retries, fired token before final check, stopped retry wait, and compiled-command status showing two sends and one retry. The existing refused-attempt and retry-inside-budget cases passed during implementation. The refused-attempt case now also reads one request and zero retries from real counters. After its fixture cleanup, the stopped retry-wait case passed again. `cargo fmt --all -- --check`, strict focused library/test Clippy, ticket evidence, pages, ratchet and `git diff --check` passed. No paid call, stress run or full port campaign ran. The coordinator owns the related batch checkpoint after review.

The derived Rust ratchet rises from 76,686 to 76,915, a net 229 nonblank lines. This includes 88 lines for the private guard and the rest for bounded tests and small call-site changes. The changed source files remain below 500 nonblank lines; the largest is `engine/usage.rs` at 480. I reused `Counters`, its queue and writer, the existing HTTP retry loop, listener helpers and the old-row fixture. The direct `attempt_sent` helper remains test-only for existing accounting tests; production has one admission path. No new public API, dependency or duplicate ledger was introduced.

## What the build taught us

- The old slow-hook proof established no transport but missed the false usage count. One real-counter assertion exposed it before the fix.
- The hook could not simply move after the last deadline check: its mutex and writer startup can wait. Counting after the response would hide an in-flight attempt. The small prepared guard preserves both boundaries.
- A cancellation fixture that fired only from a removed pre-send hook needed a real server-fired stop. The selected retry-wait test still proves one first send and no second attempt.
- Strict lint caught a one-iteration server loop left by that fixture change. A single accept expresses the test boundary directly.
- An idle writer is acceptable only if abandoning preparation leaves disk and the persistence-failure signal untouched. The zero-ledger proof protects that detail.
- The later SQL budget work must reserve its send allowance between the final check and mark, as accepted 0149 specifies. This slice cannot claim that reservation yet.
