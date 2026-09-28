# ADR 0083: Run facts name the stop

- Status: Accepted for ticket 0170 on 2026-09-27. Ian can overturn these choices.
- Date: 2026-09-27

## Context

ADR 0048 item 10 promises one `thinkthen.run/1` line when `--facts` is set, but its original field list cannot separate a retryable 503 from a permanent 401 or a missing key. A filtered row or a row dropped by `rank --top` still consumes work. ADR 0052 adds retry attempts to the counters, and ADR 0051 adds one split for a too-large batch.

## Decision

1. The command's existing exit codes and human stop lines stay. A failed run's facts object adds `stopped` with `cause` and `retryable`. It adds `at` when the stop line names a record and adds `status` only for the `status` cause. A signal stop has no `at`. Exit 6 is a finished partial result and has no `stopped`.
2. The stable causes are `usage`, `local`, `no_key`, `transport`, `status`, `too_large`, `reply`, `backend`, `cancelled`, and `defect`. A `status` is retryable only for 429, 500, 502, 503, 504, and 529, using the same internal status rule as the engine. Transport reads false because the attempt may have reached the backend. A missing key remains exit 4 and reads `no_key`.
3. Process counters supply `requests_sent`, `retries`, and `cache_answers`, including attempts and rows hidden by output selection. Finished input records count where the command accepts completion. Token totals appear only after at least one live reply and only if every live reply reported usage. A model appears only if every received live or stored reply named the same one. These presence facts stay in memory and do not change the usage file.
4. One compact line follows human diagnostics and any usage warning, before the command re-raises a stopping signal. A second signal can still end the process before it prints. Without `--facts`, output and exits stay unchanged. The option is available on the ten asking verbs through their common command arguments and appears in long help only.
5. Under `--details`, a batched row's `meta.batch` describes its whole batch, while per-row usage and request counts remain shares. A split half describes that half and carries `split:true` even if it holds one record. A batch of one record with no context omits `meta.batch`, preserving its old bytes.

This amends ADR 0048 item 10 and builds its item 9 for the command. ADR 0052's retry count and ADR 0051's split rule remain in force. Library result facts belong to B12a and later tickets; SQL facts remain deferred.
