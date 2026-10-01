# Register 45: ordered wait disposition

Source pin: main `fe59c3a2`, 2026-09-29. This is a design disposition for original register local experiment 284's `45-d10-ordered-output-stalls.md`, not a product change. The prior [readiness snapshot](2026-09-29-engine-command-cache-readiness.md) at reviewed `aaf7afaa` proposed a position-only wait fact; this pass checks that proposal against the current command contract.

## What is already fulfilled

- [Records](../../specification/records.md#jobs) now states the input-order guarantee, the slow-earlier-batch stall, the window effect, attempt timeout and retries, and the absence of a whole-run deadline. [Rank](../../specification/rank.md#what-it-prints) says it emits no order until all answers are ready. There is no finite worst-case *run* stall bound to print: `--timeout` bounds an attempt, while retries, backoff, future input and the lack of a whole-run deadline keep the run bound open. The original 1,300–1,500 records/minute observation describes an old no-stall run, not a present bound or new measurement.
- The scheduler's [`Run::waiting`](../../crates/thinkthen/src/engine/schedule.rs) uses `dispatched - next` for streaming commands and windowed `rank --top`; they stop requesting input when that ordered window reaches `jobs`. Uncut `rank` uses `in_flight` and can admit later work while the oldest result is pending. `drain` advances `next` only in order. A current position is an internal batch index, not necessarily one record: default `batch=max` can put several records in a request; interactive command users can choose `--batch 1`. The input reader may not yet have supplied a next record.
- On failure, the [stop diagnostic](../../crates/thinkthen/src/cli/failure/stopped.rs) names the failed record or batch range and finished count; [records](../../specification/records.md#failure) fixes the sentences. Signal stops deliberately name no guessed record. [Run facts](../../specification/result.md#the-run-facts-line) are one **terminal** stderr JSON line, written after the command and any diagnostic by [`cli/mod.rs`](../../crates/thinkthen/src/cli/mod.rs); `stopped.at` is present for a named failure and absent on a signal. A successful completed run has no `stopped` and no one waited position to report.

## Decision and routing

The original severity-3 item says no code change is required, but one success clause asks terminal facts to name the record the run *waits on*. That clause conflicts with the settled terminal facts shape and with packed batches. Adding `waiting_at` after completion would be stale or arbitrary; copying `stopped.at` would duplicate an existing failure fact and would not identify a live stall. It also risks confusing the first record of a batch with the specific slow member. No useful unmet observable under the present `--facts` contract warrants a 0274 implementation ticket.

Recommend that root mark register 45 **non-issue for its remaining terminal-facts clause**, citing the already met documentation and failure-position criteria above. Keep the accepted ordering and failure behavior. If a real user later needs visibility *during* a quiet run, open a separately motivated, lower-priority live-progress design; decide its opt-in channel, cadence, batch-range position and secrecy rules then. Do not graft a periodic progress API onto terminal `thinkthen.run/1` for this lane. Ian can overturn this disposition, but routine criterion routing needs no Ian decision.

No source, test, contract, provider, SQL, DataFrame or site work was done. No old throughput figure was remeasured. The only file claim used is this record; no empty ticket 0274 is created. Root owns the reviewed item-table status change and landing.

## Independent review and decision

Fresh Medium review accepted `0a0adf8a09a82cb0cad672cb30652aa1c29c8d62` after tracing the three scheduler modes, terminal facts emission and failure-position rules. The coordinator adopts the proposed non-issue disposition and updates the item row in this landing. This rejects a redundant or misleading terminal field; it claims no new code fix. No source, test or schema changed.
