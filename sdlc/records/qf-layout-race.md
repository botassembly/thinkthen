# Quick Fix qf-layout-race: read the marker again before refusing a folder

Status: landed after a fresh read-only review found the fix correct and raised four low findings, all fixed before landing. It fixes `sdlc/issues/2026-09-25-a-fresh-cache-can-refuse-itself-as-a-retired-layout.md`.

## Evidence it starts from

While measuring for `sdlc/records/qf-request-cost.md`, a 2,000-record `decide_many` at throttle 32 over a new empty cache folder failed once in about forty runs with `the recording folder uses a retired layout`. `identity::check` read the backend marker and found none. Another worker then published the marker and installed its first entry. The first worker's `has_entry` found that entry and refused the folder.

## Change

- `check` in `crates/thinkthen/src/engine/recorder/identity.rs` reads the marker again when it finds an entry after a missing marker. A marker found then is matched as usual: a different backend still refuses as a mismatch, and a matching one syncs the folder. With no marker, a folder that holds entries still refuses as a retired layout.
- The test-only `pause` hook gains the stage `after-missing-marker`. It also gains a resume file, so a paused child can continue instead of waiting to be killed. `test_deadline::wait_for_file` waits for that file with the existing 30 s bound. The interruption test now uses the same helper in place of its own wait loop.

## Proof

`a_marker_published_while_this_writer_looks_for_entries_is_matched` pauses a child writer after its marker read. The parent then publishes the marker, writes a digest-named entry, and resumes the child. Before the fix, the child failed with `RecordingFolderLegacy`. After the fix, it succeeds and the marker is intact. `tests/backend/cache_identity.rs` still proves that a real legacy folder refuses.

The test protects the rule that a fresh cache shared by concurrent writers never refuses itself. It fails if `check` refuses on the entry check without reading the marker again. No existing test holds a writer between its marker read and its entry check. The race needs a pause that always lands at that point, and the real boundary cannot give one. The pause hook compiles only under `cfg(test)`, and a child process carries its environment variables, so no other test can reach it.

## Size

The ratchet rises from 61,719 to 61,768, 49 nonblank lines. `identity.rs` gains 13: 9 for the fix and 4 for the new pause point and the resume branch. `test_deadline.rs` gains 13 for `wait_for_file`. `identity/tests.rs` gains 22 for the regression test and the shared `paused_child` helper, less the two repeated spawn blocks and the old wait loop. `tests/transform.rs` gains 1: the resume variable joins the list that command tests set to a trap path.

## Checks

The review fixes came first: the proof answers above, the `paused_child` helper, a timeout message that names the missing path, and the resume variable in `tests/transform.rs`. After that, main was merged in. With `THINKTHEN_API_KEY` unset, each rung ran once:

- `lint`: exit 0, with the ratchet at 61,768.
- `test`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

## Deferred

The PostgreSQL warm was not rerun here.
