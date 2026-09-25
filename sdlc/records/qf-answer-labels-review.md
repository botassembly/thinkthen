# Review of Quick Fix qf-answer-labels

Three fresh read-only Opus reviews ran in turn. None wrote a file.

## First review, of 8aa3588e

Findings:

1. `Response` and `ResponseAnswer` in `core/adapters/systemone/response.rs` derived `Debug` and key their odds by label. Fixed: both derive `Debug` only in tests.
2. The record said "landed" with Checks pending, and main had moved to 0085. Fixed: main merged, ratchet re-measured, ladder recorded before landing.
3. The new `Debug` impls in `core/answer.rs` sat away from their types. Fixed: each impl sits next to its type.

It checked every `Debug` impl in the diff, every holder of an answer, the removal of `Request`'s `Debug`, the four questions, the ratchet, and the library tests.

## Second review, of 8028c45b

The test then drove `Engine::judge` from ticket 0085 over a loopback listener. Findings:

1. Lint failed with `clippy::too_many_lines` on the test. Fixed: the listener and engine setup moved into `loopback_engine` in the same file.
2. The test built its key with the test-only `Key::of`. Fixed: it uses the production `Key::new`.
3. `PreparedRecording` in `engine/recorder.rs` derived `Debug` over a replayed body keyed by labels. Fixed: it derives `Debug` only in tests.

## Third review, of 66b80403

ACCEPT. It confirmed all three fixes, ran lint (exit 0, ratchet 49884/49884) and the library tests (327 passed, 0 failed, 8 ignored), and matched the record's line counts to the diff against main.
