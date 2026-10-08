# 0470: Bound DuckDB complete-file admission

Status: OPEN. The review confirmed that file descriptors accumulate before native admission; memory exhaustion is a traced risk, not a reproduced crash.

Milestone: 0.2

Owner: builder.
Severity: medium resource correctness.

## Outcome

DuckDB complete-file calls cannot accumulate unlimited decoded content before engine limits and cancellation apply. Valid record-stream behavior and native whole-set limits remain consistent with other surfaces.

## Evidence

- Starts from: 0462 SQL review at 7ea661c1e; cpp/src/complete_files.cpp accumulates every reader descriptor into one string before the native call. files_manifest.cpp caps path bytes only. Native text/image limits are per item, so repeated admitted paths do not bound total retained content.
- Keeps: DuckDB-authorized FileSystem access, order, duplicates, original locations, secrecy, cancellation and native per-item/whole-set limits. No invented hard cap that needlessly rejects otherwise supported record streams.
- Changes: Design and build bounded streaming of file descriptors into native admission, or a reviewed aggregate limit consistent with the existing whole-set contract where streaming cannot apply. Name ownership/thread/cancellation boundaries explicitly; do not read all content before checking limits. Specify any new limit as part of the public contract.
- Proof: Fresh design/ticket and code reviews, with High review for unsafe ownership changes. Existing reader/complete/cancellation fixtures plus a deterministic bounded-admission case that fails on the accumulator path. No memory-exhaustion campaign, paid calls, hosted workflow or new proof framework.
- Defers: No DuckDB feature redesign, provider limit tuning or release management. Process exhaustion is inferred and must remain labeled that way.
