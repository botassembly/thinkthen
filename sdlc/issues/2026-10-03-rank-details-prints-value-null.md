# `rank --details` prints `"value": null` on every row

Status: open. Filed 2026-10-03 from the docs message "Draft issue for 0.2: rank --details prints value null", on Ian's request of 2026-10-02. Draft: investigate, then decide. Owner: the queue owner.
Milestone: 0.2

On 0.1.0, `thinkthen rank 'QUESTION' --top 2 --details < passages.txt` prints one result object per record. Each has `"value": null` beside an `answer` with `kind: yes_no` and a probability such as 0.81. The input was 292 lines of plain text from the docs team's transcript experiment 420. The site's transcript how-to saw the same, and the six-features experiment 422 read `.answer.probability` instead on every row.

The specification asks for this. `specification/rank.md` says a yes/no `rank` row has `question.verb: decide`, `answer.kind: yes_no`, `threshold: null` and `value: null`, and that the probability that orders the rows sits under `answer`. So this is a design question, not a code defect.

A reader expects `value` to hold something. The choices:

1. `value` holds the record's rank, 1 for the first printed row, as the SQL `thinkthen_rank` already returns a `rank` column.
2. `value` holds the ordering number: the yes probability, or the weighted value of a saved `score` question.
3. `value` stays null, and `rank --help` and `rank.md` say why and where the ordering number lives.

Choices 1 and 2 change the result schema for `rank` rows, so `result.schema.json`, `rank.md`, the bindings' `rank` results and the recordings' replays need checking. A recording made by `decide` replays under `rank` today, and the change must keep that.

Related: the search features issue `2026-10-03-search-features-positions-several-questions-and-grep-style-output.md` adds a position and a score to each result.
