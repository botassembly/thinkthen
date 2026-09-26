# 0144: Build the engine plans batches

Status: built 2026-09-26, awaiting a fresh code review. It lands after 0141 and 0143. Owner: Claude.

Branch `ticket/0144-engine-plans-batches`, in lane `worktrees/thinkthen-lane-3`. The ticket is `sdlc/tickets/0144-engine-plans-batches.md`. A fresh read-only design review accepted it on 2026-09-26 after two rounds of findings. The change raises the ceiling, so a second agent reviews the code and names what it checked. Ian can overturn every decision the ticket lists.

## Result

- `core/batch.rs` holds the pure `Batcher`. `push` takes a `BatchRecord` and returns the batches that closed. `finish` closes the last one. A batch closes after a content cut, after `N` records, before a record that would pass a limit, or at the end. Each `Batch` holds its plan, body, digest, each record's first wire question, and its close reason.
- Without a context, a batch of one distinct record, alone or with copies, sends today's request. Every other batch quotes each distinct record once and lists it once in `{"records":[…]}`, or beside a context.
- The batcher counts each record's batched share from one encode of that record alone. It finds the empty batch's size from a probe record encoded once and twice. It encodes each batch once at close and compares the body with the count. Planning is linear in the input bytes.
- `Batcher::new` refuses a question written as JSON beside a context, and a context over a limit before any record. `push` refuses a later record whose batch of one with the context passes a limit. Neither error holds text.
- `Reading::selected` in `core/records.rs` holds the match `Reading::evidence` had. `Reading::evidence` and the new `Reading::batch_record` both build on it, so a whole CSV, TSV or JSONL record quotes as its object.
- `Backend::relation_ceiling` is now `Backend::ceiling`. Its one caller in `engine/prepared_request.rs` follows.
- Deviation: `core/backend_profile.rs` makes `max_evidence_bytes`, `max_request_bytes` and `max_questions` `pub(crate)`, so the batcher tests a limit before it encodes. It adds no line.
- Fixtures: five `batch-*.request.json` files and a README section under `specification/fixtures/systemone/`, and `grouping.txt` with a hand-worked README under `specification/fixtures/batching/`. A Python script in the session scratchpad, `t0144/fixtures.py`, drafted the five bodies from ADR 0048 item 1 and shares no code with the planner. `jq` parses each body. The README's hashes come from `printf` and `sha256sum`.
- No specification rule, surface or marker changes.

## Tests

Seven table tests in `core/batch/tests.rs`: design test 1 (`a_batch_of_one_is_todays_request`), design test 2 (`each_batch_body_matches_its_fixture`), design test 3 (`batches_close_where_the_readme_says`), and the edge-case rows in `limits_close_batches_by_exact_bytes_and_the_ceiling`, `the_ceiling_closes_batches_at_the_built_in_address_only`, `questions_copies_and_refusals_follow_the_batch_rules` and `a_context_is_refused_without_echoing_it`.

## Plants

`t0144/plants.py` in the session scratchpad applied each plant, ran `cargo test --lib core::batch` under the heavy lock, and restored the file from a backup it made with `mktemp`, then touched it. It removed only those backups. A grep of the diff for plant text found none.

The final run used the final code after the merge of `origin/main`. Every plant turned red. The ticket's plants map to T1a to T1c, T2a to T2f and T3a to T3g.

| Plant | Result | Tests that failed |
| --- | --- | --- |
| T1a: batched form for one record | Red | `a_batch_of_one_is_todays_request`, `questions_copies_and_refusals_follow_the_batch_rules`, `limits_close_batches_by_exact_bytes_and_the_ceiling` |
| T1b: quote at a batch of one | Red | `a_batch_of_one_is_todays_request`, `limits_close_batches_by_exact_bytes_and_the_ceiling` |
| T1c: digest other bytes than the body | Red | `a_batch_of_one_is_todays_request` |
| T2a: drop the period after the quote | Red | `each_batch_body_matches_its_fixture`, `limits_close_batches_by_exact_bytes_and_the_ceiling`, `questions_copies_and_refusals_follow_the_batch_rules` |
| T2b: quote the text not its JSON | Red | `each_batch_body_matches_its_fixture`, `limits_close_batches_by_exact_bytes_and_the_ceiling` |
| T2c: plain list evidence | Red | `each_batch_body_matches_its_fixture`, `limits_close_batches_by_exact_bytes_and_the_ceiling` |
| T2d: ask a copy twice | Red | `limits_close_batches_by_exact_bytes_and_the_ceiling`, `each_batch_body_matches_its_fixture` |
| T2e: context records in the evidence too | Red | `each_batch_body_matches_its_fixture` |
| T2f: whole record quoted from its text | Red | `each_batch_body_matches_its_fixture` |
| T3a: hash read little-endian | Red | `batches_close_where_the_readme_says` |
| T3b: hash without JSON quotes | Red | `batches_close_where_the_readme_says` |
| T3c: cut closes before its record | Red | `batches_close_where_the_readme_says` |
| T3d: limit compared with >= | Red | `limits_close_batches_by_exact_bytes_and_the_ceiling`, `batches_close_where_the_readme_says` |
| T3e: ceiling ignored | Red | `the_ceiling_closes_batches_at_the_built_in_address_only` |
| T3f: batches fill across a cut | Red | `batches_close_where_the_readme_says` |
| T3g: copies checked in the batched form | Red | `limits_close_batches_by_exact_bytes_and_the_ceiling` |

## Budget

The ticket budgeted 605 lines of growth. The build stopped at stop rule 1 with 913. The coordinator raised the budget from 605 to 930 after the reviews added the context refusals, the late-overflow check and the Selected split. Ian can overturn this.

Nonblank lines against `origin/main` at `7850db3f`.

| File | Ticket budget | Measured |
| --- | --- | --- |
| `core/batch.rs` | 240 | 395 |
| `core/batch/tests.rs` | 340 | 480 |
| `core/records.rs` | 20 net | 413 to 448, +35 |
| `core/mod.rs` and `core/backend.rs` | 5 | +2 |
| `core/backend_profile.rs` and `engine/prepared_request.rs` | 0 | 0 |

`sdlc/ratchet.json` moves from 67,758 to 68,670, up 912, within the 930 ruling. Splitting the two long tests for clippy added lines. Sharing the wire-name digit count in `batch.rs` and the test backends, the structured question and a context helper in the tests took the total back under 930. Before that the build merged the two plan encoders into one and held the open batch in one value.

## Ladder

Run once each after the merge of `origin/main` at `7850db3f`, which brought ticket 0141.

| Rung | Result |
| --- | --- |
| `lint` | exit 0; ratchet 68,670 of 68,670 |
| `test` | exit 0; 950 passed, 0 failed across 36 test binaries |
| `spec` | exit 0; demos 21 green, 0 red |

`install` and `surfaces` did not run. No public library type, method or message changed. `origin/main` gained one documentation commit, `7443d69d`, after the ladder ran.

## Deferred gaps

The ticket's five stand: cross-language hash identity (B12a), the 40,000-byte records going one a batch (D1's page), the demo recordings under B4's default, reply splitting (B4), and the late-overflow policy with a context (B7).
