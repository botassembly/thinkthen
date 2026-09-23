# 0056: Keep details inside the selection and include the band edge

Date: 2026-09-21

Status: landed

## Result

`filter --details` now prints detailed results for kept records alone. It applies the same selection as bare `filter` and keeps input order. `rank --details --top N` remains limited to the same first `N` records as bare rank.

A band now reads probabilities in three exact intervals: below `LOW` is no, from `LOW` up to but not including `HIGH` is not sure, and `HIGH` or above is yes. The shared conformance cases pin both exact edges. The filter, result, and threshold specifications now state the same rules as the binary.

The recording helper for how-to 43 now uses `decide --details`, because that helper needs every answer while the published lint pipeline still uses `filter` to keep only violations. Request bytes, result shapes, scheduling, exit codes, and paid behavior did not change. The Rust source ceiling fell from 26,012 to 26,009 nonblank lines.

## Review and proof

Independent design review rejected ambiguous threshold wording and a citation to an open issue instead of the durable ruling. The corrected ticket states all three intervals and cites the build queue. The same reviewer accepted it.

Independent code review rejected the first implementation because its shared conformance cases did not exercise the exact band edges and one option sentence still promised details for every record. The repair puts the exact low and high values in the shared cases and says “per kept record.” The same reviewer accepted the final implementation and found no regression.

Red tests reproduced both faults before the source changed. Detailed filter output included a rejected record, and the exact low edge returned no instead of not sure.

With the key and base-address variables unset, `install`, `lint`, `test`, and `spec` all exited zero. The test rung passed 172 library tests, 210 backend tests, the command edge suites, two doctests, and the shell checks. The spec rung passed 26 specification checks, seven transform checks, all replay checks, and 19 green how-tos. `git diff --check` passed. No paid call ran.
