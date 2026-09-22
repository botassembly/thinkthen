# 0071: The two narrow exceptions granted in the review waves

Date: 2026-09-22

Status: landed

## Result

Two lane boundaries were crossed by explicit, narrow permission. Both are recorded here because the permission lived only in the session that granted it.

**1. The C lane's two header comment spots.** The C lane's rule was never to touch `contract/`. The header's lifetime promise ("the message ... lives until the next call on the same engine") became false once the door moved to per-thread error slots, and the doc change belonged with the code that makes it true. The permission was: edit exactly the two comment locations — the line 27-28 lifetime promise and the `thinkthen_error_message` doc comment — and nothing else in `contract/`, with a diff check proving comments-only. Landed in `be8374f`; the header diff touches only those two comment blocks, and the header now states the per-thread truth.

**2. The phase-3 permission to edit `databases/duckdb/tools/conformance.py`.** The no-touch rule existed to keep the conformance lane from colliding with the DuckDB lane mid-flight; that lane had completed and pushed, so the conflict was gone. The new shared cases (a NULL record, a multi-record annotate, a numeric score member) crashed the driver, which could not parse them. The permission was: extend only the driver's `decide_many` and `annotate` branches (plus what the selftest needs), implement the shapes rather than skip wherever SQL can spell them, keep reasoned skips with written reasons, and extend `conformance_selftest.sh` so the corrupted-expectation proof still fails and at least one new shape also fails when corrupted. Landed in `c83add5` (the driver branches) and `2143d7e` (the cases and the extended selftest).

## Why this record exists

The second review found these permissions durable nowhere; a later reader of either file could not tell who allowed the crossing or why the rule held otherwise. Both exceptions were scoped to comments or to named branches, and both kept the discipline the surrounding lanes had installed.
