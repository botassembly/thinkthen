# Area 20: Record and replay with no network

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Record and replay lets a run store its answers into a folder (`--record`), answer from that folder alone with no network and no key (`--replay`), merge old and live folders into a committed fixture (`cache convert`), and keep demos and tests offline on those fixtures.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/`, `demos/` or `probes/`. Line counts are nonblank lines. ADR 0104 is about annotate's record-failure policy and not recording, so this report leaves it out.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code, fixture and convert | `engine/store/fixture.rs` (263), `engine/store/convert.rs` (157), `cli/cache.rs:55-78` (convert command) |
| Code, old-entry reader | `core/recording.rs` (197: `Exchange::digest`, `Entry`), `core/recording/convert.rs` (197) |
| Code, replay mode | `engine/pipeline.rs:280-296` (`Engine::store`), `engine/store.rs:113-159` (replay branch of `Store::open`), `cli/failure/recording.rs` (miss sentence at `:74`) |
| Tests | About 40 tests. `tests/backend/recordings.rs` (12) and `recordings/replay_context.rs` (2), `recording_conflicts.rs` (2), `recording_durability.rs` (3), `cache_convert.rs` (5), `core/recording.rs` unit (3), `engine/store/tests.rs` replay rows (4 of 8). 40 test files hold `--replay` |
| Demos and scripts | `demos/27-test-with-no-network`, `sdlc/scripts/demos` and `demos-self-test`, `probes/replay-check.sh`. 20 committed `demos/*/recording/thinkthen.jsonl` folders |
| Contract | `specification/recording.md` "The options", "What replay changes in a result", "A recording holds the evidence", "Converting a folder to the question fixture"; ADR 0111 section 9, 0020, 0023, 0100 (0020 and 0023 describe the retired digest store) |

## Complexity: 3 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 2 | About 970 nonblank lines: fixture 263, convert 157, old-entry reader 394, replay mode and failures about 100, convert command 24 |
| States and concurrency | 2 | Sequential. Five modes chosen from two options and one flag (`engine/pipeline.rs:282-289`). `cache convert` holds the live file's write lock from read to removal and writes by temporary file and rename (`engine/store/convert.rs:46-69`). A replay reads its fixture once per engine (`engine/store/fixture.rs:234-249`) |
| Rules and refusals | 3 | About 20: the replay miss sentence, both-files refusal, hot journal, path is a file, four fixture-line refusals (`engine/store/fixture.rs:109-123`), four convert skip or refuse reasons (`core/recording/convert.rs:62-71`), three convert exit-5 conditions, the `--quote` rule, two usage errors for two folders or `--plan` |
| Surfaces touched | 5 | All 22. 20 of 21 bindings hold replay files in their checks (Polars alone holds none), and the command owns the options |
| Settings | 3 | Six rows: Recording, Answer cache, Convert quoted form, Unused entry report, Model (alias refresh), Address (collision check) |
| Contract weight | 4 | Five spec pages (`recording.md`, `settings.md`, `result.md`, `records.md`, `channels.md`) and four ADRs (0111, 0020, 0023, 0100): 9 |
| Churn and debt | 4 | Rewritten on 2026-09-30 (the old recorder and request path removed by `19166848a`, the fixture and convert written in `7efc6c7ab`). 11 commits since 2026-09-23 on the paths. Two open issues: `2026-09-30-site-replay-folders-have-no-fixture.md` and `2026-09-30-spec-no-calls-edges-need-a-real-send.md` |

Mean 3.3, rounded to 3.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The code does what `recording.md` says on the primary paths. `--replay` opens no connection to a backend and a miss exits 5 with the key named (`cli/failure/recording.rs:74`, `tests/backend/state.rs:366`). Both options on one folder are a cache (`tests/backend/recordings.rs:317`). Convert is deterministic: fixture first, then the live file, then old entries, newer `taken_at` wins, ties keep the first, and converted answers take `taken_at` 0 (`engine/store/convert.rs:46-69`, `engine/store/fixture.rs:62-76`). Gaps. (1) The contract gives two skip sentences for convert. The code has a third reason, "a question in its request is not one this version decodes" (`core/recording/convert.rs:68`), and the spec says an entry with no decodable answer is skipped "the same way" as a non-rejoining one, but the code uses a different sentence (`:69`). A reader cannot predict the text. (2) ADR 0020 says one digest has one immutable response and a different response is exit 5, and ADR 0023 says replay reads one entry by name. ADR 0111 replaced both, and its amends table (`sdlc/planning/adr/0111-question-cache-and-one-batching-path.md:246-258`) lists neither. The spec is right and the ADRs are stale with no notice. (3) The public site still holds 15 replay folders of old digest files, which `--replay` no longer reads: `git ls-files` finds 15 `site/` folders with digest JSON and no `thinkthen.jsonl` (open issue). That issue reports 79 of 97 site examples failing in the smoke. Marketing owns the conversion, and the conversion was proved on a scratch copy. Page prose that says replay works with those folders would mislead until then |
| Reliability | B | Failure paths count requests or pin exit codes. A replay against a dead address and with no key is exit 0 with `cached: true` and zero requests (`tests/backend/recordings.rs:102-167`). Replays open no connection, counted (`:275`). A replay miss is a local failure that names only proved sources (`tests/backend/recordings/replay_context.rs:15-78`, counted listener at `:59`). Every command refuses recording storage before a key read or request, counted (`tests/backend/recording_durability.rs:39-73`). A file-size limit returns the fixed failure (`:116`). Convert waits for a live writer (`tests/backend/cache_convert.rs:345-395`). Weak spots: the two plain replay tests do not count requests, one relying on a dead listener (`recordings.rs:102`) and the other on the real default address with no key (`:170`), against the rule in `CLAUDE.md` that "sends nothing" is proved by counting loopback requests. `cache_convert.rs:358` sleeps 300 ms to hold a lock, and `recording_durability.rs:81-110` uses a 150 ms reply delay to overlap two processes but asserts nothing that needs the overlap. The code was rewritten in the last 7 days, which caps the grade at B, and three review-fix commits followed the landing (`9ea8f9f3e`, `18b537f3e`, `469354f5e`) |
| Maintainability | B | One owner for the fixture format (`engine/store/fixture.rs`) and one for the old-entry reader (`core/recording/convert.rs`). Every refusal is fixed text and carries no stored field (`core/recording.rs:22-44`, tests at `:182-223`). No lint suppression. Weaknesses: `core/recording.rs` is named for recording but now holds the request digest (still used by dry runs and attempt observations through `Recorded::new`, `engine/pipeline/send.rs:127`, `engine/facade/each.rs:141`, `engine/facade.rs:383`) and the old-entry reader, two jobs under one name. `Converting::Unreadable`'s `reason()` arm (`core/recording/convert.rs:66`) is unreachable, because `old_entries` returns on `Unreadable` before it asks (`engine/store/convert.rs:99-101`). The test helper `digest` repeats the hash (`crates/thinkthen/tests/backend/support.rs:73`). 12 of the 27 tracked folders holding digest entries are `probes/` history by design (11 of them with no fixture) |

## Strengths

- Replay and live runs share one key and one store, so a recording cannot answer a question the live path would not ask (`engine/pipeline.rs:280-296`).
- A fixture is text, sorted and reproducible: converting the same folder twice writes the same bytes (`engine/store/fixture.rs:80-103`). A line that fails to parse is refused by number, never by content (`:131-139`).
- Each old entry is decoded against its own request, rejoined byte for byte and only then keyed, so a converted answer cannot hash bytes that were never sent (`core/recording/convert.rs:77-100`).
- `cache convert` is crash-safe: it writes a synced temporary file, renames it, syncs the folder, and removes the live file last (`engine/store/convert.rs:142-158`, `:65-67`).
- The demos replay committed fixtures under a runner that is itself proved against pages that break each rule (`sdlc/scripts/demos`, `sdlc/scripts/spec` calling `demos-self-test`). 20 demo folders hold a fixture.

## Cleanup

1. **Convert the 15 site replay folders.** Where: the list in `sdlc/issues/2026-09-30-site-replay-folders-have-no-fixture.md` ("Fifteen replay folders"). Why: the public examples fail on replay until `thinkthen cache convert` runs on each, and the smoke stays red. The command is in the issue. Marketing owns `site/`. Size: M. Blocks 0.1: yes (a public page cannot be reproduced; the issue's own "before the site goes public with 0.1").
2. **Write one sentence for each convert skip reason.** Where: `specification/recording.md:115` and `core/recording/convert.rs:62-71`. Why: the spec names two sentences and the code emits three or four, and "the same way" is wrong for the no-answer case. List each sentence and pin each in `tests/backend/cache_convert.rs`. Size: S. Blocks 0.1: no.
3. **Mark ADR 0020 and ADR 0023 superseded by ADR 0111.** Where: `sdlc/planning/adr/0020-a-recording-entry-does-not-change-underneath-a-request.md`, `0023-replay-reads-the-requested-entry.md`, and the amends table in `0111-...md:246-258`. Why: they describe immutable digest entries that no longer exist, and "an unfindable decision was not made" cuts the other way for a decision that reads as live. Size: S. Blocks 0.1: no.
4. **Count requests in the two plain replay tests.** Where: `tests/backend/recordings.rs:102`, `:170`. Why: `CLAUDE.md` says to prove "sends nothing" by counting loopback requests. Point each at a counted listener and assert zero. Size: S. Blocks 0.1: no.
5. **Split or rename `core/recording.rs`.** Where: `core/recording.rs:46-104`, `:106-132`. Why: request digest and old-entry reader share a file named recording. Name the digest by what it is used for (attempt identity) and move it beside `core/pack`, or rename the file. Size: S. Blocks 0.1: no.
6. **Delete the unreachable `Unreadable` reason.** Where: `core/recording/convert.rs:66`. Why: dead text that looks like a contract. Size: S. Blocks 0.1: no.
7. **Turn `spec-no-calls-edges` into a list.** Where: `sdlc/issues/2026-09-30-spec-no-calls-edges-need-a-real-send.md`. Why: the evidence sits in a private list and the issue closes "when the current list is fed back". A count of edges fixed and edges still open would let a reviewer tell whether replay's own "sends nothing" claims are among the third that need a real send. Unconfirmed which they are. Size: M. Blocks 0.1: no.
8. **Wait on an event, not a sleep, in the convert lock test.** Where: `tests/backend/cache_convert.rs:358`, `tests/backend/recording_durability.rs:81-110`. Why: wall-clock waits in routine tests (ticket 0352 is landing on main and may already cover them). Size: S. Blocks 0.1: no.

## Confidence: medium-high

Read: `engine/store/fixture.rs`, `engine/store/convert.rs`, `core/recording.rs`, `core/recording/convert.rs`, the convert command, `Engine::store`, `Store::open`, the recording page in full, ADR 0111 section 9 and amends table, ADR 0020 and 0023, both open issues, the `demos/27` page, `probes/replay-check.sh`, and the key tests in `recordings.rs`, `recording_durability.rs` and `cache_convert.rs` around the replay and lock cases. Counted folders with `git ls-files`.

Not checked: no test or build was run. `demos/27`'s recording and `sdlc/scripts/demos` were read only to their opening sections, and the release QA suite's edge list is private. The smoke figure (79 of 97) comes from the open issue, not from a run. `tests/backend/recordings/replay_context.rs` was read in part. The bindings' replay checks were counted by file, not read. Ticket 0352 is landing on main after this commit; item 8 may be done there.
