# 0036: Tag zero or more labels

Ticket 0036 adds `tag` as the fourth single-judgment shape. It tests 1 to 20 ordered labels independently in one request and prints every label that reaches one shared cut as a JSON array. The same question shape works in a question file and inside `annotate`.

## What landed

The pure core owns Tag labels, the default 0.5 cut, ordered probabilities, selection, canonical form, and the pinned digest. The System One adapter expands one Tag question into adjacent yes-or-no wire questions and aggregates their answers back into one result. Labels with quotes or backslashes are encoded as JSON strings inside the instruction. Existing question bytes and digests remain pinned.

The command accepts bare labels or repeated `--label LABEL=DESCRIPTION`, with the established input, record, concurrency, recording, cache, detail, and dry-run options. It refuses mixed label forms, bands, raw and quiet views, malformed lists, and unsafe text before a request. Diagnostics name options, Tag labels, and score levels separately without repeating rejected content.

## Red, review, and proof

The first command test failed because `tag` did not exist. The first listener run then found that the shared validator still required two members; Tag now has its accepted lower bound of one while choose and score retain two.

The first code review rejected generic label messages and missing proof at the expansion boundary. The remediation pins exact safe CLI and question-file messages. Its compact compiled-command matrix covers 1 and 20 labels, quoted and backslashed labels, missing and malformed wire answers, complete dry-run output without a key or connection, delayed distinguishable records, reverse-completion failure order, quiet broken-pipe stop, replay, and secrecy. Shared scheduler tests continue to own the larger concurrency matrix. The same independent reviewer accepted the remediated code before the live run and accepted the rebased branch state.

## Page 39 and live evidence

Ian authorized one cached call through the repository live wrapper to the reviewed `https://api.typesafe.ai/v1` base. The saved entry has schema `thinkthen.recording/1`, adapter `systemone`, the exact three described Tag questions, no credential marker, model `jev-1.13.0`, and usage of 447 input and 55 output tokens. Its answer is secret 0.03, destructive 0.03, and urgent 0.95. The exchange digest recomputes to its filename, `6ef8b59f659e76cea55c1657dfdd0f9ce1e62155e3dc199061d1b65499a56598`. The resolved question digest is `a2f5e81d5699ff744fecd2cc06c4c1e4a00707620771ef54479a02414e06f049`.

How-to 39 replays that entry with the key unset. It asserts `["urgent"]`, shows the successful empty array by applying a local 0.99 cut to the same recording, and keeps all three measured probabilities under `--details`.

The live ledger status after the authorized reservation reports 6,075,118 charged tokens and 469,924,882 remaining. The planning spend table records actual backend usage, so this ticket adds 447 rather than the wrapper's retained maximum reservation.

## Validation

The focused page passed its three blocks. All four repository rungs pass: install; lint with 92 resolved packages and the exact 19,936-line ratchet; tests with 15 binary tests, 165 backend tests, 9 choose-and-score edge tests, 15 decide edge tests, 11 demo-runner tests, 17 question-file tests, 2 single-test binaries, 132 core tests, 2 documentation tests, and all live-test cases; and the specification rung with 26 shell cases, 2 replay cases, every replay check, and 18 green demos. `git diff --check` passes. No second paid call ran. The coordinator has not committed or pushed this work yet.
