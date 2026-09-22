# NOTES

The R lane's running log, commands and output as they happened, newest
entry last.

## 2026-09-21 — the build

**Tried:** 205's extendr recipe (rextendr 0.5.0 scaffolding, hand-written
shim, `cargo build --release`, `cargo run --bin document` from `src/`,
`R CMD INSTALL -l rlib thinkthen`), rebound to `thinkthen-contract` and
`thinkthen-standin` by path from the worktree.

**Saw:** the shim builds clean once two things are right: the crate needs
both `rlib` and `staticlib` (a staticlib alone cannot feed the `document`
bin), and R objects may only be created on R's main thread, so the worker
thread returns plain Rust data (codes, strings, numbers) and the main
thread builds the R values. The package installs into `rlib/`, which is
folder-local and gitignored; the user library from 205's round is never
touched.

**Means:** the R surface binds the contract the way the rust surface does,
with the same one-dependency move to the real engine later.

**Problems and ways round them:** the package workspace collision needed an
empty `[workspace]` table; `Makevars` needed `LIBDIR = release` because
cargo without `--target` writes to `target/release/`, not a triple
directory; `configure` needed the executable bit.

## 2026-09-21 — the threading shape, and why

**Tried:** R's interrupt discipline. `R_CheckUserInterrupt` must run on
R's main thread and may jump out of the frame with a C longjmp, which
cannot safely cross the engine's scoped threads.

**Saw:** the shape that works: every engine call runs on a fresh plain
worker thread (spawn per call, none held between calls), the main thread
waits on a channel and calls `R_CheckUserInterrupt` every 100 ms, and the
call's cancel token sits in a one-slot registry. When the check jumps, the
`.tt_cleanup` in every call's `on.exit` sets the registered token, and the
engine keeps its promise.

**Means:** the interrupt proof: SIGINT at t+3.0 s of a 1,000-record column
at width 32 against the 300 ms stub returned control in 2.905 s with the
interrupt condition caught, and the stub's counter froze at 288 requests
(9 rounds of 32) through a 2 s settle — the tenth round never left. The
process exits cleanly.

## 2026-09-21 — the null suite and the conformance slice

**Saw:** `ENGINE_NULL=1 Rscript tests_null.R` → 32 checks passed: every
verb, the six kinds as conditions with the retry signal, NA in and NA out,
band and cut, the counters counting sends. `ENGINE_NULL=1 Rscript
conformance.R` → 14 ok, 2 diverge (both already recorded in
`conformance/DIVERGENCES.md`: the stand-in ignores a pre-fired token, and
it carries no disk cache so the second usage call sends again), 2 skip
(the backend kind needs the wire).

**Engine rules learned while testing, recorded so nobody re-derives them:**
an exact tie between a choose question's top two options is unresolved
with or without a cut (the order the user typed is no evidence), so
`tt_choose` over three equally-weighted options answers NA by design; and
the null score reply is hardwired to three levels, so a two-level score
question cannot decode against it.

## 2026-09-21 — the slide sample and the stub finding

**Tried:** `slide.R` (the drawn block, untouched) against the null backend
and against the stub on 8215.

**Saw (null):** the pipeline as drawn keeps 2 of 3 rows, answers urgency
1.7 and 1.05, and the `choose` column answers NA because the three teams
tie exactly — the engine's own rule, and `NA` is the host's empty value.
The comment's second half proves with the same verb carrying a band: the
"maybe" row answers NA and `filter()` drops it.

**Saw (wire):** the `decide` column and the crossing prove on the wire —
1,000 records, 9.663 s, 32 in flight, 33 connections, matching the engine
baseline. The `choose` column cannot run against the stub: the stub
answers one noul probability a request, and a choice question needs a
per-option distribution; the engine answers "the answer to question q1 is
not the shape the question asked for".

**Means, stated as the finding it is:** the stub cannot serve `choose`,
`score`, or `tag` on the wire. The slide is right; the stub is short. The
null backend answers the full shapes (its replies carry the conformance
file's own numbers), so the mutate columns prove there, and the wire
proves the decide family. If the slide owner wants the mutate columns
proven on the wire, the stub needs choice, score, and tag answering — a
change to 205's shared stub, outside this lane's boundary. Reported, not
worked around: no retry or shaping lives in this surface.

## 2026-09-21 — details on a score question

**Saw:** the stand-in's `details` path assumes a noul answer, so `tt_details`
on a score question fails with the backend kind ("the answer carries no
probability"). The nearest level's name rides in the annotate field
instead, and the null suite asserts the current behavior.

**Means:** a stand-in gap for the contract owner to close when `details`
grows the score shape; recorded here and in the report.

**Closed by the settle wave, 2026-09-21:** the contract lane gave `Details`
the `nearest` field and the stand-in fills it on a score question. The R
half now carries it in `tt_details` (`nearest` is the level's name on a
score question, NULL on every other verb), `tt_details` gained the
`deadline` option beside every other `tt_*` verb, and the null suite
asserts the new behavior in place of the gap. The lane's P0 — the ACTIVE
cancel token never reaching the engine — is fixed too: `call` hands the
registered token to every engine call, and the interrupt proof now reads
the stub's counter at +1 s, +3 s, and +4 s (from the still-living child)
and +5 s, all identical, so the stop is proven before process exit rather
than by it.

## 2026-09-21 — the runtime-installer guard

**Tried:** `grep -n "deno\|bun" ~/.zshrc` after the install steps.

**Saw:** one match, line 23, `export PATH="$HOME/.bun/bin:$PATH"` — the
pre-existing line in the committed dotfiles file from before this work,
not this lane's doing, left as it was. This lane installed no runtime and
no tool: rextendr, jsonlite, and dplyr were already present from 205's
round, cargo and R were already present, and the package builds from the
repository's own crates. Nothing this lane ran wrote to any rc file.

## What was not run (unchecked)

- The free-threaded question does not arise for R; the REPL is
  single-threaded and the worker is fresh a call.
- `tt_details` on `choose` and `tag` questions (the same stand-in gap as
  score, expected, not asserted).
- Packaging: no `R CMD build` tarball and no clean-container install in
  this lane; the packaging rehearsal owns that.
- The interrupt proof ran with the default R handler; a user-installed
  signal handler after `library(thinkthen)` keeps firing (R's own
  chaining), not re-proven here.

## 2026-09-21 — recognize and relate, the two new functions

**Tried:** `tt_recognize` and `tt_relate` through the R surface, against
the stand-in and the recordings only: no paid call, no key, no wire.
The acceptance is the R sections of `recognize-surfaces.md`, run as
written by `recognize_check.R`; the conformance slice now carries the 45
recognize and relate cases.

**Saw:** both acceptance calls run as drawn. `tt_recognize(body, kinds)`
comes back as a list column of data frames — `text`, `kind`, `start`,
`end`, `strength` — and `tidyr::unnest()` makes one row per name; the
relations ride in the frame's `relations` attribute. Offsets are R's own
indexing: `substr(text, start, end)` is the name, and the emoji case
(`Le café 😀 Maria Chen arrived.`) carries `start 11, end 20`.
`tt_relate(records, relations, either)` returns a four-column data frame
of edges — `name`, `source`, `target`, `probability` — and the 0.9 bar
keeps the 0.94 edge, mirroring the deck's Python comment.

**The outputs, as printed:**

```
recognize acceptance: 34 checks passed
conformance slice green for the R surface   (64 ok, 3 diverge, 5 skip)
width: 1000 records, wall 9.662 s
stub: requests 1000 max_in_flight 32 connections 33
```

**The refusals, proven:** the question-file form runs too
(`tt_recognize(body, "@file.json")` and `tt_relate(records,
"@links.json")` carry the spec from the file's own section); 256 records refuse with the usage kind naming
255 and the count; an unrecorded text, an unrecorded rule, and a named
end outside the asked kinds each answer the usage condition; NA evidence
gives an empty frame and keeps the column's length; relate refuses NA
records with a host-side sentence. The six kinds keep their R condition
classes from the earlier round.

**Two quirks found and fixed, recorded for the next surface:** R's
`substr(text, start, end)` slices one text per call, so a per-name check
over a column must map over the rows (`mapply(substr, ...)`); a
length-one text with vector start/end returns only the first slice. The
offsets themselves were right on the first run — the quirk was in the
check, not the surface. And a one-element `kinds` vector must keep its
array shape across `jsonlite::toJSON(auto_unbox = TRUE)` (`I()`), or the
contract refuses the spec.

**The incident, recorded:** the C lane's `git add -A` swept this lane's
in-flight files into its commit `692ccbf` (the C lane flagged the same
incident in its own notes). The content is intact on the branch; this
lane's remaining fixes landed after, as their own commit.

## 2026-09-21 — the vocabulary sweep

Every changed file and user-facing string, grepped against the
restricted and banned lists:

```
$ grep -rniE "confidence|certainty|likelihood|cutoff|gray zone|accuracy|calibrated" \
    thinkthen/R thinkthen/src/rust/src thinkthen/DESCRIPTION *.R check.sh README.md | grep -v rlib
(no output; exit 1)
```

Zero hits: the vendor's own word for distribution shape appears nowhere
in this surface, and the number on a recognized name is `strength`, a
number the tool computes — the least of the word probabilities times the
mean of the kind probabilities — said in the package's own words. The
relation's number stays `probability`, a model-reported number passed
through.

## 2026-09-21 — the rulings wave (languages lane)

Ruling 4: `tt_reset_usage` is removed — the R function, the extendr wrapper and module entry, and the generated `extendr-wrappers.R` line. The counter checks take differences instead (`tests_null.R`, `conformance.R`). Grep proof:

```
$ grep -rn reset_usage thinkthen/R thinkthen/src/rust/src tests_null.R conformance.R
(no matches)
$ grep -rn tt_reset_usage rlib/thinkthen/R
(no matches, against the reinstalled package)
```

Ruling 1 aftermath: `.tt_rule_entry` builds `source`/`target` keys; `recognize_check.R`'s question files use the ruled spelling.

Ruling 2: `the_defect_kind_crosses_as_its_own_kind` — a `#[cfg(test)]` unit test in the Rust shim — constructs a contract `Error` with kind `defect` and asserts `carry` packs `defect`, `false`, and the message around the separator; `tests_null.R` then asserts `tt_raise` raises the `thinkthen_defect` condition with the message and retry signal. Both run in `check.sh`: shim test 1 passed, null suite green.

## 2026-09-21 — the shapes from lane B item 2 (languages lane)

The three shapes `e44d492` landed in `contract/` and `standin/`, carried
into this surface, plus the fast-backend interrupt proof, the examples
file, and the conformance slices for cases 73 and 74. Commands and output
as they happened on this machine.

**`Details.requests` and `failed_questions` (0053, 0054).** `tt_details`
now carries both beside the trail it already had:

```
$ ENGINE_NULL=1 Rscript -e ".libPaths(c('rlib', .libPaths())); library(thinkthen); str(tt_details('Q?', 'refund me')$requests)"
 chr "150735edb1a23c1f94e868c1b93992f06f39058d0f270e0f802b7e1c451870ee"
```

**The failed marker, this host's spelling: a named list.** A question
that failed for any record widens its `tt_annotate` column to a list
whose failed cell carries `list(failed = list(kind = ..., cause = ...))`,
never `NA`, while good answers keep their bare shape in the same column:

```r
str(tt_annotate(partial_file, data.frame(body = "order 4471: charged twice, please refund"), on = "body"))
# $ topic :List of 1
#  ..$ :List of 1
#  .. ..$ failed:List of 2
#  .. .. ..$ kind : chr "backend"
#  .. .. ..$ cause: chr "missing_answer"
```

**The record row (go-ahead item 4).** The conformance slice builds the
ruled `list(input = ..., value = ...)` from the surface's own outputs and
checks it where the cases carry rows (`05`, `06`, `19`); no R function
was added. Cases 73 and 74 run green:

```
ok       05-filter-keeps-some-of-five
ok       06-filter-empty-list
ok       19-decide-many-judgments
ok       73-details-carries-requests
ok       74-annotate-preserves-good-answers
conformance slice green for the R surface
```

Case 73 checks the audit's identity fields (the null backend's own rule
cannot reproduce its recorded probability); case 74 checks the marker and
its count.

**A pre-existing red the slice hid, found and fixed.** `check.sh`'s
conformance step was failing on every case: ruling 4 un-exported the
internal `tt_*` wrappers from the NAMESPACE, and `conformance.R` still
called them by bare name, so every case read
`could not find function "tt_question_grammared"` (66 failures at
baseline). The calls are qualified `thinkthen:::` now, and the slice is
green. This is why the R surface's full check had not been runnable since
the rulings wave.

**The fast-backend interrupt.** `interrupt_fast.sh` backgrounds
`interrupt_fast_child.R`, sends SIGINT one second into a two-million-row
null-backend column (about 9 s deaf), and requires the interrupt to land
within 1.5 s:

```
outcome: interrupt
elapsed: 0.919
OK: the interrupt landed at 0.919 s, within a tick of the signal
```

**The examples file.** `examples.json` holds all ten functions keyed by
name, each a runnable call and the answer the null backend gives;
`examples.R` runs each in a fresh Rscript (a private directory for the
question files) and `check.sh` calls it:

```
10 of 10 examples ok
```

**Null suite growth:** 40 checks passed (the requests/failed_questions
checks and the marker's list shape are new). The full `./check.sh` is
green end to end: shim test, null suite, fast interrupt, examples,
conformance (73 and 74 included), the recognize acceptance (34 checks),
and the slide sample; the wire suite skips with no stub on 8215.


**Also recorded here:** `R CMD INSTALL -l rlib thinkthen` failed with
"cannot remove earlier installation" because the packaging rehearsal's
container had installed `rlib/thinkthen` as root (the root-owned
artifacts the adversarial review flagged). The stale tree was moved aside
to `rlib/thinkthen-root-stale` (root-owned; a `sudo rm -rf` or a fresh
clone clears it) and the install then succeeded. The wrappers file
`thinkthen/R/extendr-wrappers.R` regenerated with the new `deadline`
arguments and is committed.
