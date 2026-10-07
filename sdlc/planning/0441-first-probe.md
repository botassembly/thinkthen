# 0441 first text probe

Status: prepared, not executed. Root owns calls, interpretation, review and landing. This disposable job reduces the wire/limits risk in [0441](../tickets/0441-openai-decisions-backend-preview.md); it is no gate, benchmark or proof framework. No paid call or observed reply fixture exists from this design step.

Use the root-local handoff job `thinkthen-0441-first-probe.sh`. Its first line is exactly `#!/bin/sh`; Python uses the standard library only. `--prepare NEW_OUTPUT_DIRECTORY` writes the finite request manifest without reading a key or opening a connection. `--run NEW_OUTPUT_DIRECTORY` sends only the predeclared requests. Both require a fresh caller-named directory and overwrite/delete nothing. Root has the prepared requests and the script separately from this public planning commit. Actual replies must be collected by root before adapter fixtures are promoted; prepared requests are examples, never claimed observations.

## Admission and spend

Reserve $2 within Ian's $20 aggregate, not $2 per run or lane. Root checks aggregate remaining authority, the applicable Decisions tariff/multipliers and `sdlc/scripts/live --status` first. With gpt-6-luna the admitted [guide](https://developers.openai.com/api/docs/guides/decisions) prices base input at $0.10/M, with no output/cache read/write charges. Regional and long-context multipliers may apply. Use the fixed global endpoint and no region/tier controls; do not assume another endpoint's pricing. If applicable pricing or aggregate headroom cannot be established, do not run.

The fixed job admits at most 30 requests, sequentially, each at most 65,536 ASCII JSON bytes. Worst admitted aggregate bytes are 1,966,080; `encoded-body-bytes-908-v1` admits at most 59,507 estimated input tokens each and 1,785,210 total. The live helper reserves1,800,000 tokens once, including all diagnostics/repeats. The prepared manifest actually contains 62,934 bytes across 30 requests, largest 35,271 bytes and 57,158 estimated tokens. Base-price arithmetic on the worst local estimate is $0.178521; actual cost is pending observed usage. The $2 allocation retains room above that estimate and must cover uncertainties and multipliers.

These are exact local request/admission bounds, not a provider-enforced token or monetary cap. The API schema provides no max-input-token request control. Root must not report the reservation as billed usage or a guaranteed billing bound. The job checks returned usage against its local bounds and nominal budget, stops on any missing/invalid usage or bound violation, and holds the remaining $2 allocation when a failed/uncaptured call cannot be priced. Failed calls are not assumed free. Root reports actual usage-priced cost and any unknown charge separately, and never continues by resetting counters or reusing an output directory. A further run requires root to reconcile all earlier attempts and allocate remaining authority explicitly.

No automatic retry, refusal splitting, redirect, proxy-environment forwarding, search for a maximum, large benchmark, paid image, endpoint substitution or model fallback occurs. Timeout is 30 seconds per attempt; responses stop at1MiB plus8 times actual request bytes. A rate limit, HTTP failure, transport/read failure or malformed billing usage stops immediately; unrun cases stay unobserved. A saved diagnostic refusal does not authorize another attempt or advancing the size.

## Fixed cases in order

| Requests | Case | What root inspects |
| --- | --- | --- |
| 1 | decide refund predicate | probability, actual model, complete usage |
| 1 | choose billing/shipping/technical, described | full string-valued distribution, winner and confidence |
| 1 | score cosmetic/workaround/blocked, described | ordered labels/indices, total, weighted score versus vendor score |
| 1 | tag20:20 independently named label predicates | answer names/order, logical grouping and accepted question count |
| 1 | find100:100 described units, unit042 refund | accepted option count, exact keys and totals; this is a choice wire diagnostic, not a new find planner |
| 20 | individual items, alternating refund and delivery | one scoped predicate per item in its singleton framed input |
| 1 | same 20 items in shared framed input |20 scoped predicates; compare each probability/decision with its individual call |
| 2 | exact repeats of initial decide body | three observations of identical bytes; differences/agreements establish only this sample |
| 1 |512 short predicate questions | finite larger-count diagnostic; accepted count or HTTP/error body; no binary search |
| 1 |512 short string choices | finite larger-count diagnostic; accepted count or HTTP/error body; no binary search |

The two 512 cases exceed the earlier successful target counts; neither is known to exceed a vendor maximum. If accepted, stop there. If refused, the count/code describes that input, not an established boundary. The first failure may leave the second diagnostic unrun. String-length limits in the reference are not question/choice maxima. No large evidence payload is needed to discover the diagnostic shape.

For each scalar/packed response, inspect nonblank actual model, reported nonnegative integer usage, one answer per named ordered question, finite [0,1] members, distribution totals and typed option identity. Record the actual OpenAI rounding before proposing a tolerance amendment; do not renormalize or invent missing probabilities. If matching batch/scalar answers differ, report each difference; batching promises neither semantic equality nor determinism. A repeat agreement does not establish global determinism or mutable-alias cache freshness.

## Safe hand execution and saved files

The job reads only THINKTHEN_API_KEY from the inherited environment. Root may map the named OPENAI_API_KEY into that variable purely in memory when invoking the existing live helper, using a small Python subprocess launcher with an environment copy and exec, without printing either variable. Do not source a profile, inspect a credential file, put a key in command arguments, turn on shell tracing or capture environment dumps. `live` still requires the checkout's prebuilt debug binary even though this job sends raw Decisions bodies; reuse it, or let root arrange the ordinary prerequisite. This design runs no build.

After root's checks and in-memory mapping, the paid invocation is:

```sh
sdlc/scripts/live --max-tokens 1800000 FIRST_PROBE_JOB --run NEW_OUTPUT_DIRECTORY
```

FIRST_PROBE_JOB denotes the disposable shell job. The helper validates and appends the common-directory ledger; no script reads, copies, edits, removes or replaces that ledger manually. Run `live --status` to audit it. A launch failure retains its charged reservation. The job cannot establish that it was launched through live; root is responsible for using the authorized door.

Request headers are only Content-Type and in-memory Authorization. Body files are numbered `CASE.request.json` and `CASE.response.json`, with exact bodies and no headers/key. The script withholds a body containing the effective key and prints only fixed failure text, case-independent counts, HTTP status and priced totals. Generic urllib exception text never becomes a diagnostic. All inputs are synthetic public text. Body captures use a caller-named private output directory; never commit arbitrary backend error text before root checks it.

Separate `transport.json` contains only case, status and bounded printable values for this closed response-header allowlist: x-request-id, retry-after, retry-after-ms, x-ratelimit-limit-requests, x-ratelimit-remaining-requests, x-ratelimit-reset-requests, x-ratelimit-limit-tokens, x-ratelimit-remaining-tokens and x-ratelimit-reset-tokens. No response-header dump, cookies, authorization, organization/account headers or request-header recordings. Reject values containing the effective key; absent headers stay absent. These observations are not fixture bodies or policy controls. The absence of retry headers on a successful request does not establish retry behavior.

`admission.json` contains only public bounds and prepared sizes. `cost.json` counts captured successful input usage once at the Decisions base tariff and labels multiplier uncertainty. Root reconciles failed/unknown attempts separately, reports actual charged basis and total under the $2 allocation, and admits safe reply bodies as saved adapter request/reply fixtures. No replies are fabricated, and no receipt/provenance tool is created. Test secret/error paths later with fake-key loopback exchanges, not extra paid invalid-key calls.
