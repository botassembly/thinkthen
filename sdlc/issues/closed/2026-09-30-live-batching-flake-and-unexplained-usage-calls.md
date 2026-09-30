# One live batching request failed once, and about 1,000 calls on the usage counter are unexplained

Status: closed 2026-09-30 by the queue batches Quick Fix. Both findings are paid: the unexplained calls came from `test-stress` (closed issue `2026-09-30-test-stress-writes-the-real-usage-totals.md`, fixed in `87602e2ad`), and ticket 0341 (`f0308dd9d`) fixed the one mechanism found that gives a lone exit 4 that passes on rerun. A later live exit 4 kept with `--details` and stderr is a new issue.

Resolution: paid by `87602e2ad` and `f0308dd9d`.

## What was seen

1. One live batching request exited 4 and passed when run again. Exit 4 is a backend failure. The bench kept no response body, so the cause is unknown: a service error, a timeout or a dropped connection.
2. The shared usage counter shows about 1,000 calls that no known run explains. They cost about $0.02.

## What to find out

- Whether the exit 4 came from the service or from thinkthen's retry rules. The loopback tests cover retries, server errors, status 520, oversized replies and dropped connections, so a live repeat under `--details` would show the cause if it recurs.
- Which runs wrote the extra calls. The count-only usage totals (ADR 0113) and `status --json` `.usage.total.requests_sent` can be compared with the bench's own logs for the same window.

Any paid repeat needs Ian's authorization and goes through `sdlc/scripts/live` under a token cap.

## What the investigation found

Read-only, 2026-09-30. No paid call, no key read, no build or test run. `live --status` read `limit_tokens 476000000`, `charged_tokens 465112291`, `remaining_tokens 10887709`. The live ledger records reservations only, so it cannot name a run. The real usage folder was not read.

### The unexplained calls: likely test-stress

The usage totals count loopback sends as well as real ones. Every surface built with `from_env` adds its sends and reply tokens to the machine's one `thinkthen-usage` folder (ADR 0113 section 2). So "calls on the usage counter" includes any process on the machine that sent to a loopback test backend with the real `XDG_CACHE_HOME` or `HOME`.

The isolation holds everywhere but one place:

- `sdlc/scripts/test` and `surfaces` take `usage_guard` (a decoy usage folder that fails the run if written). Every `libraries/*/check.sh` and `databases/*/check.sh` takes its own `usage_home`. `spec` and `release-smoke` take it too.
- Integration tests that call `from_env` run it in a child process from a cleared environment with a scratch `HOME`: `public_env.rs`, `key_address.rs`, `ca_bundle.rs` and `named_backends/builder.rs`.
- `public_batches` and the other stress selections build with `Engine::builder()`, which writes no usage.
- **`sdlc/scripts/test-stress` takes no usage step.** It exports the fake Polars key and a loopback address, then runs the `polars_throttle` target in-process. That target builds each engine with `EngineBuilder::from_env()`, so its sends land in the real totals. A `--run` adds about 400 sends; a `--check` about 15. Two or three `--run` campaigns during the 0304 and load-flake work would give about 1,000. Only the loopback backend's `/arm/full/v1` arm reports token usage, and this test uses the delay and held arms. So the $0.02 may come from other runs; the count rests on sends.

No test, script, demo or library default was found that sends to a real host. Demos and `spec` replay recordings, and replay adds nothing to the totals.

One experiment would confirm the cause without a paid call. Note the real month file's `requests_sent`, run `sdlc/scripts/test-stress --check` once, and read the count again. It should rise by about 15. Run it when the machine is idle, since it builds the Polars target.

### The exit 4: still unknown

No log or response body survives, so the source cannot be read now. Exit 4 is a backend failure after thinkthen's retries. The next live bench run should pass `--details` and keep stderr, so a repeat names its status or transport error. Any paid repeat needs Ian's authorization and goes through `sdlc/scripts/live` under a token cap.

### One cause that fits the exit 4

Ticket 0341 found one mechanism that gives exactly this symptom: a single exit 4 that passes on rerun, with no service error. A backend closes an idle keep-alive connection just as the next request goes out on it. The request fails as a close before a reply and is not sent again, by the 0089 rule. Under ureq's old 15 second idle age, a backend with a keep-alive wait from 2 to 15 seconds could hit it after any pause of that length, such as a retry wait or pacing. 0341 cut the idle age to one second, which closes that range. Nothing here shows the bench hit it, since no stderr survives. The next live run should still keep stderr under `--details`. A repeat that prints `the backend closed the connection before a reply` would fit this cause.
