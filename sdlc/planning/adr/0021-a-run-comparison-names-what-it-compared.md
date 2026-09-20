# ADR 0021: A run comparison names what it compared

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

`compare.jq` pairs two runs by case id and reports changes in their answers. It currently counts a pair even when the evidence or trusted label changed, and it detects a question change from printed question text alone. Ticket 0017 added `meta.question_sha256`, which covers the resolved verb, question text, true and false texts, options and descriptions, levels, and threshold. Two question files can therefore have the same printed text and make different judgments while the comparison says the question stayed fixed.

## Decision

- `paired` remains the count of unique ids present in both runs. A new `compared` count names the pairs whose evidence and trusted label match.
- `same` and `flips` read only comparable pairs. `mismatched_input` and `mismatched_label` continue to list every excluded id. One id may appear in both lists.
- `changed.question` compares the unique `meta.question_sha256` values when every row in both runs carries a valid 64-character lowercase hexadecimal digest. `changed.question_by` says `digest`.
- If any row lacks a valid digest, or carries a digest with another type or format, both runs fall back to their printed question text. `changed.question_by` says `text`. This keeps rows from before ticket 0017 usable and makes the weaker test visible.
- If either run is empty, no question identity can be compared. `changed.question` is null and `changed.question_by` says `unavailable`.
- Model and threshold change checks keep their current rules. Missing cases and repeated ids remain unpaired and excluded from `compared`.

## Consequences

The existing fields keep their meanings except that `same` and `flips` stop counting pairs the transform already identifies as incomparable, and an empty run no longer claims that its absent question did or did not change. The new count makes the partition visible: `compared` equals `same` plus the lengths of all flip lists. Existing rows need no migration.

Text fallback cannot detect a change confined to true or false descriptions, option descriptions, levels, or another field omitted from the printed text. The output names that limitation. A complete modern run uses digests.
