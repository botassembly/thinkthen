# 0027: Compare only like cases and whole questions

Branch `ticket/0027-honest-run-comparison`. Built 2026-09-20.

## What landed

`compare.jq` now separates ids present in both runs from pairs that can actually be compared. `paired` remains the shared unique-id count. The new `compared` count includes only pairs whose evidence and trusted label match. `same` and every flip now use those comparable pairs alone.

A complete nonempty modern run identifies its resolved questions by valid 64-character lowercase hexadecimal `meta.question_sha256` values. An older, mixed, or malformed nonempty run falls back to printed question text and says `question_by: text`. An empty side returns `changed.question: null` and `question_by: unavailable`.

How-to 41 now reports the true and false description change through the question digest. Both comparison issues are closed, and the foundation order now leaves only enforcement of the live spending limit before the hands-on repair tickets.

## Red then green

The existing draft and tuned runs use the same printed question text, so the old transform reported `changed.question: false` despite three flipped answers and different question digests. The updated how-to reports `true` by digest while retaining all 24 comparable pairs, 21 same answers, and the three expected flips.

The transform page builds a small pair matrix. It includes missing ids, repeated ids, one changed input, and one row whose input, label, and verdict all change. The last row appears in both mismatch lists and in no flip list. The proof checks exactly that `compared` equals `same` plus the lengths of every flip list.

Legacy, mixed-type, malformed-string, and empty fixtures cover the identity choices. The malformed digest has 64 hexadecimal characters followed by a real line feed. A weaker line-end expression wrongly accepts it; the strict whole-string expression rejects it and selects text fallback.

## Review

The design reviewer rejected the first draft because empty inputs made the phrase "every row has a digest" vacuously true. It also raised the proof score for the legacy compatibility case. The corrected design gives empty inputs an unavailable question comparison, states the comparison partition exactly, and remains level 2 with Luna High.

The code reviewer rejected the first pass because any string counted as a digest and the mismatch fixture did not change the excluded row's verdict. It rejected the first remediation because the newline fixture held two literal characters rather than a line feed. The final pass accepted strict digest validation, the real malformed value, and the flipping mismatch proof with no remaining finding.

## Choices made where the pages were silent

Ian can overturn these choices.

- **A malformed digest selects the legacy text fallback.** The comparison remains useful for saved or hand-edited rows and names the weaker identity it used.
- **An empty side makes question change unknown.** No row supplies an identity, so true or false would claim evidence that does not exist.
- **Mismatches do not reduce `paired`.** That count continues to mean shared unique ids. `compared` names the safe subset, so existing readers keep the original pairing fact.

## Gates

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `15732/15732` |
| `sdlc/scripts/test` | exit 0, 299 tests and 2 documentation tests pass |
| `sdlc/scripts/spec` | exit 0, both transform proofs, every committed recording, and 16 green how-tos pass |

The coordinator runs the ladder with the key and base address unset. No live call runs.
