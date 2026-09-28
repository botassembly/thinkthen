# 0170 run facts under batches: build record

Status: Correction candidate on `ticket/0170-run-facts-under-batches` from main `83e3cf5c`, awaiting fresh follow-up code review. No paid request or load campaign ran.

## Result and proof boundaries

The ten asking commands accept hidden-from-short-help `--facts`. One compact `thinkthen.run/1` object ends standard error after diagnostics and any usage warning, before signal re-raise. Its process counters count sent attempts and retries; its completion counter includes filtered and top-dropped input. Live usage presence and common model live only in memory. A failed run carries a stable stop cause and the engine's retried-status rule. Detailed batched rows carry whole-batch metadata, including split-half counts, without changing batch-one rows or the usage file schema. ADR 0083 amends ADR 0048 item 10.

The distinct compiled-command proofs are:

| Boundary | Focused evidence |
| --- | --- |
| Whole-run accounting and default bytes | 25 filtered inputs yield 11 printed rows, three loopback sends, 25 records and 300/30 tokens; status totals agree. The same command without `--facts` has identical stdout and empty stderr. `rank --top 5` over 20 counts 20. |
| Presence and storage | Mixed live usage and model omit both optional fields; replay makes no send and has no live tokens; cache answers count without inventing tokens; an unwritable usage folder warns before the final facts line. The old usage row reader remains covered by existing engine tests. |
| Stop and placement | A second batch fails at record 11 after one retry with ten earlier rows printed and three sends. A bounded table covers 503, 401 and 413 retry/too-large mapping, missing key and refused connection; dry-run and early configuration failure report zero sends. A held SIGTERM sends once and reports zero completed rows before signal termination; the existing second-signal case now passes `--facts` and keeps its no-line rule. |
| Detailed batch value | The existing duplicate-member test pins whole metadata, row shares and absent batch metadata at size one. The 0154 split fixture pins 3+2 half counts, positions, whole usage and first-half refused attempt. The batching module's 18 cases still pass. |

The proofs share the repository's compiled command harness and loopback. They do not create a second listener, signal, or timing framework. The status test uses its real per-process usage folder. The listener helper's `requests()` drains recorded bodies, so body assertions call it once and later totals use `count()`.

Focused validation on the candidate passed: `cargo test -p thinkthen --test backend facts::` (nine cases), `cargo test -p thinkthen --test backend batching::` (18 cases), the held-SIGTERM facts case and the existing interrupt module (ten cases before the final signal-row refinement), and the two named library tests for debug secrecy and an old usage row without `retries`. `cargo fmt --all -- --check` and strict `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass. `sdlc/scripts/tickets`, `pages`, `settings` with the compiled binary on `PATH`, `git diff --check`, and the exact ratchet pass. The coordinator retains the related-batch full gate after independent code review.

## Measured scope and budget amendment

At baseline `83e3cf5c`, the Rust ratchet was 77,650 nonblank lines. The corrected candidate measures 78,704, a net increase of 1,054. The accepted estimate was 567: 247 product and 320 tests. The coordinator authorized a measured amendment because the accepted behavior crosses more adapters than the estimate listed; the fresh reviewer must assess each group. No Rust file exceeds 500 nonblank lines. Closest are `cli/failure.rs` 496, `cli/asking/batched.rs` 490, `tests/backend/interrupt.rs` 490 and `cli/args.rs` 489.

| Responsibility | Accepted estimate | Measured net | Why it grew |
| --- | ---: | ---: | --- |
| Facts line and failure map | 110 | 128 | Separate compact serialization and one stop classifier; the status retry list is shared with the engine. |
| Flags and command lifecycle | 27 | 44 | `FindCommon` copies flags; early setup failures need a final line; `entry` writes after usage flush and before signal finish. |
| Completion adapters | 15 | 47 | `Output::take` counts hidden rows; judge, find, annotate, recognize and relate each hand off their completed result. |
| Batch metadata builder | 20 | 66 | A small private constructor holds the split and batch-one predicates; the worker carries setting and close reason across whole and half batches. |
| Engine in-memory reply facts | 35 | 87 | Presence and model state sit beside the existing counters without changing persisted `Counts`; the request path records them at its existing reply boundary. |
| Typed result and JSON path | 40 | 82 | Optional typed `BatchMeta` plus a result-only constructor preserves old callers and bytes. |
| Finalization signal state | 0 | 3 | A read-only guard accessor checks cancellation after the blocking usage flush. |
| Product total | 247 | 457 | The above exact groups. |
| Tests | 320 | 597 | New outside-in facts module 431; existing batching 22, split 11, signal 41, module registration 6 and a private held-flush child 86. |
| Rust total | 567 | 1,054 | `sdlc/ratchet.json` equals 78,704. |

Before raising the ratchet, I reused the existing `Counters`, `Output::take`, result serializer, loopback, signal fixture and split fixture. The old enum-only `Output` constructors were replaced with a single struct that carries the counter; no parallel completion walker was added. New stop tests use one status table and existing failure paths; they do not enumerate every command/status pair. Existing batch and signal tests gained assertions where they already own the boundary. Test functions were split by contract to pass strict cognitive-complexity and length lint; compacting them into one case would hide the failure being proved. No scaffold-only test was added or left behind.

## What the build taught us

- The preparation named full call paths but did not name every file that needed mutation. `find`, `annotate`, `recognize`, `relate` and `core/mod.rs` needed separate claims. The retained preparer updated the 0171/0172 handoff to inventory completion adapters and private exports explicitly.
- A loopback address permits a missing key by contract, so the no-key proof must use the built-in non-loopback address; it exits before any transport. Concurrent batches can withhold a successful first batch when the next fails, so the record-11 stop proof uses `--jobs 1` to establish its printed prefix.
- The initial signal proof counted a completed held response. The accepted zero-record edge instead returns a failed held response after SIGTERM; the signal remains the final cause and the sent-attempt count stays one.
- Existing source headroom matters more than a ticket's aggregate estimate. Local experiment 2028 independently confirmed 0172's `core/batch.rs` and `cli/args.rs` risks at frozen main. Refresh those numbers after this ticket lands. The 0170 measured growth needs independent review and does not alter any accepted behavior.

## Fresh review correction: signal during usage flush

The first fresh code review of `9f3c71d4` found one required race. `cli/mod.rs::entry` decided `stopped` before `Counters::finish`, which can wait on the usage ledger's file lock. A first SIGINT or SIGTERM during that wait produced a facts line with no `stopped`, followed by signal exit 130 or 143. The correction takes the usage snapshot and standard-error lock after the flush, then reads the guard's cancellation state immediately before writing facts. A late signal overrides the earlier cause only for this final facts member; the existing human diagnostic, usage warning, second-signal action and signal re-raise remain in their prior order.

The new child `tests/backend/interrupt/facts_flush.rs` reuses the command harness, signal acknowledgment and held usage lock. It reads a completed row before sending SIGTERM, keeps the ledger lock held until the signal is acknowledged, then proves the final facts line says `cancelled` with one sent request and signal exit 143. Removing only the new final-boundary check made that case fail at exit 101; restoring it made the case pass. The corrected interrupt module passes 11/11 and the facts module 9/9. The correction adds six product and 87 test nonblank lines to the independently reviewed +961 candidate. It does not add a timing campaign or another signal framework. The lesson for 0171/0172 is that finalization includes a potentially blocking persistence flush; a stop-state snapshot taken at request completion is too early for a final report.
