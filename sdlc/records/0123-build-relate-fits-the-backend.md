# 0123: Build "relate splits requests to fit the backend"

Status: built; awaiting code review. Owner: Claude.

Branch `ticket/0123-relate-fits-the-backend`. Ticket `sdlc/tickets/0123-relate-fits-the-backend.md`, accepted at `023b9ae1`. Ian can overturn every choice this record marks as decided.

## Result

- `Backend::relation_ceiling` in `core/backend.rs` returns 96,000 when the resolved posting URL equals `https://api.typesafe.ai/v1/systemone` byte for byte, and nothing otherwise.
- `SettledRelation::settle` passes that ceiling to `PreparedRequests::with_profile`, unless the profile names `max_request_bytes`. Every other caller passes `None`. The ceiling reaches `relate` and the relation step of `recognize`.
- `PreparedRequests::with_profile` grows each chunk by doubling from one question, stopping at the remainder. It then halves the gap to the first count that does not fit. It keeps the longest fitting prefix and digests only that chunk. One question alone always passes the ceiling. When one question alone fails the profile, the splitter returns that error, as the old loop did. `ProfileLimit::permits_split` lost its last caller and was removed. Any failure now counts as "does not fit" during the search. The non-split limits (evidence bytes and options) belong to single questions, so the chunk stops before the failing question and the next chunk returns that question's own error. The old loop returned the same error with the same counts.
- `engine/http.rs` reads a 400 body with `.limit(4096).read_to_vec()` and reads only `detail.error_type`. The exact value `max_tokens_exceeded` becomes `Error::TokenLimit`, which carries no data, has `Kind::Backend`, and is not retried. Every other body stays `Error::Status(400)`. The command prints the ticket's fixed sentence. `public/error.rs` keeps `the backend answered with status 400`. `check` treats it as a probe row, as it does status 400.
- `relate --either @FILE` prints its own refusal sentence. `relate --help` hides `--jobs` through `mut_arg` and carries the cost sentence.

## Measurements

All offline dry runs with no key. The main build was `origin/main` at `08a8e754`.

| What | Number |
| --- | --- |
| The first request of the Beatles set at a loopback address with `max_questions` 142 (the 142 `appears_on` questions that passed live) | 126,820 bytes |
| The ratio 65,423 input tokens over those bytes | 0.516 tokens a byte (stop rule: 0.60) |
| 96,000 bytes at that ratio | about 49,500 tokens |
| Beatles `appears_on` at a loopback address, main and this branch | one request, 161,252 bytes, digest `fd43d9c9…` on both |
| Beatles plan at the built-in address, this branch | `sung_by` 81,943; `appears_on` 95,779 and 76,411 |
| The deck's own run (`examples/graph.json`, `beatles.jsonl`), no profile, built-in address, by hand | 85,458; then 95,779 and 76,411 |
| The deck's run with its `jev.json` (`max_questions` 64) | 36,800, 36,866, 33,771, 63,154, 63,220, 56,829 |
| 40 line entities, `linked`, loopback, `max_request_bytes` 20,000, main and this branch | 19,938, 19,966, 19,923, 19,923, 19,923, 19,924, 16,660 |
| 255 line entities, release, loopback, no profile | one request of 5,386,112 bytes, 0.16 s with `jq` |
| 255 line entities, release, built-in address | 63 requests, largest 95,998 bytes, 0.62 s with `jq` |
| 255 line entities, debug, loopback, no profile | 0.71 s (ticket limit: 10 s) |
| 255 line entities, debug, built-in address | 3.69 s |

No stop rule was crossed on these numbers.

## Tests

New file `tests/backend/relate/ceiling.rs`, five tests, all dry runs through the harness spawn. `tests/backend/exchange.rs` gained the body rows of the status table. `tests/backend/refusals/relate.rs` gained the `--either @FILE` row. `tests/relate_edge.rs` pins the cost sentence and the absence of `--jobs`. The four questions are answered in the ticket's Acceptance section. None uses a test-only hook.

## Planted faults

Each fault was planted in the source, the named test ran, and the file was restored from Git and touched.

| # | Planted fault | Test | Result |
| --- | --- | --- | --- |
| 1 | `relation_ceiling` returns nothing | `the_beatles_set_splits_at_the_hosted_address_and_nowhere_else` | red |
| 2 | `relation_ceiling` returns the ceiling at every address | same | red |
| 3 | The ceiling stays beside a profile's `max_request_bytes` | `a_profile_byte_limit_replaces_the_ceiling_and_other_limits_join_it` | red |
| 4 | One question over the ceiling is refused | `one_question_over_the_ceiling_goes_alone_and_is_not_refused` | red |
| 5 | Each chunk keeps one question fewer than the longest fit | `the_splitter_keeps_the_longest_fitting_prefix_the_old_loop_chose` | red |
| 6 | Chunks grow one question at a time, the old loop's cost | `a_full_line_set_plans_inside_the_child_deadline` | red, killed at the 60 s child deadline |
| 7 | The 400 body is never read | `common_request_statuses_name_fixed_actions_and_hide_the_body` | red |
| 8 | Any `error_type` counts | same | red |
| 9 | The body is read up to the 1 MiB reply bound | same | red |
| 10 | `--either @FILE` returns the grammar sentence | `no_refusal_on_any_command_writes_the_key_quotes_the_evidence_or_sends_anything` | red |
| 11 | `mut_arg` dropped | `help_names_the_beta_complete_set_and_secrecy_contract` | red |
| 12 | The cost sentence reworded | same | red |

## Lines and the ratchet

Main measured 61,768 nonblank lines at `02dc0b96`. This branch measures 62,054, so the ratchet rises by 286.

| Part | Budget | Nonblank lines, net |
| --- | --- | --- |
| `engine/prepared_request.rs` | 30 | 28 |
| `core/backend.rs` | 12 | 12 |
| The 400 reason: `http.rs` 27, `error.rs` 3, `failure.rs` 3, `failure/status.rs` 2, `convert.rs` 1, `public/error.rs` 1, `check.rs` 0 | 45 | 37 |
| Wording: `args/command.rs` 7, `relate/config.rs` 2 | 12 | 9 |
| `tests/backend/relate/ceiling.rs` and its `mod` line | 200 | 158 |
| `tests/backend/exchange.rs` | 30 | 21 |
| `tests/backend/refusals/relate.rs` and `tests/relate_edge.rs` | 20 | 20 |
| Other call sites (`facade.rs`, `facade/recognize.rs`, `backend_profile.rs`) | none | 1 |
| Total | 350 | 286 |

Where I looked for duplication first: the status phrases in `cli/failure/status.rs` hold one new constant and no copy. `core/backend_profile.rs` lost `permits_split`, and its new `limits_request_bytes` replaces it line for line. `checked_body` now serves both the single request and the splitter, which removed the old `prepare_chunk` copy of the encode-and-check lines. The relate test helpers in `tests/backend/relate.rs` send to a listener with a key, so the dry-run tests keep their own small `plan` helper.

## Rungs

RUNGS
