---
flow: build
priority: 123
opens: crates/thinkthen/src/core/backend.rs crates/thinkthen/src/engine crates/thinkthen/src/cli crates/thinkthen/src/public/error.rs crates/thinkthen/tests/backend/relate crates/thinkthen/tests/backend/relate.rs crates/thinkthen/tests/backend/exchange.rs crates/thinkthen/tests/backend/refusals/relate.rs crates/thinkthen/tests/relate_edge.rs specification/backends.md specification/relate.md sdlc/planning/adr/0040-split-requests-under-backend-limits.md sdlc/tickets/0059-add-named-backend-profiles.md profiles/README.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues
---

# 0123: Relate splits requests to fit the backend

Status: ready. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

A user runs `relate` against the hosted backend with no `--profile`. Every request fits under the backend's input token cap, so the backend answers each one. When the backend still refuses a request as too long, the user reads the backend's reason and knows what to change. Planning a large entity set takes well under a second.

The ask is items 1 and 2 of `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`. The backlog report of 2026-09-25 (`sdlc/planning/issue-backlog-2026-09-25.md`) puts them third in section A, before 0.1. The coordinator added three relate wording items on 2026-09-25, because they sit in the same files. They are items 3 and 4 and the relate half of item 8 in `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Ticket 0126 takes the rest of that issue.

## Design

Three changes share the one request splitter in `crates/thinkthen/src/engine/prepared_request.rs`. Three small wording fixes ride with them.

**A built-in split ceiling for the hosted backend.** `Backend` in `core/backend.rs` gains one method. It returns a ceiling of 96,000 request bytes when the resolved address is the built-in address `https://api.typesafe.ai/v1/systemone`. It returns nothing for any other address. `PreparedRequests::with_profile` reads it. A chunk of two or more questions fits only when its exact encoded body is at most the ceiling. A chunk of one question always passes the ceiling. So the ceiling splits a plan and never refuses a request. A profile that sets `max_request_bytes` replaces the ceiling. A profile without it keeps the ceiling beside its other limits. The ceiling is not a profile. It carries no name, adds no `backend_profile` value to a dry run, and never causes a choice to fall back to yes/no questions.

**A splitter that grows the chunk by doubling.** The loop at `prepared_request.rs:47` encodes every prefix `1..=remaining`, so planning grows with the square of the question count. The new loop tries 1, 2, 4, and so on questions from the chunk start, up to the remainder. It stops at the first count that does not fit. It then binary-searches between the last fitting count and that count. Every limit only tightens as a chunk grows, so the search finds the same longest fitting prefix the old loop found. Any failure counts as "does not fit" during the search. When one question alone does not fit, the splitter returns that one question's error, as the old loop did. The search encodes and checks each candidate. It computes the digest only for the chunk it keeps.

The issue suggested trying the whole remainder first. Doubling from the chunk start does the same work for a plan that fits in one request. It costs about twice one full encoding. It avoids encoding the whole remainder again at every chunk, which would cost the chunk count times the plan size.

**The backend's reason for status 400.** On status 400, `engine/http.rs` reads at most 4 KiB of the reply body. It parses the body as JSON and reads only `detail.error_type`. When that value is exactly `max_tokens_exceeded`, the engine returns a new error `Error::TokenLimit` in place of `Error::Status(400)`. Every other body keeps `Error::Status(400)` and its fixed sentence. The command prints no other byte of the body. This keeps the ticket 0082 rule that a body can quote the evidence back and must not be printed. The closed list holds one value, because it is the only value anyone has seen.

**Relate wording.** `relate --either @FILE` gets its own refusal sentence. `relate --help` hides `--jobs` through clap's `mut_arg` on the `Relate` variant, and the refusal of `--jobs` stays. `relate --help` gains one cost sentence.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. **The ceiling is 96,000 request bytes.** The measured density is about 0.53 input tokens a byte for relate's JSON (the Beatles runs of 2026-09-24). 96,000 bytes then comes to about 50,900 tokens, plus a fixed part near 256 tokens. That is about 22% under the 65,536-token cap. A request would reach the cap only at more than 0.68 tokens a byte. The densest text measured so far is relate's JSON. Experiment 219's filler, dense with digits, measured 0.30 tokens a character. `backends.md` and ADR 0040 record this source. When a denser text does reach the cap, decision 5 names the cause.
2. **The ceiling is sized against refusal, not for accuracy.** Experiment 260 found packed accuracy falls from 0.92 near 2,000 tokens a request to 0.79 near 27,000. That experiment packed many records, each with its own question. relate asks many questions of one shared entity set. A lower ceiling would send the whole entity set again with every extra chunk, and no experiment measured relate at smaller chunks. The size at which a pack should close for accuracy belongs to the packing setting (`pack_tokens` in `sdlc/issues/2026-09-25-packing-and-batching-what-they-buy-what-they-cost-and-the-setting.md`) and its ADR.
3. **Only the built-in address gets the ceiling.** The measured cap belongs to the hosted service. The ceiling applies whenever the resolved address equals the built-in one, whether the user named it with `--url`, with `THINKTHEN_BASE_URL`, or not at all. Any other address keeps today's behavior. A user of another backend states its limits in a profile.
4. **The ceiling splits and never refuses.** A single question over 96,000 bytes goes out alone, as it does today. The ceiling comes from a ratio, not from the backend. A local refusal of a request the backend might accept would break a run that works today. The backend's own refusal and decision 5 cover that case.
5. **Print `max_tokens_exceeded` only when the body names it.** The sentence is fixed: `the backend answered with status 400 (max_tokens_exceeded): the request has more input tokens than the backend takes; shorten the text or set a lower max_request_bytes with --profile`. It exits 4, as every status failure does. A 400 with any other body, an unreadable body, or a body over 4 KiB prints the existing 400 sentence. Only status 400 reads the body.
6. **The public library message stays the same.** `public/error.rs` maps `Error::TokenLimit` to the text it prints today for status 400, `the backend answered with status 400`. The public API and every surface keep their pinned messages. Naming the reason in the library is a deferred gap.
7. **A profile's `max_request_bytes` replaces the ceiling, and its other limits add to it.** A user who states a byte limit knows their backend. A user whose profile states only `max_questions`, such as the Beatles deck's `jev.json`, keeps the protection.
8. **ADR 0040 gains an amendment.** ADR 0040 says the repository ships no Jev byte limit until a measurement supports one. The issue's eight live calls and the measured density are that measurement. The amendment names them. The ticket 0059 example gains a correction line under its code block. It says its byte numbers had no source and points to this ticket. `profiles/README.md` says the hosted backend has a built-in ceiling and ships no profile file for it.
9. **The `--either @FILE` sentence** is `` `--either` applies only to inline relation rules; a question file sets either on each relation ``. It exits 2 with zero sends, as today.
10. **The relate cost sentence** is `A run makes paid requests: each concrete relation asks one question for every entity on its larger side, or one for every pair it asks yes or no. --dry-run prints the questions and requests and sends nothing.` It goes in the `Relate` doc comment in `cli/args/command.rs`. The sentence matches `specification/relate.md`, "Relations".

## Edge cases

The ceiling and a profile:

| Address | Profile | Plan | Requests |
| --- | --- | --- | --- |
| Built-in, named or not | none | Encoded whole at most 96,000 bytes | One, with today's exact body and digest |
| Built-in | none | Over 96,000 bytes, every question fits alone | The fewest contiguous chunks, each the longest prefix at most 96,000 bytes |
| Built-in | none | One question alone over 96,000 bytes | That question goes alone. The questions before and after it chunk as usual. Exit 0 |
| Built-in | `max_questions` only | Any | Both limits apply to every chunk |
| Built-in | `max_request_bytes` N | Any | N alone limits bytes, above or below 96,000. Over N alone refuses at exit 2, as today |
| Loopback or any other address | none | Any | One request, as today |
| Loopback or any other address | any | Any | As today |

The 400 body:

| Status | Body | Stderr line after `thinkthen: ` |
| --- | --- | --- |
| 400 | `{"detail":{"error_type":"max_tokens_exceeded"}}` | The decision 5 sentence |
| 400 | `{"detail":{"error_type":"MARKER"}}` with an evidence marker | The existing 400 sentence, with no marker |
| 400 | Not JSON, empty, `{}`, or `detail` not an object | The existing 400 sentence |
| 400 | Over 4 KiB, naming `max_tokens_exceeded` past the bound | The existing 400 sentence |
| 422 | `{"detail":{"error_type":"max_tokens_exceeded"}}` | The existing 422 sentence |

## Acceptance

Every relate test lives in a new `crates/thinkthen/tests/backend/relate/ceiling.rs`. It runs the built command through the existing backend harness spawn, which clears the environment and sets a temporary `HOME`. A dry run with no `--url` resolves to the built-in address, reads no key, and sends nothing. The Beatles set is the committed fixture `tests/fixtures/recognize-239/input.json`: 184 songs, 13 albums, and 4 people, with rules `sung_by=song:person` and `appears_on=song:album`.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| The Beatles set fits the hosted backend | A dry run with no `--url` and no profile. `sung_by` plans one request. `appears_on` plans the exact request count and byte list the build records, each at most 96,000. The same run with a loopback `--url` plans one `appears_on` request over 96,000 bytes, and its digest equals today's | Return no ceiling. `appears_on` plans one request over 96,000 bytes |
| Other addresses keep their plan | Inside the test above: the loopback run's plan equals main's byte for byte | Apply the ceiling at every address. The loopback run splits |
| A profile byte limit replaces the ceiling | The built-in address with a profile of `max_request_bytes` 200,000 plans one `appears_on` request. With a profile of `max_questions` 64, every chunk holds at most 64 questions and at most 96,000 bytes | Take the smaller of the profile and the ceiling. The 200,000 run splits |
| One oversize question goes alone | The built-in address with a set whose one choice question encodes over 96,000 bytes, from long album names. The dry run exits 0 and plans that question alone | Refuse a single question over the ceiling. The run exits 2 |
| The splitter keeps the old boundaries | A loopback `relate linked --lines --dry-run` over 40 entities with a profile of `max_request_bytes` 20,000. The byte list of every request equals the list main prints before this change. The build captures that list first and pins it as a literal | Keep one question fewer than the longest fitting prefix. The list changes |
| A full set plans fast | `relate linked --lines --dry-run` over 255 entities at the built-in address. It exits 0 inside the harness's 60-second child deadline, and every request is at most 96,000 bytes. The old loop needs hours for this set in a debug build | Restore the prefix loop. The child deadline kills the run |

In `tests/backend/exchange.rs`, `common_request_statuses_name_fixed_actions_and_hide_the_body` gains the rows of the 400 body table. Each pins the whole standard error line and checks that no marker appears.

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| The reason prints | The 400 `max_tokens_exceeded` row prints the decision 5 sentence | Drop the body read. The old sentence prints |
| Only the known reason prints | The 400 marker row prints the old sentence with no marker | Print any `error_type`. The marker appears |
| The bound holds | The row over 4 KiB prints the old sentence | Read the whole body. The new sentence prints |

For the wording items:

| Test | Proof | Planted fault that turns it red |
| --- | --- | --- |
| `--either @FILE` names its cause | A new row in the relate refusal table (`tests/backend/refusals/relate.rs`) pins the decision 9 sentence, exit 2, and zero sends | Return the grammar sentence. The row fails |
| relate help hides `--jobs` and names its cost | `help_names_the_beta_complete_set_and_secrecy_contract` in `tests/relate_edge.rs` asserts no `--jobs` in `relate --help` and the whole decision 10 sentence. The existing `jobs on one document` refusal row still pins exit 2 | Drop the `mut_arg`. `--jobs` appears |

Every new test answers the four questions of `CLAUDE.md`:

- **What behavior does it protect?** Each row names it.
- **What credible regression fails it?** Each row's planted fault. The build runs every plant and records it red.
- **Why does no existing test catch it?** No test runs a plan at the built-in address. No test sends a 400 body with a reason. No test plans a set large enough to show the prefix loop's cost. No test pins relate help's `--jobs` line or cost sentence.
- **Does it need a test-only hook?** No. The built-in address is reachable by a dry run, which sends nothing. The 400 bodies come from the shared loopback `Canned::status`. The 60-second child deadline already exists. The existing test-only `PREPARATIONS` counter is not used.

Also:

- The builder runs the real Beatles deck dry run once by hand from the release build, with no key and no network. The build record gives its per-request bytes, and the timing of a release dry run over 255 line entities. The issue asks for under one second.
- The `spec` and `surfaces` rungs show that no recorded exchange changed digest. A recorded plan at the built-in address over 96,000 bytes would change digest and miss. None exists in this repository today.

## Specification pages

- `specification/backends.md`, "Explicit profiles and local preflight": one paragraph on the built-in ceiling, its number and source, that it splits and never refuses, and how a profile's `max_request_bytes` replaces it. The status table's 400 row gains the `max_tokens_exceeded` sentence.
- `specification/relate.md`, "Dry run": one sentence that a plan at the built-in address splits under the built-in ceiling.
- ADR 0040 amendment, `profiles/README.md`, and the ticket 0059 correction line, as decision 8 says.

## Budgets and the ratchet

- `engine/prepared_request.rs`: at most 30 nonblank lines added, net.
- `core/backend.rs`: at most 12 nonblank lines added.
- The 400 reason across `engine/http.rs`, `engine/error.rs`, `cli/failure.rs`, `cli/failure/status.rs`, `cli/failure/convert.rs`, `cli/check.rs`, and `public/error.rs`: at most 45 nonblank lines added.
- The wording in `cli/relate/config.rs` and `cli/args/command.rs`: at most 12 nonblank lines added.
- `tests/backend/relate/ceiling.rs`: at most 200 nonblank lines. `tests/backend/exchange.rs`: at most 30 added. `tests/backend/refusals/relate.rs` and `tests/relate_edge.rs`: at most 20 added together.
- Specification, ADR, README, and ticket 0059 lines: at most 30 added together.
- No dependency.
- The ratchet rises to the measured total in the commit that adds the code, at most 350. That commit says what grew and why. It names where the builder looked for duplication first: the status phrases in `cli/failure/status.rs`, the limit checks in `core/backend_profile.rs`, and the relate test helpers in `tests/backend/relate.rs`.

## Stop rules

Stop, re-score, and tell the coordinator before any of these:

- Crossing a budget, adding a dependency, or changing the public API or the dry-run schema.
- A changed body or digest for a plan that fits the ceiling, or for any plan at another address.
- The real Beatles dry run planning an `appears_on` request over 96,000 bytes, or a release dry run over 255 line entities taking one second or more.
- Touching the cache or recorder code (ticket 0124), audit or diff (0125), command help outside relate (0126), the loopback backend or secrecy tests (0127), `databases/` (0129), or `libraries/python` (0122).
- Any live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Scope and exclusions

Excluded: learning a bytes-per-token ratio from replies, splitting again after a refusal, a token estimate in any output, and a local check of the backend's per-text limit near 32,000 tokens. Also excluded are items 3 to 8 of the scale-and-shape issue, and every item of the wording issue except 3, 4, and the relate half of 8.

## Closing

Landing trims items 1 and 2 from `sdlc/issues/2026-09-25-recognize-and-relate-scale-and-shape.md`. It leaves items 3 to 8 open. It trims items 3 and 4 and the relate half of item 8 from `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Each trimmed item leaves one line naming this ticket and the landing commit. Neither issue closes, so neither file moves to `closed/`.

## Dependencies and order

Build from `origin/main` at `08a8e754` or later. Ticket 0126 edits other commands' help in `cli/args/command.rs`. Whichever lands second merges the other's lines. No other ticket touches the splitter or `engine/http.rs`.

## Routing

Builder: Claude (Opus subagent). Reviewer: a fresh Claude session for design and for code. The ceiling raise needs a second-agent review that names what it checked.

## Complexity

Contract 2; state and timing 1; reach 2; proof 2; cost of error 2; total 9. Final level: 2. The risk is a splitter that picks different chunks than the old loop. That would change digests and re-bill cached answers. The pinned boundary test and the unchanged recordings guard it.

## Evidence

- Starts from: The issue's item 1, from the marketing session's eight live calls of 2026-09-24 (commit `10d425e5`): 142 `appears_on` questions passed at 65,423 input tokens and 143 failed, and the body read `{"detail":{"error_type":"max_tokens_exceeded"}}`. About 0.53 input tokens a byte for relate's JSON. Experiment 219 (`~/workspace/experiments/219-thinkthen-limit-edges/RESULTS.md`): 61,819 tokens answered and about 66,000 refused for a whole request, and 0.30 tokens a character for digit-dense filler. Experiment 260's accuracy by request size. The 0088 final review, finding F1 (`sdlc/records/0088-review-final.md`): 0.47 s at 40 entities and 7.65 s at 80 in a release dry run. ADR 0040 and ticket 0079's splitter. Ticket 0082's rule never to print a reply body. The committed Beatles fixture `tests/fixtures/recognize-239/input.json`.
- Keeps: Every plan that fits the ceiling, and every plan at another address, keeps its exact bodies and digests. The longest-prefix rule of ADR 0040. Every profile limit and refusal sentence. The relation fallback rule. The 400 sentence for every body but one. Exit codes. The public library's messages. The `--jobs` refusal on relate.
- Changes: A built-in 96,000-byte split ceiling at the built-in address. A splitter that grows chunks by doubling and binary search. A named `max_tokens_exceeded` sentence on status 400. A new `--either @FILE` sentence. relate help hides `--jobs` and gains a cost sentence.
- Proof: The tests in "Acceptance", each with a planted fault that turns it red. The hand-run Beatles dry run and the release timing in the build record. The `install`, `lint`, `test`, `spec`, and `surfaces` rungs. Every test uses a dry run or a loopback backend with a fake key.
- Defers: Naming the reason in the public library and the surfaces. Splitting again after a `max_tokens_exceeded` refusal. Learning each backend's bytes-per-token ratio from its replies. An accuracy-sized request, which the packing ADR owns. A local check of the per-text limit near 32,000 tokens. Ceilings for other backends, which profiles state. Items 3 to 8 of the scale-and-shape issue.

## What Ian can overturn

The 96,000-byte number. Sizing it against refusal rather than accuracy. Applying it only at the built-in address. Sending one oversize question rather than refusing it. A profile's `max_request_bytes` replacing the ceiling. The closed list of one reason and its sentence. Keeping the library message unchanged. The two relate help sentences.
