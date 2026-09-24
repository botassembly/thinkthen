# Quick Fix qf-debug-withholds: withhold request state and record labels under Debug

Status: landed. It closes `sdlc/issues/2026-09-24-debug-follow-ups-from-the-0102-review.md`. A fresh read-only Opus review is in `sdlc/records/qf-debug-withholds-review.md`.

## Result

- `Request` in `core/adapters/systemone/request.rs` has its own `Debug`. It prints `Request { state: <24 bytes withheld>, model: "jev-latest", .. }`. The length is the byte length of the state's JSON line. The wire question types derive `Debug` only in tests, so nothing outside a test prints a pick's criteria.
- `Labels` withholds every name under `Debug` and prints `Labels(<27 bytes withheld>)`. A list cannot tell whether `choose --options` read it from a record, so it withholds names from the command line and question files too. Question text still prints.
- `Withheld(usize)` in `core/text.rs` prints `<N bytes withheld>`. It replaces the six `format_args!` sites and the hand-written `RelationEntity` line.
- `Record` prints its kind and byte length, such as `Record(text, <24 bytes withheld>)` and `Record(json, <51 bytes withheld>)`.
- `Token` prints `byte_start` and `byte_end` beside `start` and `end`.
- `request(plan)` builds the body before `encode_raw` writes it, so a test can print a `Request`.

The review found that answers still carry record labels under `Debug`. That is filed as `sdlc/issues/2026-09-24-answers-still-print-record-labels-under-debug.md`.

## Tests

- New: `no_record_label_or_request_debug_line_shows_the_evidence` in `crates/thinkthen/src/cli/failure/tests.rs`. It builds a `choose --options` question from a JSON record whose first label is the evidence marker, a plan over the marker, and the request body. It prints the question, the plan, and the request in plain and pretty form, and refuses the marker. It pins the exact `Request` and `Question` lines.
- Extended: `no_recognize_debug_line_shows_the_evidence` pins the exact `Token` line for `é` plus the marker, so byte and character places differ. It pins both `Record` lines. The text record starts with `é`, so a character count fails.
- Changed: `a_question_shows_its_own_text_in_debug_and_withholds_its_labels` in `core/question/tests.rs`, renamed from the test that said labels show.

Red first: before the fix, the new test printed `Labels([Label { name: "marker-evidence-7b3ac5", ...` and `Request { state: String("marker-evidence-7b3ac5"), ...` and failed on the marker. The recognize test failed on the exact `Token` and `Record` pins.

## Ratchet

The ceiling rises from 48140 to 48212. The two secrecy tests take 41 lines. The `Request` and `Labels` impls, `Withheld`, and the `Record` rewrite take the rest, less the lines the `Withheld` sites gave back.

## Checks

With `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset, at `8524cec2`, with the one-minute load between 8.2 and 9.5:

- `install`: exit 0.
- `lint`: exit 0, `ratchet: crates + conformance 48212/48212`.
- `test`: exit 0, 782 passed, 0 failed, 6 ignored across 18 result lines, `live-test: all cases passed`.
- `spec`: exit 0, `demos: 21 green, 0 red`.
- `sdlc/scripts/live` did not run.

Main then moved by `3dc68449`, a planning note only. The merge re-measured the ratchet at 48212/48212. The whole ladder ran again at the commit that adds this section, which is the commit that lands, and the landing commit on main records that run.
