# One live batching request failed once, and about 1,000 calls on the usage counter are unexplained

Status: open. Reported by the Beatles Bench team on 2026-09-30 from live runs. Owner: the queue owner, from logs; no paid call was run for this issue.

## What was seen

1. One live batching request exited 4 and passed when run again. Exit 4 is a backend failure. The bench kept no response body, so the cause is unknown: a service error, a timeout or a dropped connection.
2. The shared usage counter shows about 1,000 calls that no known run explains. They cost about $0.02.

## What to find out

- Whether the exit 4 came from the service or from thinkthen's retry rules. The loopback tests cover retries, server errors, status 520, oversized replies and dropped connections, so a live repeat under `--details` would show the cause if it recurs.
- Which runs wrote the extra calls. The count-only usage totals (ADR 0113) and `status --json` `.usage.total.requests_sent` can be compared with the bench's own logs for the same window.

Any paid repeat needs Ian's authorization and goes through `sdlc/scripts/live` under a token cap.
