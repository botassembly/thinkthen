# The `recognize` brief for the experiment team

Status: Closed on 2026-09-22. Both experiments built against the brief and closed on it.

Ian ruled on 2026-09-21 that `recognize` is the ninth function, and that the marketing side rules its shape. The shape is `sdlc/planning/recognize-design.md`. This page tells the recognize experiment and the surfaces experiment what to build against it. Every paid run goes through `sdlc/scripts/live` under a token cap. Nothing here touches the thinkthen source.

## For the recognize experiment

Build a stand-in command that matches the design page exactly, so the build team ports a proven shape.

1. **The command line.** `recognize [KIND]...`, `--kind KIND=DESCRIPTION`, `--relation NAME=FROM:TO`, `--threshold`, `--relation-threshold`, `--details`, `--dry-run`, and the `@file` form. The default kinds are `person`, `organization`, and `place`.
2. **The relation rule.** Every rule has a direction. Each end is a kind or `*`. Cover all four forms with a test each: kind to kind, kind to any, any to kind, and any to any. `"either": true` asks a both-ways relation once per pair. A missing end is a usage error. A name is never paired with itself. `--dry-run` prints the pair count.
3. **The output.** The object on the design page: the user's kind word in `kind`, never `PER`. `name`, `from`, and `to` on a relation. `token_start` and `token_end` only under `--details`. Re-emit the Maria Chen demo in this shape from the saved recording, at no cost, and put the path in this issue. The deck slide then shows `person` in place of `PER`.
4. **Records.** `--lines` gives one object per line. `--jsonl` gives the record back with the object under `recognize`.
5. **Pieces and offsets.** State how a long text is cut. Prove with a test that offsets index the whole text, on a text of at least three sentences with a name in the last one. Report what happens to a name that crosses a cut.
6. **The request count.** `--dry-run` prints the number of requests. Measure requests and tokens for a text of 50 words, 500 words, and 5,000 words.
7. **Two texts for the cases file.** One with no names, which must exit 0 with an empty list. One that names the same place twice, which must give two entities with two ids.

## For the surfaces experiment

`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md` is the acceptance test. Each call there runs as written against the stand-in engine, on all nine surfaces.

1. **Databases first.** DuckDB returns a list of structs. SQLite is a table-valued function. PostgreSQL is a set-returning function used with `LATERAL`. Prove the "names become rows" pattern on each: build a `mentions` table, then join it to an ordinary table by equality, and show the join sends no request.
2. **The join cost, measured on the stand-in.** Count the requests for `JOIN ... ON thinkthen_decide(...)` at 10 by 10, 100 by 100, and 1,000 by 1,000 rows. Then count them again with an ordinary condition beside the meaning condition. Report, for each engine, whether the cheap condition ran first, and the subquery that forces it when it did not. The manual needs this table.
3. **Offsets per language.** `text[start:end]` must give the name in each host's own string indexing. Test with a text that holds an accented letter and an emoji before the name.
4. **The any-kind end** is the string `"*"` everywhere except Rust, where it is `Kind::Any`.
5. **C returns the JSON string** and one free function. Prove no leak under the existing leak check.

## `relate`, added the same day

Ian ruled that `relate` is built with `recognize`. The shape is `sdlc/planning/relate-design.md`, and the first real output is `experiments/225-relate-demo/`.

1. The recognize experiment builds the stand-in `relate` on the same pair-asking code as step three of `recognize`. One path, two commands.
2. Test the four rule forms, `--either`, `--kind-field`, the 255-record refusal, and the pair count under `--dry-run`.
3. Measure pick-one against a yes-or-no question per relation, on pairs that truly hold two relations.
4. Measure pairs against one question per subject, at 10, 50, and 200 records: accuracy, requests, and tokens.
5. Measure whether `same_as` groups come out the same when the record order changes.
6. The surfaces experiment proves the three database forms, and one recursive query over the edges in each engine.

## What comes back

One short report per experiment, the paths in this issue, and a list of every place the design page was wrong or unclear. The design page changes to match what was learned. The experiment never bends the shape quietly.
