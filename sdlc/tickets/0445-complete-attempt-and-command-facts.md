# 0445: Complete attempt observations and command facts

Status: in progress. Native implementation in lane0 on ticket/0443-native-complete-results; host adoption and final landing checks remain open.

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
