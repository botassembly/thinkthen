# ADR 0051: Every address has a ceiling, and a refused batch halves once

- Status: Proposed through ticket 0154. The coordinator ruled the ceiling and the halving on 2026-09-26, and Ian ruled the request-size setting that day. It becomes Accepted when the fresh ticket review accepts ticket 0154. Ian can overturn each item
- Date: 2026-09-26

This ADR amends ADR 0048 items 2, 5, 6, 9 and 11, and the ADR 0040 amendment of ticket 0123. ADR 0048 says that changing one of its items takes a new ADR. This is that ADR. Ticket 0154 builds every item but the `split` member of item 10, which B5 builds with `meta.batch`.

## Context

The 96,000-byte request ceiling applies only when the posting URL is the built-in address (`specification/backends.md`, "Explicit profiles and local preflight"). Any other address with no profile has no size limit. There a default batch fills until a content cut, which falls on average once in 4,096 records. At about 181 bytes a short title (evidence section 9 of `sdlc/records/2026-09-26-batching-and-recognize-evidence.md`), a batch of a few thousand records runs to hundreds of kilobytes. A local server whose model has a smaller context window refuses it, and ADR 0048 item 2 then fails the run at exit 4. The first default run against a new address can fail that way before the caller learns that a profile exists.

Only a profile file's `max_request_bytes` can change the ceiling today. A profile is a closed JSON file that also carries a calibration name, so a caller who only wants larger requests must write a file.

ADR 0048 item 6 says that no batch is split and resent. The batching design gives its reason in section 4 of `sdlc/issues/2026-09-26-batching-design.md`. A retried status means the backend answered and may have billed the first attempt, so a batch may be paid twice. The design also keeps a recorded run's batches a function of the records and settings alone, so replay finds every batch. Section 2 of the design, "Other addresses", leaves a refused batch to the caller, who sets `max_request_bytes` in a profile or runs at `--batch 1`.

The hosted backend refuses a request over about 65,536 input tokens with status 400 and the body `{"detail":{"error_type":"max_tokens_exceeded"}}` (evidence section 7). Relate's JSON measured 0.516 input tokens a byte, the worst rate measured, so 96,000 bytes comes to about 49,500 tokens (ticket 0123). That keeps about a quarter of the limit back. Record text measured 0.40 tokens a byte in experiment 268. At those rates the hosted limit lies near 127,000 bytes for relate's JSON and near 164,000 bytes for record text.

## Decision

1. **The request size is a setting, and its default applies at every address.** The setting is the most bytes one request may hold before the tool splits a plan or closes a batch. It is `--max-request-bytes N`, then `THINKTHEN_MAX_REQUEST_BYTES`, then the default of 96,000. `N` is a whole number of at least 1. Zero, a sign, a fraction and any other text are a usage error at exit 2. It has no question-file key and no configuration-file key, because it describes the backend, not a question. A relation plan splits under it, and a batched record plan closes at it, at every address. One question alone, and one record alone, still pass it and go as one request. It splits and never refuses, with one exception, and it never turns a choice into yes/no questions. The exception is a shared context. By ADR 0048 item 11, `Batcher::context_fits` refuses at exit 2 a context whose request with one record passes the limit, and the request size is that limit when no profile sets a smaller one. Today that refusal can happen only at the built-in address. After this ADR it can happen at every address, at 96,000 bytes by default, and the caller raises the setting to pass a larger context. Ticket B7, which builds `--context`, meets that wider reach.
2. **How the setting and a profile combine.** A profile's `max_request_bytes` keeps its meaning: it is a limit the backend enforces, a plan splits under it, and a request that cannot split under it is refused at exit 2. It no longer replaces the setting. A request holds at most the smaller of the two. So a profile can lower the size and cannot raise it. A caller raises the size with the setting. A profile's other limits apply beside both, as today.
3. **Sizing the setting for a larger model.** Divide the model's input-token limit by the worst measured rate of 0.516 tokens a byte, then keep a quarter back, as the default does. For a context of 500,000 tokens: 500,000 / 0.516 is 968,992 bytes, and three quarters of that is about 726,000, so `--max-request-bytes 726000`. For 1,000,000 tokens the same sum gives about 1,453,000. A reply's bytes and the model's own output still need room when the context counts both. A request that large may also need a longer `--timeout`.
4. **Above 96,000 at the built-in address, the command warns.** A run whose setting is above 96,000 at the built-in address prints one line on standard error, once, before any request and under `--dry-run` too: `thinkthen: warning: max_request_bytes 200000 is above the default of 96000; the built-in backend refuses a request over 65536 input tokens`. The run goes on. A refusal would block a sound use, because 96,000 bytes keeps a quarter of the measured limit back, and a caller who has measured their own text may use more of it. A warning keeps the choice visible. A setting at or under 96,000, and any setting at another address, warns nobody.
5. **What this changes for `relate` and the relation step of `recognize`.** Both already split under the ceiling at the built-in address. At any other address, a relation plan over 96,000 bytes now splits into several requests, where it went as one before. The split requests have new bytes and digests. A recording or cache made at another address with a relation request over 96,000 bytes misses once for that relation, and the rerun pays for its split requests. No recording in this repository holds a request over 90,000 bytes. A caller who wants the old single request raises the setting. A profile whose `max_request_bytes` is above 96,000 no longer raises the size, at any address, and the caller moves that number to the setting. Plans at the built-in address with no profile do not change. The library and SQL surfaces share the engine's default of 96,000, and ticket 0157 gives them the setting.
6. **Which refusals count as too large.** Two refusals count, at every address:
   - Status 400 whose body names `max_tokens_exceeded` as its `detail.error_type`. The command already reads that body at every address. A server that presents the System One shape and sends that value means what the hosted backend means.
   - Status 413. HTTP defines it as a body larger than the server will take (RFC 9110, section 15.5.14).

   Plain 400, 422 and every other status do not count. Each can also mean a malformed request, and a malformed batch would fail both halves too. Neither counted status is retried, as today.
7. **One halving.** When a request for a batch of two or more records is refused as too large, the command splits the batch into two halves. The first half holds the first ⌈n/2⌉ records of the batch's n, and the second holds the rest. Here n counts the batch's members, one for each input record it answers, repeats included, and not its distinct texts. Each half forms its request by ADR 0048 item 1, so a half of one record without a context sends today's request of that record, byte for byte. The command asks the first half, then the second, inside the batch's place under `--jobs`, so at most N requests stay in flight. A half gets the normal retry rules. A half that is refused as too large is not split again. It fails at exit 4, and the run stops at the half's first record. When the first half fails, the second is not sent. When the second half fails, the first half's rows print first. The stop line takes the form ticket 0146 gives a failed batch and names the half's range: `thinkthen: stopped at record 46; the request for records 46 to 50 failed: the backend answered with status 413: the backend refused the request as too large; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile; 45 records finished`. A failed half of one record prints today's two lines. `--dry-run` plans the whole batch, because a plan cannot know a refusal.
8. **The too-large phrases name every fix.** The status table in `specification/backends.md` gains the row `413`: `the backend refused the request as too large; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile`. The 400 `max_tokens_exceeded` phrase ends `shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile` in place of `shorten the text or set a lower max_request_bytes with --profile`. Each phrase names every fix, because one phrase serves every verb and every batch size. Shortening the text fixes a lone record or question, which no request size can split. The flag fixes a batch or a relation plan on `decide`, `filter`, `rank`, `relate` and `recognize`. The profile fixes the verbs that have no flag yet, such as `choose`. The phrase also prints unchanged at `--batch 1`, where a 413 printed a bare status before. The library and SQL surfaces keep their bare status line.
9. **Replay, cache and recording stay deterministic.** The halves are a pure function of the batch: the same records under the same settings form the same halves and the same digests. A refused request writes no entry, as a failed request writes none today. Under `--record` the halves are recorded like any batch. Under `--replay`, a batch of two or more records whose request has no entry asks its two halves from disk, in order, before it counts as missing. When the first half has no entry either, the whole batch is the missing batch, and the stop line names the whole batch's range, as ticket 0146 prints it. When the first half answers and the second has no entry, the first half's rows print and the second half is the missing batch. So a replay of a split run gives the recorded run's rows, standard error and exit code, byte for byte. Under `--cache` and the default cache, a batch whose request has no entry goes live as today. When the backend refuses it, each half is asked through the cache like any request. The request size changes request bytes, so it changes batches and digests, as `--batch` does.
10. **The split is marked, and its costs are counted.** The refused request's attempts count in the first half's `requests_sent`, and each half's rows carry its own `usage`, `requests_sent` and digest in `meta.requests`. So per-record shares still sum to the run, and `--facts` still counts every HTTP attempt. Ticket 0154 builds before B5, by the coordinator's ruling of 2026-09-26, so until B5 builds `meta.batch` those per-request counts and digests are the only record of a split. B5 then gives every row a half answers `meta.batch` with `"split":true` under `--details`. This holds even when the half holds one record and no context, where ADR 0048 item 9 leaves `meta.batch` out. The half's `records` and `position` describe the half, and `closed` keeps the whole batch's reason. A row of a batch that was not split carries no `split` member.
11. **What stays.** A retried status resends the whole batch as one request, as ADR 0048 item 6 says. Relation chunks are not halved on a refusal, because they already split under the request size. A record whose request passes the request size alone still goes alone and fails at exit 4 when the backend refuses it.

## Why the design's argument does not hold for this one case

The design's clause answers retried statuses. A retried status is transient, so the same bytes can succeed the next time, and resending the whole batch is the right move. A too-large refusal is not transient. The same bytes fail every time, so resending the whole batch never helps, and only a smaller request can succeed. The clause therefore reverses for this one case and stands for every other status.

The billing concern does not grow. A refused request returns no answers and reports no usage, so no record's answer is paid twice. Whether the hosted backend bills a refused request at all is unmeasured. The extra cost is at most the one refused request for each split batch.

Determinism holds. The halves depend only on the batch, and the batch depends only on the records and settings. Item 9 gives replay the same order of lookups as the live run.

The cost stays bounded. A batch sends at most three requests before it answers or fails. A half still refused says that the backend's limit lies far below the request, and a lower request size is the right fix. Item 1 makes that case rare, because a default request now holds at most 96,000 bytes at every address. A single halving then covers a backend that takes a little over half of the request size or more.

Leaving every refusal to the caller costs a failed run at the first large default batch against a new address. The halving turns most of those failures into a slower run with a marked split.

## What this amends

| Where | What changes |
| --- | --- |
| ADR 0048 item 2 | "Other addresses have no byte ceiling" no longer holds. The request size, 96,000 by default, closes batches at every address, and a profile's `max_request_bytes` lowers it |
| ADR 0048 item 5 | A replay counts a batch missing only when its halves are missing as item 9 says. The request size joins the settings that change batches |
| ADR 0048 item 6 | "No batch is split and resent" holds for every refusal but too large. A refused batch halves once |
| ADR 0048 item 9 | `meta.batch` gains `split`, and a half of one record carries `meta.batch`, built by B5 |
| ADR 0048 item 11 | A context whose request with one record passes the request size is refused at every address, not only at the built-in one. B7 builds `--context` with that reach |
| ADR 0040, amendment of ticket 0123 | "A profile's `max_request_bytes` replaces it" becomes "the smaller of the request size and a profile's `max_request_bytes` applies". "Other plans and other addresses keep no ceiling" holds for other plans only. Its marker lands after ticket 0147, which edits that line, lands |
| `specification/backends.md` | "Explicit profiles and local preflight" states items 1 to 7 and 9. The status table gains 413, and the 400 `max_tokens_exceeded` phrase names the setting |
| `specification/records.md` | The replay and failure rules for batches state items 7 and 9 |
| `specification/result.md` | The `requests_sent` row counts a refused request in the first half. B5 adds `split` to the `meta.batch` row |
| `specification/settings.md` | A new row, "Request size", gives the flag, the variable, the default of 96,000 and its reason. The backend profile row says that a profile lowers the request size and never raises it |
| `specification/decide.md`, `filter.md`, `rank.md`, `relate.md`, `recognize.md` | Each names `--max-request-bytes` in one sentence |

The build of ticket 0154 edits each page and removes the old sentence in the same commit, so no page carries a marker for this ADR. B5 adds `split` to the `meta.batch` row when it builds that row.

## Overlap with the recognize ADR

Ticket 0147's ADR 0050 extends the built-in ceiling to recognize word and `confirm` questions, and names one piece at an address with no ceiling and no profile. Once this ADR is built, every address has a request size, so that case no longer exists. Ticket 0147 edits ADR 0040 and `backends.md` line 17. This ADR edits neither until ticket 0147 lands.

## What Ian can overturn

Ian's ruling of 2026-09-26:

1. The request size as a setting with a flag, a variable and a documented default, item 1, including its wider reach over a shared context.

The coordinator's rulings of 2026-09-26:

2. The 96,000-byte default at every address, item 1.
3. One halving on a too-large refusal, item 7, which reverses ADR 0048 item 6's clause for that case.

The ticket author's calls:

4. The smaller of the setting and a profile's `max_request_bytes`, so a profile no longer raises the size, item 2.
5. No question-file or configuration-file tier, item 1.
6. A warning, not a refusal, above 96,000 at the built-in address, item 4.
7. Status 413 and the named 400 body as the two too-large refusals, at every address, and 422 left out, item 6.
8. The first half taking the extra record of an odd batch, and the halves sent one after the other, item 7.
9. A replay asking the halves when the whole has no entry, item 9.
10. The refused request's attempts counted in the first half, and `split` on every row a half answers, item 10.
11. The phrases in item 8.
