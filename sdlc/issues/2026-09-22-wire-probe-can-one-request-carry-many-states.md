# Wire probe: can one request carry many states?

Status: Open

Answers `2026-09-21-one-state-per-request-caps-table-scale-classification` with one paid probe. Ian authorized the spend on 2026-09-22 at a one-dollar ceiling; the probe used about one hundredth of a cent of it.

## The question

Does the System One endpoint accept multiple states (texts) in one request, at what price, and at what rate? The recorded contract sends one evidence string per request — the `systemone` adapter writes `state` as one string (`crates/thinkthen/src/core/adapters/systemone/request.rs:148`), and `specification/annotate.md:108` keeps one record per request. At the documented 1,200 requests a minute (`specification/records.md:141`) and a fixed floor near 256 billed input tokens per request, that caps table-scale classification near 20 rows a second and about $1.08 per 100,000 rows. The open issue could not reconcile that arithmetic with a partner's published 2,484 rows a second at $0.50 per 100,000 rows.

## The method

Two launches through `sdlc/scripts/live`, which serializes paid work and precharges the declared maximum: a smoke launch of one control request at a declared 5,000 tokens, then eight requests at a declared 60,000. Every request went to `https://api.typesafe.ai/v1/systemone` with `model: jev-latest`, one at a time with half a second between calls. No burst was attempted, so the rate question is not tested by design. Every reply named `jev-1.13.0`.

The job, the exact request bodies, and the raw rows sit in `/home/ian/workspace/experiments/thinkthen-wire-probe-2026-09-22/` (`post.py`, `job.sh`, `out/run-smoke.jsonl`, `out/run-full.jsonl`). The harness read the key from the environment; it was never printed, logged, or written. No response header value was kept. A search of the job and the two rows files for `bearer` and `authorization` in any case finds only the two header-name strings in `post.py`, and no value anywhere.

Three made-up classification texts: T1 (business), T2 (sport), T3 (business). The single-state question was "Is this text about business or finance?". The packed shapes asked one condition, "The text is about business or finance.", with one row question per state.

## The shapes tried, and what each answered

| # | The one field that differs from the control | Status | Answers | Input tokens |
| --- | --- | --- | --- | --- |
| 1 | control, `state` one string (T1) | 200 | `q1` 0.99 | 285 |
| 2 | control (T2) | 200 | `q1` 0.01 | 288 |
| 3 | control (T3) | 200 | `q1` 0.99 | 284 |
| 4 | `state` as a list of two strings | 200 | `q1` 0.50 — one answer | 304 |
| 5 | `states` as a list of two strings | 400 | none | none reported |
| 6 | `states` as a map of two strings | 400 | none | none reported |
| 7 | `state` as one object, `condition` plus two rows, questions `r0`, `r1` | 200 | `r0` 0.98, `r1` 0.01 | 372 |
| 8 | the same object with three rows, questions `r0` through `r2` | 200 | `r0` 0.98, `r1` 0.01, `r2` 0.98 | 416 |

Shape 7 and 8 carried exactly the layout of `realZachi/pg-jev` 0.2.0 and both DuckDB ports:

```json
{"model": "jev-latest",
 "state": {"condition": "The text is about business or finance.",
           "rows": [{"text": "..."}, {"text": "..."}]},
 "questions": {"r0": {"type": "noul", "instructions": "Does the record `rows[0]` satisfy the condition stated in `condition`?"},
               "r1": {"type": "noul", "instructions": "Does the record `rows[1]` satisfy the condition stated in `condition`?"}}}
```

Shapes 5 and 6 answered `{"detail": {"error_type": "api_usage_error", "message": "Invalid request."}}` with no usage block. The plural field does not exist under either spelling.

Shape 4 is the one to watch. The endpoint did not refuse a list `state`: it accepted the request, billed it, and returned ONE judgment at 0.50, the midpoint. The bill grew by 19 input tokens over the first text alone, roughly the second text's own length with JSON punctuation, so the second text entered the input while only one answer came back. A caller who sends a list believing it packed states gets one ambiguous judgment and no way to tell which state it judged. What the model read for a list `state` is unchecked. The structured object is the only form that returns one answer per state, and it names each answer.

## What it costs

The ~256-token floor is paid once per request, not once per state.

| Shape | Requests | Input tokens | Cost at $0.042 a million |
| --- | --- | --- | --- |
| two states, one request each | 2 | 573 | $0.000024 |
| two states as rows in one request | 1 | 372 | $0.000016 |
| three states, one request each | 3 | 857 | $0.000036 |
| three states as rows in one request | 1 | 416 | $0.000017 |

Two rows in one request cost 35 percent fewer input tokens than two single-state requests, a factor of 1.54. Three rows cost 52 percent fewer, a factor of 2.06. The marginal packed row bills 87 tokens for the second and 44 for the third, against about 285 for a one-state request. The per-row bill falls from 285.7 tokens at one row a request to 186.0 at two, 138.7 at three, and 86.4 at ten rows on experiment 208's short rows (`experiments/208-thinkthen-row-packing/RESULTS.md`).

The spend: declared 65,000 tokens across the two launches, billed 2,234 input tokens and 198 output tokens, which is $0.000094 at the recorded input price. The ledger moved by the two declarations, from 408,953,418 to 409,018,418 charged tokens. The smoke launch carried the T1 control once more; its 285 tokens are inside the total and not in the table above.

Rate: the documented limit stays 1,200 requests a minute. Every reply, 200 and 400 alike, carried the same header names: `date`, `server`, `content-length`, `content-type`, `x-typesafe-request-id`, `x-envoy-upstream-service-time`, `connection`. None names a rate limit or a quota. No 429 came back. Nine sequential requests say nothing about the limit; the rate question is unchecked by this probe.

## Verdict

The batched form exists, and it is the structured `state` object: one `condition`, one row object per state, one named question per row, one answer per row, and the request floor paid once — two rows billed 372 input tokens against 573 for two single requests, three rows 416 against 857. The plural spellings do not exist, and a list `state` is worse than refusing, because it returns one silent, ambiguous judgment; the engine must never send one. If the packing decision is taken, the engine must encode this object in the `systemone` adapter, name the questions `r0` through `rN`, map answers back by position, and keep the recording and cache keyed by the whole request, because a row's answer depends on its neighbors (regrouping alone moved 34 answers of 1,000 in experiment 208). The width, the off-by-default rule, and which verbs may pack stay the ADR decision the databases page already frames, and no page may claim table-scale throughput until that encode exists. For the arithmetic in the open issue: at ten rows a request the documented limit would carry about 200 rows a second, the per-row bill measured 86.4 tokens on 208's short rows, and that is about $0.36 per 100,000 rows, below the partner's $0.50, so their price is consistent with a packed lane. Their 2,484 rows a second still needs about 124 rows in one request, a width no measurement here tested. The rate and the partner-lane question stay unchecked.
