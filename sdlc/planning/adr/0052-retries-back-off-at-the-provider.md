# ADR 0052: Retries back off at the provider

- Status: Proposed through ticket 0155. Ian ruled the default of 3, the provider-level backoff, the fixed throttle and separately counted retries on 2026-09-26. It becomes Accepted when the fresh ticket review accepts ticket 0155. Ian can overturn each item
- Date: 2026-09-26

This ADR amends the retry rule of `specification/backends.md`, "The request", the usage counts of ADR 0034, and ADR 0048 item 10's run facts. Ticket 0155 builds every item but the part of item 8 that ticket 0149 builds.

## Context

A retried status is 429, 500, 502, 503, 504 or 529. Today each request that meets one waits on its own timer and sends again. The wait doubles from one second, or follows the reply's `Retry-After-Ms` or delta-seconds `Retry-After` header, capped by 60 seconds and `--timeout` (`backends.md`, "The request"). `--max-retries` defaults to 2, with no reason recorded (`settings.md`, the Retries row).

While one request waits, the others in flight keep sending. A backend that answered 429 or 503 because it is overloaded then gets more requests, each of which may fail and wait on its own timer. The run meets the overload once for every request in flight.

The throttle already bounds attempts, not requests. `engine/http.rs::Client::post_observed` takes a permit from the process's width gate for each attempt and drops it before any retry wait. The unit test `a_retry_gives_its_permit_back_for_the_wait_and_takes_a_new_one` pins this. A replay or cache answer takes no permit (`a_replayed_answer_takes_no_permit`).

`requests_sent` counts every HTTP attempt, retries included: on a row's `meta`, in the usage totals of ADR 0034, in `thinkthen status`, and in the library's `Counters`. Nothing reports how many of those attempts were retries.

No command option limits requests. The library's request limit, `max_requests`, refuses a call over more records than its number before anything is sent (`public/bulk.rs::within_limit`). The SQL surfaces' process request total refuses a call once the process's `requests_sent` reaches its number. It is checked before each call, so it already counts retries, and a call's own retries can pass it.

## Decision

1. **Three retries by default.** `--max-retries` defaults to 3 on the command, and the engine builder's fixed value becomes 3 on the libraries and SQL. Ian ruled it on 2026-09-26. With the doubling wait, three retries ride out an overload of about 7 seconds (1, 2 and 4) where two rode out about 3. A retried status means the backend answered and may have billed the attempt (ticket 0089), so a request that fails every attempt may now be paid four times instead of three. `check` keeps using the default, so its four probes send at most sixteen attempts.
2. **One backoff gate for each process and each address.** The gate belongs to the resolved posting URL, the exact string a request is posted to. Every engine, call and thread in one process that posts to that URL shares it. A forked child starts with every gate open, as it starts with a fresh width gate.
3. **What closes the gate.** An attempt that meets a retried status closes its address's gate for the wait that request would take today: the reply's `Retry-After-Ms` or `Retry-After`, else its own doubling wait, capped by 60 seconds and its `--timeout`. The gate keeps the later of its current opening and the new one. A request whose attempts are spent closes the gate as well, because the next request would meet the same overload. A transport failure, and every status that is not retried, leaves the gate as it is.
4. **Who waits.** Before each live attempt, a first send or a retry, a request waits until its address's gate is open. It takes a send slot only after the gate opens. So a request waiting out a backoff holds no slot, and waits never block the throttle. A request that takes a slot while the gate closes again gives the slot back and waits. An answer from a recording, a replay or a cache never waits, because it sends nothing.
5. **How long a wait may last.** Before one attempt, a request waits on the gate for at most its own `--timeout` and at most 60 seconds, then sends even if the gate is still closed. A deadline or a cancellation ends the wait at once, as it ends a retry wait today.
6. **Retry counts stay per request.** Each request counts its own retries against its own `--max-retries`. Waiting on the gate spends none of them. Its own doubling continues from its own count, so a request that waited for another request's 503 and then meets its first 503 waits one second, or longer if the gate is still closed.
7. **The throttle is fixed.** `--jobs N`, and the throttle on the libraries and SQL, bound the sends in flight at N, counting first sends and retries alike. The number never shrinks after an overload and never grows back. This is today's behavior, and ADR 0052 writes it down.
8. **Every send spends one unit of a request budget.** A retry is a send, so it counts wherever sends are counted.
   - The command has no request budget.
   - The library's `max_requests` counts records in one call and is checked before anything is sent. It counts no sends, so a retry never touches it, and it is unchanged.
   - The SQL process request total reads `requests_sent`, which counts every send. A retry already spends one unit of it.
   - When the total runs out in the middle of a request, the retry is not sent. The request fails with the retried status that asked for the retry, and the call raises the surface's spent-total refusal after the rows it finished. Ticket 0149, which carries the SQL settings, moves the total's check to the engine's send gate so that this holds within a call. Until then the total is checked between calls, and one call's retries can pass it.
9. **Retries are counted apart.** `requests_sent` keeps its meaning: every send, first sends and retries alike. A new count, `retries`, reports the sends that were retries, so first sends are `requests_sent` less `retries`. It joins the usage totals: the engine's counters, the monthly usage file, and `thinkthen status` as `month_retries` and `total_retries`. It joins the run facts of ADR 0048 item 10, which ticket B5 builds. A usage file written before this ADR has no `retries` field and reads as 0. A row's `meta.requests_sent` does not change, and rows gain no field.
10. **Batches.** A batch is one request, by ADR 0048 item 5. A retried status on one batch closes the gate for every batch to that address. A batch's retry resends the whole batch, by ADR 0048 item 6, and counts against that batch's own retries. `--jobs N` still means N batches in flight, and at most N sends at once. The halves of ADR 0051 are two requests with their own retry counts, and a too-large refusal leaves the gate open.
11. **Libraries and SQL.** An engine is shared across calls, and the gate is shared wider: by every engine and call in the process that posts to the same URL. Two engines at two addresses never wait on each other. The library's `Counters` gains `retries()` in the library settings follow-up named by ticket 0154, beside `max_request_bytes`. Until then the engine counts retries and no binding reads them.
12. **What stays.** The retried statuses, the headers read, the 60-second cap, the doubling from one second, and the rule that a transport failure is never sent again.

## Why a shared gate and not a smaller throttle

Ian ruled on 2026-09-26 that the throttle stays simple. A gate stops every request to the overloaded address for the time the backend asked for, then lets the throttle's full width send again. Shrinking the throttle would take a second rule, for when to grow back, and a second number the caller cannot see. The gate needs neither, and it acts on the one signal the backend gives.

## Why `requests_sent` keeps counting retries

Ian ruled that retries count apart from `requests_sent`. Two readings fit the ruling. In the first, `requests_sent` keeps counting every send and `retries` names the subset. In the second, `requests_sent` drops to first sends only. The first keeps every row, recording test and usage file meaning what it means today. It also keeps a retry spending a unit of the SQL request total with no code change, which Ian's correction of the same day requires. The second would change the meaning of every existing count and let retries escape the total. This ADR takes the first. Ian can overturn it.

## What this amends

| Where | What changes |
| --- | --- |
| `specification/backends.md`, "The request" | `--max-retries` defaults to 3. The retry paragraph describes the gate, items 2 to 7 |
| `specification/settings.md` | The Retries row: default 3, with the reason, and no longer "Fixed at 2" but "Fixed at 3 on the libraries and SQL". The list of defaults with no reason loses "the 2 retries". The Throttle row says it counts retries and that a wait holds no slot |
| `specification/check.md` | `--max-retries` keeps its default of 3, and four probes send at most sixteen attempts |
| `specification/recording.md` | The usage paragraph names the `retries` count, and `status` reports it |
| ADR 0034 | The usage counts gain `retries`. Its marker lands with the build |
| ADR 0048 item 10 | The run facts gain `retries`, built by B5. Its marker lands with the build, after ticket 0154 edits the same file |
| ADR 0017 section 5 | The libraries' fixed 2 retries become 3. Its marker lands after ticket 0148, which amends that section, lands |

## What Ian can overturn

Ian's rulings of 2026-09-26:

1. Three retries by default, item 1.
2. Backoff at the provider, as one gate for each process and address, item 2.
3. A fixed throttle that counts retries, and waits that hold no slot, items 4 and 7.
4. Retry counts per request, item 6.
5. Every send, retries included, spending one unit of a request budget, item 8.
6. Retries counted apart from `requests_sent`, item 9.

The ticket author's calls:

7. `requests_sent` keeping every send, with `retries` as the subset, item 9.
8. The gate keyed by the exact posting URL, item 2.
9. A spent request closing the gate too, item 3.
10. A wait capped by the waiter's own `--timeout` and 60 seconds, then a send, item 5.
11. The mid-request budget check left to ticket 0149, item 8.
12. `retries()` on the library `Counters` left to the library settings follow-up, item 11.
13. Old usage files reading as 0 retries under the same `thinkthen.usage/1` schema, as ticket 0124 changed `thinkthen.status/1` in place before a release.
