# Search features: positions, several questions, and grep-style output

Status: open only for remaining complete SDK adoption and final qualification under 0425. CLI search intake/display and native rank sets have landed; the numbered asks below preserve the original proposal.
Milestone: 0.2

Ian asked for ThinkThen to work as a smarter grep, so the transcript how-to needs no awk or paste. The docs team's experiment 422 measured search approaches against 15 known quotes in three talk transcripts. The best approach found 14 of 15, where the earlier search found 5. All 36 live runs cost $0.13. The docs message holds each measurement.

## The library and engine

Every binding gets these:

1. Each result carries its position in the input list, its index, and its score. A program that knows a match's index can group windows and show neighboring items itself.
2. `rank` accepts several questions in one call and merges their results by turns: the best record of each question, then the second best of each, skipping a record already placed. The merge rule lives in one place, so every binding behaves the same. Four narrow questions merged by turns found 11 of 15 quotes in 300 lines read, where one broad question found 5. Merging by highest probability found 9.
3. `--top N` stays as it is, applied per question.

## The command line

The shell has no list, so the command adds text handling on top of the library:

1. Read input files by name as trailing arguments, in order, numbering lines per file. `--input FILE` stays.
2. Group lines into windows with `--window N`. A window never crosses from one file to the next. Ten-line windows did best in experiment 422. Overlapping windows did not pay, so no overlap is the default.
3. Print the file and line number with each match with `-n`, as `grep -n` and `grep -H` do. `--details` gains the position.
4. Print surrounding lines with each match with `--around N`, as `grep -C` does. `--context` is taken. Five lines on each side raised recall from 12 to 14 of 15.
5. Print the score with each match with `--scores`. A cut at 0.3 found 11 of 15 in 420 lines read, and a cut at 0.5 found 9 in 170.

Windowing and surrounding lines stay out of the library for now. They are one line of code once results carry positions, and adding them to every binding costs more than it gives.

## Open questions for the ticket

- The flag names. The docs team proposed `@SET.json` for a question set, `--merge turns|max`, `-A`, `-B`, `-C` and `--strip REGEX`. A ticket decides which ship.
- Whether `--window N:S` with a step ships. Overlap cost twice as much for no gain.
- Each new setting must leave the question digest unchanged when unused, so cached answers and audit records still match.

A cheaper form needs no feature: one question that names all four ideas found 9 of 15 in 300 lines for a quarter of the cost. The how-to can teach it today.

Related: `2026-10-01-rank-keeps-only-records-over-a-threshold.md`, `2026-10-03-rank-details-prints-value-null.md`, and the later idea `2026-09-30-batch-command-runs-many-questions-in-one-process.md`.

## Reconciliation, 2026-10-08

Ticket 0401 delivered intake, windows, numbers, scores, neighbors and native/CLI rank sets; `57de974e8` lands display, and `ffb1b253d` finalizes CLI rank positions. [0401 record](../records/0401-search-intake-and-display.md) states the reviewed behavior: top applies after turns merging, replacing the original per-question-top proposal below. Typed ordered SDK members landed at `b835816b6` and `0d78347db`; [0432 qualification](../records/0432-shared-parity-cases.md#final-installed-qualification-2026-10-08) covers the installed consumers at `60f0dcb9a`. The public [rank contract](../../specification/rank.md) governs exact ordering and complete member facts. Current Request adoption remains with the 0491 family tickets, and final platform/release proof remains with 0425. Transcript-search recipe publication stays later; it is not missing runtime behavior.
