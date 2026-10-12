# RequestRows serves rank, rank-set, find and relate with caller-owned originals, and the whole-set records wrappers retire

Status: open.

Kind: idea
When: after 0.2.0 ships

## Problem

RequestRows serves rank, rank-set, find and relate with caller-owned originals, and the whole-set records wrappers retire. One request path for every function, and about a dozen fewer public Engine methods.

The wrappers still public after 0558 include `relate_complete_with`, `try_annotate_with`, `try_rank_with` and `try_find_with`. The last three stayed public only because the C library calls them. None of these wrappers shipped in 0.1.2. Under the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md), unshipped public API should go before 0.2 freezes it. If 0561 removes their last C callers, 0561 deletes them in 0.2 and this idea shrinks to whatever is left.
