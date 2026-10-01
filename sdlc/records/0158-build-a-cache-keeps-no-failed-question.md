# 0158: Build a cache that keeps no failed question

Status: built 2026-09-26, awaiting code review. Owner: Claude.

Branch `ticket/0158-a-cache-keeps-no-failed-question`, in the worktree `thinkthen-0158`. The ticket is `sdlc/tickets/0158-a-cache-keeps-no-failed-question.md`, accepted at `24a9c406`, and the build started from `dc19a923`, merged with main. ADR 0053 item 6 and its amendment rule the behavior. Ian can overturn every decision the ticket lists. No live call ran. Every rung and plant ran with `THINKTHEN_API_KEY` unset, against the loopback conformance backend.

## Result

- `Recorder::caches()` is true when the recorder both records and replays. That covers `--cache DIR`, `--record DIR --replay DIR` on one folder, `THINKTHEN_CACHE` and the default cache.
- `Reply::failed_any()` is true when any outcome is `AnswerOutcome::Failed`.
- `ask_prepared` cancels the write permit in place of finishing it when the recorder caches and a live reply failed a question. It still returns the reply, so the run prints and exits as before.
- `Recorder::prepare_checked` takes a `complete` check. `ask_prepared` passes one that returns false only under a cache, and only for an entry that decodes with a failed question. The recorder reads such an entry as damaged, so it takes the digest's lock, checks again, sends, and replaces the entry only when the new reply is complete. An entry that fails to decode as a whole passes the check, is replayed, and stops the run at exit 4 as before.
- `prepare_cancelled` keeps its signature and is now compiled for tests alone. `prepare_in` derives the entry name from the digest, which keeps it under clippy's argument limit.
- `recording.md` and `records.md` carry the ticket's page sentences. `relate.md` is unchanged.
- `sdlc/issues/2026-09-26-recording-page-says-a-failure-is-never-recorded.md` moved to `sdlc/issues/closed/`.

## Tests

`crates/thinkthen/tests/backend/cache_partial.rs` holds two outside-in tests. Each spawns the compiled command against `conformance_backend::Backend`, with a private `XDG_CACHE_HOME`, and pins standard output, the exit code and the backend's running request count after every run.

- `a_cache_keeps_no_failed_question` covers edge rows 1 to 5. The row-1 check reads the default cache folder under the private home.
- `a_cache_reads_a_partial_entry_as_a_miss` covers edge rows 7 to 11. It plants the partial entries the ticket names by rewriting the `response` line of a recorded entry and keeping its request line.

Both went red before the fix: the default cache kept an entry, and a cached run against a partial entry sent nothing. Both are green after it.

The four questions:

- **What behavior does it protect?** A cache keeps only a complete reply and reads a partial entry as a miss. `--record` alone and `--replay` alone keep and replay a partial reply byte for byte.
- **What credible regression fails it?** Plants (a) to (h) below.
- **Why does no existing test catch it?** No earlier test sent a partial reply twice through a cache or read a partial entry through one.
- **Does it need a test-only hook?** No. The arms are ordinary replies, the planted entry is an ordinary file, and `XDG_CACHE_HOME` is where the default cache lives.

Edge row 6, two concurrent runs, rests on `cache_locking::backend_and_decode_failures_release_the_digest_lock`, as the ticket says. That test passes in the `test` rung.

## Plants

Each plant edited `engine/request.rs` or `engine/recorder.rs`, ran `cache_partial` under the heavy lock, and restored both files from copies saved in the session scratchpad. `git diff` showed a clean tree after every restore. All eight ran twice, the second time on the final test.

| Plant | Result |
| --- | --- |
| (a) Install every decoded reply | Red. Row 1's no-entry check fails after the first run, and the arm row's entry comparison fails |
| (b) Skip the write under `--record` too | Red. The `--replay` row exits 5 |
| (c) Skip the write for every reply under a cache | Red. The complete-reply row sends 2 requests where 1 is pinned |
| (d) `caches()` reads `cache_answers`, false for two options on one folder | Red. The no-entry check fails after that row's first pass. The ticket expected the second pass's count to fail. The entry check runs first and catches the same fault one step earlier |
| (e) Replay a partial entry under a cache | Red. The cached run against the planted arm entry sends nothing |
| (f) Run the check under `--replay` alone | Red. Both `--replay` rows exit 5 |
| (g) Replace a partial entry with a reply that also failed | Red. The arm row's entry no longer equals the planted text |
| (h) Read an undecodable entry as a miss | Red. Row 11 sends 1 request and exits 0 |

## Budgets

Nonblank lines, net against the build's start.

| Item | Budget | Measured |
| --- | --- | --- |
| `request.rs`, `recorder.rs`, `reply.rs` together | at most 35 | 34 (7, 20, 7) |
| `cache_partial.rs` | at most 150, and one `mod` line | 146, and one `mod` line |
| Pages under `specification/` | at most 8 | 0 |
| `sdlc/ratchet.json` | at most 195 above main | 181, from 69,249 to 69,430 |

No dependency was added.

## Ladder

| Rung | Result |
| --- | --- |
| `install` | exit 0 |
| `lint`, with `THINKTHEN_PRIVATE_NAMES` set | exit 0. Its log holds one bare "Killed" line between two `cargo deny` passes. It came from inside the rung, and the rung still exited 0 |
| `test` | exit 0 |
| `spec` | exit 0, demos 21 green |
| `surfaces` | exit 0: every surface passes, plus the release smoke |

## Deviations

- Plant (d) went red one assertion earlier than the ticket said, as the table records.
- Plant (c) was planted as "skip the write for every reply under a cache". Skipping it for every reply in every mode also turns the test red, but at the `--replay` row first. The narrower plant shows the complete-reply row's own reason.
- The issue named ticket 0163 as the sentence's owner after this ticket. This ticket rewrote the sentence as its own text requires, so ticket 0163 no longer needs to. The coordinator may want to drop it from 0163.

## Left for landing and later

- A fresh read-only Claude session reviews the diff. The ceiling raise needs that review too.
- Ticket 0146 merges `records.md`, `tests/backend/main.rs` and the ratchet after this lands.
