# 0015: `annotate`

Ticket 0015 builds `thinkthen annotate QUESTIONS`. One saved question set asks several named decisions, choices, and scores about one document or every record. Questions with the same normalized evidence pointers share one request. The command prints one complete JSON object per record in input order. How-tos 39 and 14 are green, and page 08 is deleted into 39.

## What landed

The question-set parser accepts version 1, an optional inherited decision threshold, and a nonempty ordered question map. It reuses the single-question grammar, refuses `model` inside a question, reports complete key paths, normalizes absent `on` to the root pointer, detects pointer-key collisions, and computes the pinned digest of resolved behavior. Its generated property test parses decide, choose, and score sets, writes a source-compatible resolved form, parses it again, and preserves both behavior and digest.

The command accepts the established document, record, field, backend, retry, concurrency, recording, replay, cache, detail, and dry-run options. A likely second input path points to `--input`, and swapped question and evidence files name the mistake. An object record gains answer fields in question order. Other evidence returns an answer object. Detailed rows keep the input, bare values, complete per-question results and request digests, the question-set digest, checked aggregate usage, replay state, and one safe model version.

One global queue orders requests by record and then question group. `--jobs` bounds requests for both documents and streams. Read-ahead is bounded by the same work window. Once a request or model-version failure is observed, no queued request starts. Paid requests already in flight finish, and valid replies enter the recording. The earliest failure in queue order wins even when failures finish in reverse order. A closed output pipe stops reading and scheduling quietly; later in-flight failures stay quiet after paid work finishes and records.

The binary now has an internal library entrypoint and a thin executable. Pure parsing, grouping, digesting, and result assembly stay in the core. The scheduler owns execution and ordering. Existing single-question request bytes and digests remain fixed.

## Red then green

The first command test failed because `annotate` did not exist. Parser cases then failed on the missing question-set type. The first scheduler implementation passed serial cases but failed review because record workers could start later groups ahead of the accepted global order. Its replacement uses one request queue and passes jobs 1, 4, and 32 over documents and record streams.

The first remediation added exact request order and count, bounded read-ahead, cancellation, paid-completion recording, mixed replay and live aggregation, reverse-completion failure order, broken-pipe, model mismatch, secrecy, and usage-overflow tests. It also normalized absent and explicit root pointers before grouping and evidence selection.

The second review found three remaining proof gaps. A new listener regression closes standard output after the first row, then returns one valid delayed reply and a later HTTP failure. The command exits 0, writes nothing to standard error, sends three requests, and retains two valid recordings. `QuestionSetError` now keeps an exact duplicate path in its user-facing display and withholds it from `Debug`; the failure secrecy sweep drives key and evidence markers through that path. The generated grammar property now performs a real resolved-source round trip rather than comparing compact and pretty spellings.

## Pages and live evidence

How-to 39 screens one support message for a secret, a destructive request, and urgent wording. All three questions share one request. Its result is `false`, `false`, and `true`, and the policy returns `hold`.

How-to 14 grades six assistant answers with three decisions, one choice, and one score across two disclosure groups per record. The correctness decision resolved four rows and agreed with all four human labels. Two rows stayed inside the review band. The page treats them as work for a person.

Ian authorized one cached job against the reviewed `https://api.typesafe.ai/v1` base after local review. `sdlc/scripts/live --max-tokens 100000 probes/annotate-0015/run` reserved at most 100,000 input tokens. It made 29 requests: one for page 39, twelve for page 14, four for the mixed comparison, and twelve for the borderline comparison. The recordings report 10,104 input tokens and 1,533 output tokens. Every file passed schema, endpoint, request-response key, model, usage, digest-link, and credential-marker checks.

The mixed comparison packed one decision, one choice, and one score. Packed and separate values matched. Packing used 371 billed input tokens; three separate requests used 915. The borderline comparison used six labeled cases. Packing changed one result from `false` at probability 0.17 to unresolved at 0.21. The largest probability movement was 0.04. Packed requests resolved three cases and got all three right; separate requests resolved four and got all four right. The sample is deliberately small and near the cut, so the reference records the observed movement rather than generalizing an accuracy claim.

## Review and validation

A separate design reviewer accepted the ticket after three passes. A separate code reviewer rejected the first implementation and required the global request queue and broader proof. Its second pass accepted that redesign and rejected three remaining gaps: broken-pipe precedence, debug redaction, and a real canonical round trip. The same reviewer accepted those repairs before the paid job.

Focused validation passed six question-set tests, the failure secrecy test, and the exact late-failure-after-closed-pipe test. All four final rungs exit 0. The install rung fetched the locked closure and advisory data. Lint reports 92 resolved packages, 17 listed green pages, 3 coming pages, 2 leaving pages, and an exact 18,766-line Rust ratchet. The test rung passed 14 binary-library tests, 149 backend tests, 9 choose and score edge tests, 15 decide edge tests, 11 demo-runner tests, 17 question-file tests, 1 version test, 126 core tests, 2 documentation tests, and the Linux live-test cases. The specification rung passed 26 shell cases, 2 replay cases, every committed replay check, and 18 runnable green demos. `git diff --check` also passes.

## Decisions Ian can overturn

- The question-set canonical form holds an ordered list even though the source uses a named object. This makes order and names explicit in one digest.
- A row refuses mixed reported model versions. Safe diagnostics name both and tell the user to pin the model and rerun from a recording.
- Adding or changing one question invalidates its whole request group. The reference warns that this re-asks every question in the group and can move an answer near the cut.
- A closed output pipe wins over later in-flight failures. The command still waits for already-paid requests and records valid completions.
