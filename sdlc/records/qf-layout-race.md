# Quick Fix qf-layout-race: read the marker again before refusing a folder

Status: built, awaiting review. It fixes `sdlc/issues/2026-09-25-a-fresh-cache-can-refuse-itself-as-a-retired-layout.md`.

## Evidence it starts from

While measuring for `sdlc/records/qf-request-cost.md`, a 2,000-record `decide_many` at throttle 32 over a new empty cache folder failed once in about forty runs with `the recording folder uses a retired layout`. `identity::check` read the backend marker and found none. Another worker then published the marker and installed its first entry. The first worker's `has_entry` found that entry and refused the folder.

## Change

- `check` in `crates/thinkthen/src/engine/recorder/identity.rs` reads the marker again when it finds an entry after a missing marker. A marker found then is matched as usual: a different backend still refuses as a mismatch, and a matching one syncs the folder. With no marker, a folder that holds entries still refuses as a retired layout.
- The test-only `pause` hook gains the stage `after-missing-marker`. It also gains a resume file, so a paused child can continue instead of waiting to be killed. `test_deadline::wait_for_file` waits for that file with the existing 30 s bound. The interruption test now uses the same helper in place of its own wait loop.

## Proof

`a_marker_published_while_this_writer_looks_for_entries_is_matched` pauses a child writer after its marker read. The parent then publishes the marker, writes a digest-named entry, and resumes the child. Before the fix, the child failed with `RecordingFolderLegacy`. After the fix, it succeeds and the marker is intact. `tests/backend/cache_identity.rs` still proves that a real legacy folder refuses.

## Size

The ratchet rises from 61,719 to 61,768, 49 lines. The fix takes 9, the resumable pause and its wait helper take 16, and the test takes 30. Reusing the helper in the interruption test removed 7.

## Checks

With `THINKTHEN_API_KEY` unset, each rung ran once:

- `lint`: exit 0, with the ratchet at 61,768.
- `test`: exit 0.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

## Deferred

The PostgreSQL warm was not rerun here.
