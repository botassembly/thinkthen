# 0060: Return each record with its answer

Date: 2026-09-22

Status: landed

## Result

Default streamed output from `decide`, `choose`, `tag`, and `score` now keeps the parsed record under `input` beside its answer under `value`. Lines become JSON strings, JSONL retains its value, and CSV or TSV input becomes an object of string cells. Output stays compact JSONL.

Streamed `annotate` keeps enriching object records, including CSV and TSV rows. Lines, JSON scalars, and JSON arrays now retain their input beside the named answer map. One-document annotations, detailed output, raw output, quiet output, `filter`, `rank`, and `find` keep their established behavior.

Request bytes, digests, recordings, cache behavior, order, worker width, and exit behavior did not change. The Rust ceiling rose from 28,112 to 28,528 nonblank lines for one production serializer, host-neutral shared cases, the compiled framing matrix, compatibility tests, help proof, and updated exact assertions.

## Review and proof

Independent design review rejected the first draft because it could wrap one-document annotations, put CLI table parsing in the host-neutral conformance layer, and call the raw view JSONL. The repaired design limits wrapping to streamed records, separates typed serializer cases from framing tests, and preserves plain-text raw output. The same reviewer accepted it.

Independent code review rejected the first implementation because shared framing help falsely promised the wrapper for `filter`, `rank`, and object annotations. The repair names the three output rules and adds a compiled help test over `decide`, `annotate`, and `filter`. The same reviewer accepted it.

The final install, lint, test, and specification rungs passed with the key and base-address variables unset. The test rung passed 185 library tests, 234 backend tests, and every other compiled suite. The specification rung passed all 19 green how-tos. The ratchet was exact at 28,528, and `git diff --check` passed. No outside network or paid call ran.
