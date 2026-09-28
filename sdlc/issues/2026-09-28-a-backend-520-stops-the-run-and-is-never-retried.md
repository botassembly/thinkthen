# A backend 520 stops the run and is never retried

Status: Open. Filed 2026-09-28 from local experiment 345, finding 2. Severity 3: a sharp edge, with a wrong refusal message beside it.

## What happens

`engine/error.rs:89` retries `[429, 500, 502, 503, 504, 529]`. The Cloudflare range 520 to 524 is not in the list, so one 520 is a permanent failure and the whole run stops:

```text
thinkthen: the backend answered with status 520
thinkthen: stopped at record 61; 60 records finished
```

Live on 2026-09-28 at `--batch 1` over the 306 bench titles, exit 4. The hosted address sits behind Cloudflare and returned 520 twice within eight runs that hour, alongside two per-attempt timeouts.

The timeout arm carries the same gap from the other side. A batched request that exceeds the 30 s attempt budget fails with `the backend timed out; lower --batch or --max-request-bytes, or increase --timeout`. A timeout is latency, not size, and the size advice sends a reader the wrong way. Review finding D18, refusal phrases that do not fit the case, covers the wording.

## Evidence

Local experiment 345, sections 5 and 6. `RETRIED` at `crates/thinkthen/src/engine/error.rs:89`. The recorded stderr lines are in `experiments/345-live-verification-2026-09-28/raw/speed2.err` and `raw/d.err` on the machine that ran them.

## Direction

Retry the transient Cloudflare family 520 to 524 on the same backoff as 5xx, and split the timeout advice from the size advice. A `Retry-After` rule already caps waits, so the change is the status list and one message.

## Done when

A 520 is retried on the ordinary backoff, and the timeout message names the timeout, with a test for each.
