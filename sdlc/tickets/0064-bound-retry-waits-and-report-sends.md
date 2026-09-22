---
flow: build
priority: 35
opens: crates/thinkthen specification sdlc/issues sdlc/planning sdlc/ratchet.json
---

# 0064: Bound retry waits and report sends

Status: landed

## Outcome

A backend cannot make one retry wait longer than the `--timeout` value the user chose. Every detailed successful result says how many HTTP requests were sent to produce it, so retries and cache answers agree with `thinkthen status`. A clean raw TCP test settles whether a peer that closes before response headers already fails promptly.

## Current facts and decisions

The HTTP client bounds each attempt with `--timeout`, but sleeps for exponential backoff or the backend's `Retry-After` value outside that bound. A header-selected sleep can reach 60 seconds after a two-second attempt. The engine already calls the usage counter before every attempt, but a result carries provider token counts and logical request digests only. ADR 0017 requires the send count in detailed output. A filed loopback run classified a connection closed before response headers as a timeout after the whole deadline, but that Python stand-in did not prove that every accepted socket handle closed. The pinned HTTP client maps a clean response-header end-of-file to an immediate structured error.

`--timeout` remains an attempt timeout. An exponential wait is `min(the doubled backoff, --timeout)`. A header wait is `min(the parsed Retry-After value, 60 seconds, --timeout)`. The total run can still include the initial attempt, the configured number of retries, and the waits between them. The help and backend specification state that boundary. No progress line is printed while waiting.

Detailed `Meta` and `AnnotateMeta` always serialize `requests_sent` as a JSON nonnegative integer. It appears immediately after optional provider `usage` and before `replayed`. A live first-attempt answer carries `1`; an answer after two failed attempts carries `3`; a cache or replay hit carries `0`. `meta.requests` remains the ordered list of logical request digests and retries add no digest. Provider `usage` remains the successful response's reported tokens and is not multiplied by the attempt count.

Storage modes use one rule. Record-only live success reports its attempts. Paired `--record/--replay` reports `0` on a hit and the live attempts on a miss. Explicit replay, `--cache`, `THINKTHEN_CACHE`, and the default cache report `0` on a hit. Packed `annotate` sums attempts across its group requests, including a row whose groups mix a replay hit and a live answer. Bare output, request bytes, cache entries, recording digests, and exit meanings do not change.

The engine HTTP result owns the successful attempt count. The command passes it through `Answered` and `RequestMeta`; serializers only name it. The persistent counter from ticket 0063 remains the independent process total and continues to count failed runs whose result never prints.

A deterministic raw TCP test accepts the request, reads through the request body, and closes every accepted-socket handle before writing response headers. With no retries and a multi-second timeout, the command should promptly print exactly `thinkthen: the backend closed the connection before a reply; try again or change --max-retries`. If the pinned client already satisfies the case, the test and issue closure finish this finding. If it waits or returns an unclassifiable error, implementation stops and returns to design review before adding a heuristic, changing the client, or adding a dependency.

Ian can overturn the `requests_sent` field name and the decision to cap each wait rather than the whole multi-attempt exchange. Both choices use the vocabulary and timeout boundary already public in status and help.

## Scope

Bound exponential and header-selected retry waits by the attempt timeout; carry successful per-result sends through live, cache, replay, recording, record streams, find, and packed annotation; run and pin the clean close-before-headers experiment when it already behaves correctly; update the result and backend specifications, ADR 0017 if its words need clarification, the owning issues, and the queue.

Excluded: a whole-run deadline, cancellation or SIGINT, width and fork repair, retry progress output, prices, token multiplication, cache address warnings, `meta.replayed` renaming, dependencies, paid calls, and changes to request or recording bytes.

## Acceptance

- A loopback 429 with `Retry-After` above `--timeout` sends its retry after the timeout-sized wait. A shorter header retains its value. The header keeps the existing 60-second ceiling. An exponential backoff that grows above `--timeout` is capped too. Zero retries sleep zero times. Tests use short hidden test durations, count requests, and prefer synchronization over wall-clock-only assertions.
- Exact detailed JSON pins `requests_sent` immediately before `replayed` for first-attempt live success, retry success, default and named cache hits, explicit replay, record-only live success, paired record/replay hit and miss, record streams, find, and packed annotation. A mixed replay/live multi-group annotation proves the sum. Bare output stays byte-identical.
- The two-send process total and the per-result counts reconcile for successful results. Provider token counts and logical request arrays keep their existing meanings. Failed attempts without a result still appear only in the process and persisted status totals.
- The raw peer-close test reads the request and drops every accepted-socket handle. If the pinned client returns promptly, it pins the exact premature-close sentence above, repeats no key, evidence, address, or client text, and follows the existing retry rule. A peer that stays open still reaches the timeout diagnostic. Any contrary result returns the ticket to design review.
- Existing status counts, token counts, logical request arrays, ordering, exits, request bytes, cache and recording entries, replays, and secrecy sweeps stay compatible. The owning issues, ADR, specification, and queue agree. The four offline gates and `git diff --check` pass.

## Dependencies

Tickets 0055 and 0063, ADR 0017, and the open retry-wait, early-close, and retry-visibility issues.

## Complexity

- Contract: 2
- State and timing: 2
- Reach: 1
- Proof: 2
- Cost of error: 1
- Total: 8
- Minimum level floor: public result shape and retry timing
- Final level: 3
- Reasons: one public metadata field and one timing rule must remain consistent across retries, cache and replay, packed annotation, status counters, and transport failures.
- Selected model: `gpt-5.6-sol` with medium reasoning.

Re-score if the work adds a whole-run deadline, cancellation, a dependency, a client replacement, or another persistent field.

## Review

Independent design review rejected the first draft because it did not freeze the public field's position, could erase the existing 60-second header ceiling or miss exponential backoff, left record/replay and mixed annotation counts ambiguous, and treated the early-close report as proven. This whole rewrite closes those points and keeps the level-3 Sol Medium route.
