# 0445: Complete attempt observations and command facts

Status: in progress. Native behavior is implemented and landed. Remaining surface adoption is tracked in the existing SDK, foreign binding, SQL, dataframe and MCP family checklists; final shared-case qualification remains.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

All ten functions and typed surfaces expose the supported opt-in attempts on success and started failure. Recording-side timing remains separate from header-free answer bodies.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4 and 9.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Finish the open 0302 remainder: find/recognize/relate/annotate, failed runs, typed host attempts, command_ms and a bounded opt-in timing sidecar keyed to saved entry digests. Preserve the fixed provider header allowlist and distinguish provider request_id from outbound SDK IDs.
- Proof: Saved/loopback exchanges prove present/absent provider times and IDs, retries, failed runs and zero-send replay through each host consumer. Missing server time remains absent. Sidecar has no raw headers, key, address or body. Pin command duration definition so overlapping HTTP intervals are not double-subtracted; shared timing uses monotonic edge clocks, never core time.
- Defers: Paid header diagnostics, guessed server time, provider bills and inferred chat-adapter timing without a supported contract.

## Dependencies and ownership

0442 settles result contract; 0443 owns outbound IDs. Host families expose attempt carriers; 0435 owns SQL facts and native failure route. Update every-surface facts guidance alongside owning implementation.

Native WIP: CallOptions.attempts(bool) retains actual ordered attempts in final
success/started-failure facts, including requested zero-send []. Prepared SDK
IDs are distinct from screened provider request IDs. CompleteAttempt and
CompleteFacts serialize those identities; released facts/attempt serialization
remains its explicit compatibility shape. Existing caller-thread observers,
joined late replies and join-before-unwind/drop behavior remain tested.
Command timing, recording sidecars and complete execution routes remain open.

CLI usage constituent: final run facts now sum actually reported input/output
independently, retaining input887 when output is absent on successful or refused
answers. Count-only durable totals add each known dimension without treating a
missing counter delta as observed zero. Full-cost estimates still require all
sent replies to report both counts. The compiled-command regression failed
before this fix; existing facts/pricing/overflow/cancellation cases pass. Native
cache/replay cases also distinguish empty usage from explicitly reported zero,
and the strict complete schema rejects a present empty usage object. Source
grows 76 nonblank Rust lines (145934 to 146010) for those outside-in cases and
shared partial-count bookkeeping; the unused full-count helper was removed.
Command timing, sidecars, aggregate CLI and host adoption remain unfinished.

Command timing constituent: `--facts` now includes `command_ms`, rounded up
after subtracting the monotonic union of actual HTTP/body-read scopes from
accepted command execution. Measurement continues through ordered output and
usage-writer completion; total `seconds` keeps its original meaning. A shared
constant-space timeline counts overlapping workers once, closes through unwind,
and omits an invalid measurement instead of fabricating it. The prior-failing
compiled-command case holds two parallel replies and then keeps stdin open:
backend time is excluded once and subsequent input waiting remains command time.
The timeline's table covers its own disjoint/overlap behavior. Existing facts,
pricing, stop/flush and native tally checks pass. Timing sidecars and remaining
CLI/host adoption stay open; this is no whole-ticket completion claim.
Source grows 221 nonblank Rust lines (146010 to 146231) for the shared
constant-space clock, actual transport integration and outside-in command case;
HTTP timing and configuration warning helpers keep the existing flows readable.

### Added public declarations

```text
const fn Facts::call_id(&self) -> Option<&CallId>
struct CompleteAttempt<'a>
struct CompleteFacts<'a>
impl Serialize for CompleteAttempt
impl Serialize for CompleteFacts
const fn AttemptObservation::sdk_request_id(&self) -> &SdkRequestId
const fn AttemptObservation::complete(&self) -> CompleteAttempt<'_>
const fn CallOptions::attempts(self, bool) -> CallOptions<'a>
fn Facts::attempts(&self) -> Option<&[AttemptObservation]>
fn Facts::complete(&self) -> Option<CompleteFacts<'_>>
const fn CompleteFacts::call_id(&self) -> &CallId
const fn CompleteFacts::counts(&self) -> &Facts
fn CompleteFacts::attempts(&self) -> Option<&[AttemptObservation]>
```

### Native bounded recording timing constituent

Explicit recording now uses the existing attempts opt-in (`CallOptions::attempts(true)` or CLI `--details --record`) to append `thinkthen.timing.jsonl`. Cache/refresh and ordinary recording create no timing history. Entries contain only saved question key and actual attempt ordinal/wall/outcome/status/server time. Parent size refusals accompany both accepted split children; current replay attempts remain empty and replay does not mutate history. Stable folder locking preserves concurrent appends, with 8 MiB/65,536-entry limits checked before the answer/original transaction. Invalid or full history preserves previous answers/history and fails locally after the actual send without retry. The separate durable replacement is not a cross-file SQLite transaction; a post-commit filesystem failure can retain answers without the new history and returns the existing safe local error. No-store refuses before sidecar creation.

Five native outside-in cases and the compiled CLI recording/replay case cover opt-in, retry, split, both bounds, no-store, secrecy and concurrent writers. The scalar prior-failing case also fixes attempt collection in the ordinary single-input pipeline, reusing the existing sink. Relevant format, policy and package Clippy are required before this constituent push. Whole native/host integration, fresh High review and root landing gates remain open.

Source grows 545 nonblank Rust lines (147730 to 148275) for the bounded writer and actual storage/CLI regressions. Checked existing original-body transaction, conversion durability and attempt collection for duplication; reuse their transaction, folder sync, pipeline attempts and unchanged options rather than adding a second recorder or control.

Native foundation update: actual call/request identities, attempts on successes and started failures, partial input/output facts, command-only timing and bounded opt-in recording sidecar are implemented through the ordinary transport and store. Empty/current replay attempts remain truthful; no transport identities or raw headers enter answer bodies or timing history. Public complete-call/error serialization and CLI details/facts are implemented. Root whole review/landing and host execution adoption remain open.
