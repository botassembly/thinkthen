# Architect review severity 3 findings, for triage after 0.1

Status: Open. Filed 2026-09-26 by the queue owner from local experiment 273, reports 01 and 03 to 12. For triage after 0.1. Nothing here blocks 0.1.

The review rated these findings severity 3: a sharp edge or a missing feature. None was checked against main for this issue, so each is not verified. "Tracked" names the ticket or issue that already carries it. Triage splits the rest into Quick Fixes, doc lines, and later issues.

## Report 01, `filter` and streams

- I3. One request per record. Tracked: 0146, batching B4.
- I4. Ordered output blocks at the head of the line. One slow record stalls the run. Document the stall bound.
- I5. Retries are per worker and move in lockstep. Tracked: 0155, ADR 0052.
- I6. One bad record ends the stream with no machine-readable stop point. Tracked in part: `2026-09-26-run-facts-b5-owe-cause-retryable-and-stop-record.md`.
- I7. `filter` records have no default size guard. Tracked: 0154, ADR 0051.
- I8. `filter` cannot serve as a coprocess, and the README does not say so. Doc line.
- I9. After Ctrl-C, the stop line names a record that never arrived. Wording fix.
- I10. The second-Ctrl-C escape is undocumented. Doc line in `channels.md`.

## Report 03, `annotate`

- 3-1. A name collision stops the run even under `--details`.
- 3-2. A missing pointer stops the whole run.
- 3-3. Row shape and field types vary within one stream. Doc recipe.
- 3-4. Failure isolation depends on how questions are grouped.
- 3-5. `annotate` has no built-in request ceiling. Tracked: 0154, ADR 0051.
- 3-6. The libraries refuse any set that uses `on`. Tracked: `2026-09-25-public-library-api-gaps.md` item 5.
- 3-7. Document mode sniffs JSON. Doc line.

## Report 04, libraries and databases

- I5. In SQL, one failing row fails the statement, with no per-row error value.
- I6. A deadline bounds one call, not a query. Tracked in part: the DuckDB query-hook issue.
- I7. SQLite and PostgreSQL scalars hold one request in flight. Tracked: the equivalence page, E2 and E7.
- I8. DuckDB's warm pass ignores session settings.
- I9. DuckDB refuses every call after 16 distinct engines.
- I10. The throttle belongs to the process for its whole life. Tracked in part: ADR 0047 item 5.
- I11. Libraries cannot set timeout, retries, profile or replay. Tracked: 0148, 0149.
- I12. A SQLite cancel leaves a detached worker holding its permit.
- I13. DuckDB relate's message for an uncommitted table misleads.
- I14. List calls refuse `choose`, `score` and `tag` over many texts. Tracked: the equivalence page, E2.
- I15. The missing-key error has a different kind on each surface. Consider folding into 0148.
- I16. Packaging and loading sharp edges. Tracked: 0128 phases 3 and 4.

## Report 05, the answer contract

- 3.1. Tie policy differs across functions, and score's level tie rule is undocumented.
- 3.2. Only `decide` has a not-sure region, and `find` answers even with no `--none`. Tracked in part: L4.
- 3.3. `--threshold` cuts a different quantity in each function.
- 3.4. The details line has parse traps.
- 3.5. A single question file cannot carry a version.
- 3.6. A record run stops at the first failed record. Tracked: the roadmap's `--on-error` hold, H3.
- 3.7. Contract pages disagree in small ways. Tracked: 0152 Part B.

## Report 06, backends and configuration

- I-6. No request-size setting on main. Tracked: 0154, then 0157.
- I-7. A broken configuration file stops every command, and the message names no field.
- I-8. The model has no environment tier.
- I-9. A keyless local server still needs a dummy key. Doc line.
- I-10. The default cache serves one address. Tracked: ticket 0124's deferred gaps.
- I-11. An extreme `--timeout` panics with exit 101. Quick Fix, with report 11 issue 7.
- I-12. The release binary honors `THINKTHEN_TEST_RETRY_WAIT_MS`. Quick Fix, with report 07 I13.
- I-13. Model names are not trimmed or checked for control characters. Quick Fix.
- I-14. Refusal phrases give advice that misfits the case.

## Report 07, throughput, limits and cost

- I3. A long `Retry-After` is cut short. Tracked: 0155.
- I4. `Retry-After: 0` resends at once with no floor. Consider folding into 0155.
- I5. Retries have no jitter. Tracked: 0155.
- I6. Every retry opens a new connection.
- I7. One slow record stalls the run. Doc line, with report 01 I4.
- I8. `--dry-run` cannot estimate cost.
- I9. No run total. Tracked: B5.
- I10. Retries cannot be told apart from first sends. Tracked: 0155's `retries` count.
- I11. Libraries cannot set timeout or retries. Tracked: 0148.
- I12. Nothing caps concurrency across processes.
- I13. An undocumented test variable removes the retry wait. Same as report 06 I-12.
- I14. The final 429 message hides the retries. Quick Fix.

## Report 08, the cache

- 6. The key is exact bytes, so input framing changes it. Doc line.
- 7. Prune evicts the oldest-written entries, not the least recently used.
- 8. A typo in `--answered-by-other-than` deletes everything, and prune has no dry run. Tracked: ticket 0124's deferred gaps.
- 9. A cached result repeats the stored usage. Documented. Accept as cost.
- 10. The default cache binds to one address. Tracked: ticket 0124's deferred gaps.
- 11. `Engine::builder()` ignores the configuration file's `cache: false`. Check within 0148.
- 12. Crashed writes leave hidden temporary files. With report 11 issue 4.
- 13. DuckDB's warm pass ignores `SET thinkthen_cache`. With report 04 I8.

## Report 09, record, replay and testing

- 4. Reformatting a recording file breaks it. Doc line.
- 5. A replay miss does not say why.
- 6. No process-wide strict replay switch.
- 7. `--record` into a used folder pays again and discards the answer. Help line.
- 8. Replay cannot tell a recording is stale. Waits on backlog question 1, the default-model pin.
- 9. Golden-file tests break across versions. With `2026-09-26-the-result-schema-identifier-never-versions.md`.
- 10. A token in the base path is written into every recording.
- 11. Byte-exact keys make recordings fragile. Doc line.
- 12. No way to find unused fixture entries.

## Report 10, `recognize` and `relate`

- 7. Lowering the name threshold never adds a word. Doc line in R8.
- 8. Possessives and quotes stay in names, and touching names merge. Tracked: R2, R3.
- 9. Mentions, not entities, multiply relation cost.
- 10. `recognize --relation` has no entity cap. Check within 0147.
- 11. `recognize` sends its relation rules one after another. Check R4b's scope.
- 12. `recognize --details` hides relation probabilities.
- 13. Offset units and field names change by surface.
- 14. Split texts repeat the whole text in every request. Tracked in part: R4.

## Report 11, failure and scripting

- 3. SIGTERM is unspecified and ends a run abruptly.
- 4. Killed runs leave temporary cache entries that prune ignores. With report 08 finding 12.
- 5. A stored partial reply replays forever. Tracked: ticket 0158 and `2026-09-26-recording-page-says-a-failure-is-never-recorded.md`.
- 6. A deterministic refusal on one record blocks every rerun. Tracked: 0154 and the roadmap's `--on-error` hold.
- 7. An absurd `--timeout` panics with exit 101. Quick Fix, with report 06 I-11.
- 8. Ctrl-C waits out a hung request, then blames the backend. Wording fix.
- 9. The default cache binds to one address. Tracked: ticket 0124's deferred gaps.
- 10. No catalog of error sentences with stable identifiers. With `2026-09-26-run-facts-b5-owe-cause-retryable-and-stop-record.md`.

## Report 12, security and the data boundary

- 3.1. No private TLS roots, and a certificate failure reads as a network failure.
- 3.2. Local faults are reported as network faults.
- 3.3. The configuration file is trusted whatever its mode. With `2026-09-26-folder-writers-decide-the-answers.md`.
- 3.4. The planted-text guidance is narrower than a reader will take it. Doc line.
- 3.5. Ruby result values print caller text. Tracked: `2026-09-26-ruby-result-values-inspect-caller-text.md`.
- 3.6. No release to verify. Tracked: 0128.
- 3.7. The default destination is a third-party service, and nothing says what it keeps. Doc paragraph.

## Done when

Triage after 0.1 has placed every untracked line in a ticket, a Quick Fix, an issue, or a recorded decision to leave it. Then this issue closes.
