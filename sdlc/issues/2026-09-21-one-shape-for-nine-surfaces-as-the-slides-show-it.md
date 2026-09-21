# One shape for nine surfaces, as the slides show it

Status: Open. It feeds the ADR 0017 rewrite and the database ADR. It authorizes nothing.

Ian asked on 2026-09-21 for one slide per language and one per database, drawn as the finished product, "because that'll help make sure that we're driving towards the right shape". He gave the marketing side the job of keeping the shape of the libraries good. The marketing repository holds the nine code samples in its ThinkThen deck folder, in `surfaces.md`. Writing one example through ten surfaces showed where the planning pages and the experiments disagree. This page picks one answer for each. Ian can overturn every pick.

## The picks

1. **The same names everywhere.** The eight verbs, `question`, and `details`. The prefix is the host's habit: `tt.` in Python and TypeScript, `ThinkThen.` in Ruby, `tt_` in R, `thinkthen::` in Rust, and `thinkthen_` in C and SQL. No surface renames a verb, and no surface adds one. Ruby's `decide?` goes: one spelling is easier to teach than two.
2. **The first argument is the question, as text or as a built question.** `decide("...", text)` and `decide(refund, text)` are one function. A question built once carries its options, levels, labels, meaning texts, threshold, and model. The engine builds it from parts. Experiment 205 found every shim formatting JSON by hand.
3. **"Not sure" is the host's own empty value.** `None`, `null`, `nil`, `NA`, and SQL `NULL`. Rust and C have no such habit and use a three-armed enum, `Answer::Unsure` and `THINKTHEN_UNSURE`. The planning pages give a band a separate `Outcome` type in Python, TypeScript, and Ruby, because an empty value is falsy and `if tt.decide(...)` would read "not sure" as no. The command already made that choice: in a shell `if`, exit 3 takes the `else` branch. A user who sets a band did it on purpose and checks for the empty value. One rule across ten surfaces is worth more than the guard. This is the pick most worth a second look.
4. **The public word is "unsure", and the specification's is "unresolved".** The enum arms and the C constant follow the public word. The builder may prefer the specification's word, and one of the two must win before a header ships.
5. **A band is the host's pair.** `threshold=(0.2, 0.8)` in Python, `[0.2, 0.8]` in TypeScript, `0.2..0.8` in Ruby, `c(0.2, 0.8)` in R, `.band(0.2, 0.8)` in Rust. The `"0.2:0.8"` string stays in question files and on the command line. The R experiment parsed the string form, and the R page shows the pair.
6. **`score` returns a number on every surface,** and the nearest level's name is in `details`. The three database experiments returned three shapes. `2026-09-20-what-the-two-experiments-ask-of-the-engine-and-the-order-to-build-it.md` already says so, and this repeats it for the libraries.
7. **Rust is blocking.** `rust.md` shows `.await`. The engine plan stays blocking, and experiment 211 matched the async bench at 9.666 s. The Rust sample has an `Engine` value built from the environment and plain calls that return `Result`.
8. **Bulk is the same verbs over the host's container.** `filter`, `rank`, and `annotate` take a list, an `Enumerable`, a column, or a table and cross once. `annotate` over a data frame returns the frame with new columns, in Python and in R. A vectorized `decide` over a column is R's habit and SQL's, and R keeps it. Python, TypeScript, and Ruby get `decide_many`, because a string is also a sequence there and guessing is a trap.
9. **SQL names a question file as `'@refund.json'`,** the command's own spelling, so a tuned question reaches a query unchanged. The database ADR decides where the file may be read from.
10. **DuckDB's page still lists `thinkthen_warm` as open.** Experiment 207 answered yes.

## What the slides deliberately leave off

Details, usage counters, cancel tokens, deadlines, and the cache setting. They exist on every surface with one spelling each, and they belong on the reference pages. A surface slide that needs more than fifteen lines of code is showing too much.

## What Ian can overturn

All of it. Pick 3 changes the planning pages for three languages, and pick 8 adds `decide_many` to their public lists.
