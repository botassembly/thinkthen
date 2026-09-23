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

## 2026-09-21 — punch-list item 2: the loops to hand the engine

Four per-record scheduling loops live here, each conversion-ready for the
engine's bulk entry point; none converted (no second bulk implementation
while the engine entry is pending). The full inventory is
`sdlc/records/2026-09-21-punch-list-report.md`:

- `tt_choose` — `thinkthen/R/thinkthen.R` (the `vapply` loop calling
  `tt_choose_one` a row). Conversion-ready -> future `choose_many`.
- `tt_score` — same file (the `vapply` loop). -> future `score_many`.
- `tt_tag` — same file (the `lapply` loop). -> future `tag_many`.
- `tt_recognize_column` — `thinkthen/src/rust/src/lib.rs` (the `.map`
  calling `engine.recognize_opts` a record). -> future `recognize_many`.

## 2026-09-22 — the review's R findings: typed columns, the checked deadline, the guarded interrupt

Source: `sdlc/issues/2026-09-22-surfaces-branch-review-the-full-findings.md` (finding 5, the crash group's deadline, and the phase-1 adoptions). Commands and output as they happened on this machine.

**Finding 5, an answer column's type from the first answer's shape.** The old `tt_annotate` typed each column by `fields[[1]]`: a tag column whose first cell held no label fell to the `choose` branch and every later cell kept only its first label or `NA`. Fixed by asking the question set its kinds once (`tt_annotate_kinds`, new shim export; `set.names()` and `question.kind()` in the set's own name order) and typing each column through `.tt_answer_column`/`.tt_bare` by its kind: decide logical, choose character, score double, tag a list of its labels. The widened failed-marker column keeps its ruled shape (0054) with the bare cells now kind-typed too.

**The baseline, before the fix** (`ENGINE_NULL=1 Rscript /tmp/r-baseline-checks.R`, one check at a time so the first failure does not stop the rest):

```
FAIL  tag column whose first answer holds no label stays a list of labels
FAIL  choose column with an unsure first answer keeps later choices
FAIL  the sentinel means no deadline
FAIL  a NaN deadline is usage
FAIL  a negative deadline is usage
FAIL  an oversized deadline is usage
```

**The same checks after** (`ENGINE_NULL=1 Rscript /tmp/r-after-checks.R`): all eleven pass, including the five that were already right (the decide band with an unsure first answer, the multi-label tag cell, the decimal score column, the character choose column, and zero as the spent deadline).

**What the offline backend cannot vary, recorded honestly.** The null backend's choose weights read the question's options and its tag probabilities read the labels, so neither varies a row: a choose column cannot be made unsure on one row and decided on the next, and a tag column answers the same labels for every row. The first-answer-unsure choose case is therefore proven on the converter itself (`thinkthen:::.tt_answer_column(list(NULL, "refund", "billing"), "choose")` → `c(NA_character_, "refund", "billing")`, R's `NULL` being the engine's unsure), and the tag case end to end with a question whose labels all fall (`labels = ["billing", "other"]` → `character(0)` per cell). `NA` in the `on` column is refused by the shim's own `Vec<String>` conversion ("Must not be NA."), so it is not a path to an empty cell.

**The checked deadline, adopted from the contract (phase 1).** `call` converts a host's deadline through `thinkthen_contract::deadline_from_seconds` before any thread exists: the sentinel `-1` and `NULL` mean no deadline, `0` stays the spent deadline (conformance case 27), and a NaN, a negative other than the sentinel, or a budget past 4294967295 s comes back as the usage kind. The old code clamped with `seconds.max(0.0)` and handed `Duration::from_secs_f64` a panic on an infinite budget — the worker thread died, the main thread saw the channel disconnect, and the caller got `defect` ("the call's worker ended without an answer"). The new checks pin each kind; `Inf` is now `usage` before a thread starts.

**The interrupt guard: the jump never crosses a Rust frame.** The old poll called `R_CheckUserInterrupt` directly, and its longjmp unwound R's C stack across the shim's Rust frames — undefined behavior, with the cancel token registered only because `.tt_cleanup` ran during R's own unwind. The experiment, in `/tmp/tt-r-guard` (a throwaway C file, R 4.3.3):

```
$ R CMD SHLIB guard.c && Rscript -e 'dyn.load("guard.so"); cat("toplevel result:", .Call("tt_guard_probe"), "\n"); cat("script continued\n")'
toplevel result: FALSE
script continued
exit: 0
```

`R_ToplevelExec` catches the interrupt's jump and answers FALSE; the session continues. The re-signal half, also checked:

```
$ Rscript -e 'res <- tryCatch(stop(structure(class = c("interrupt", "condition"), list(message = "", call = NULL))), interrupt = function(e) "caught as interrupt", error = function(e) "caught as error"); cat(res, "\n")'
caught as interrupt
$ Rscript -e 'stop(structure(class = c("interrupt", "condition"), list(message = "", call = NULL)))'; echo "uncaught exit: $?"
Error:
Execution halted
uncaught exit: 1
```

R's own Ctrl-C in a script prints "Execution halted" and exits 1 as well (`Rscript -e 'Sys.sleep(30)'` with a SIGINT: "Execution halted", exit 1); the only difference is the bare `Error:` line an uncaught condition prints. So the shape is: `interrupt_pending()` runs `R_CheckUserInterrupt` inside `R_ToplevelExec`; the pre-check in `call`, the 100 ms tick in the wait, and the R half's checks before and after every `.tt_call` all use it; a pending interrupt cancels the token (no new request starts, the requests already sent finish), clears the slot, and returns the packed `interrupt` marker; `.tt_error` reads the marker and raises R's own interrupt condition (`tryCatch(interrupt = ...)` catches it, proven by `interrupt_fast.sh`). The worker is not waited for: it holds its own token clone and its requests finish on their own, so Ctrl-C stays answered within a tick instead of waiting on a slow send.

**The panic guard.** extendr 0.8.2 already wraps every generated `#[extendr]` body in `std::panic::catch_unwind` and converts a panic to an R error (`extendr-macros-0.8.2/src/wrappers.rs:227-262`), so no panic can unwind across the R C API from this shim; the shim's own slot lock now recovers from poisoning (`PoisonError::into_inner`) instead of panicking every later call, and a panic on the worker thread lands as the `defect` kind through the channel disconnect. The `call` closure now names `&dyn Engine`, so the engine value comes from the contract's connector door (`StandinConnector.connect(&EngineConfig::from_env())`, phase 1) — the one line the real engine's connector replaces.

**The stand-in's test opt-in, adopted (a pre-existing red, not this wave's).** Phase 1 armed the synthesized annotate failure only under `ENGINE_SYNTHETIC_PARTIAL` (`7acb3da`), and the R lane still ran without it: `null suite` died at "the failed field carries the ruled marker" and case 74 would have too. `tests_null.R` and `conformance.R` now set the opt-in themselves before the first call, like the Ruby lane does; the engine reads it when its engine value is built.

**The evidence, full runs.** `./check.sh` with no stub: shim 2 tests, null suite 56 checks, the fast interrupt at 0.886 s with `after: TRUE` (the session answered after the interrupt; the parent now asserts it), fork check, 10 of 10 examples, the conformance slice green (67 ok, 3 diverge, 5 skip — unchanged), 34 recognize checks, 11 ownership checks, the slide sample. With the loopback stub on 8215 (`STUB_DELAY_MS=300`, stopped afterwards and the port checked closed): the wire suite too — `width: 1000 records, wall 9.664 s`, `stub: requests 1000 max_in_flight 32 connections 33` (the recorded baseline), and the wire interrupt at 2.907 s with the counter frozen at 320 through +1 s, +3 s, +5 s.

**Residual windows, stated.** An interrupt is noticed at the next tick, so up to 100 ms after the signal (measured 0.886 s against a 1 s signal). A request already on the wire finishes before the engine stops (the engine's promise); the call returns without waiting for it, and the worker's own token clone keeps the batch stopped. An uncaught re-raised interrupt prints one `Error:` line before `Execution halted` where R's own interrupt prints none. `R_ToplevelExec` catches any jump from the interrupt check; the check is called on the main thread only. The `-1` deadline sentinel is the contract's one spelling for "no deadline"; R's `NULL` remains the natural one.

**One side effect, stated:** with the kinds in hand, `tt_annotate` now names its columns from the set even when the frame has zero rows, where the old code errored on `names(rows[[1]])`; a zero-row frame comes back with the typed empty columns instead of stopping.

## 2026-09-22 — the second review's R items: the percent crash, the encodings, the shared guard, and the fixture door

Source: `sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md` (items 5 and 6, the contract's shared panic guard, and the fixture-door leftover). Commands and output as they happened on this machine (R 4.3.3).

**The percent crash (finding 5).** The stand-in's refusal quotes the caller's text back — `no recorded answer for the text "plain sentence"; the stand-in answers only from the recordings` — and extendr raises every shim `Err` string through `Rf_error`, whose argument is a printf format (`extendr-macros-0.8.2/src/wrappers.rs`, `throw_r_error` in `extendr-api-0.8.2/src/thread_safety.rs:51-56`). A `%s` in the quoted text made R read varargs that do not exist. The repro, before the fix:

```
$ ENGINE_NULL=1 Rscript text_check.R
 *** caught segfault ***
address 0x6a, cause 'memory not mapped'
...
 7: tt_recognize("100% sure %s")
...
An irrecoverable exception occurred. R is aborting now ...
exit: 139
```

Fixed in the one place every error crosses, `carry`: every `%` in the message doubles, and `Rf_error`'s own formatting turns `%%` back into `%`, so the condition message R finally shows is byte-for-byte the engine's text. The shim unit test proves the doubling and fails without it:

```
$ cargo test --quiet --lib        # with the doubling reverted
failures:
    tests::a_percent_in_a_message_doubles_for_r
test result: FAILED. 2 passed; 1 failed
$ cargo test --quiet --lib        # with the fix
test result: ok. 3 passed; 0 failed
```

**Encodings (finding 6).** The root cause was extendr's own string conversion: `charsxp_to_str` builds a `&str` with `std::str::from_utf8_unchecked` over a CHARSXP's raw bytes (`extendr-api-0.8.2/src/wrapper/rstr.rs:26-40`), so a latin1 or invalid string became a `&str` that is not UTF-8 — undefined behavior that aborts the host on some R builds, and here answered silently (latin1 `café` → FALSE, invalid bytes → TRUE). Every `#[extendr]` text argument now arrives as `Robj` and crosses through `text_of`/`texts_of`: the CHARSXP's encoding mark is read with `Rf_getCharCE`; native, UTF-8, and latin1 strings convert through `Rf_translateCharUTF8` and are validated with `std::str::from_utf8`; a bytes-marked string, any other mark, or bytes that are not valid UTF-8 are refused with a usage error naming the fix (`convert it with enc2utf8() or iconv() first`); NA is refused by name. The new `text_check.R` (9 checks) covers the percent text, latin1 conversion (proven through the refusal's quote), the latin1/UTF-8 forms answering alike, clean UTF-8, invalid bytes, and a bytes-marked string. After: `text checks passed: 9`. Note the shim no longer uses extendr's `Vec<String>`/`String` conversion anywhere.

**The contract's shared panic guard (phase 3).** As recorded above, this crate carried no local `catch_unwind` copy to delete: extendr already wraps every generated `#[extendr]` body, and a worker panic previously landed as a channel disconnect with a generic message. The one engine boundary — the worker thread that runs `work` — now runs behind `thinkthen_contract::catch_panic("the R call", ...)`, so a panic beneath the engine comes back as the defect kind carrying the panic's own words (then escaped by `carry`), instead of a lost message. The guard's own behavior is pinned by the contract's tests; this crate's adoption is one call site and has no R-level test because the shim cannot make the engine panic on demand.

**The fixture door, a cross-lane red found by running this lane's gate.** The stand-in's synthesized partial failure became a compile-time door (`synthetic-partial`, `a06b70d`): the R lane's `ENGINE_SYNTHETIC_PARTIAL=1` arming went inert, and the null suite went red before any of this lane's fixes — `Error: failed: the failed field carries the ruled marker`. Adopted the C lane's pattern: the crate forwards the feature (`synthetic-partial = ["thinkthen-standin/synthetic-partial"]`), `tools/config.R` passes `--features synthetic-partial` to its cargo build when `THINKTHEN_R_SYNTHETIC_PARTIAL=1` is set on the `R CMD INSTALL` line, and `check.sh` installs the fixture build for the run and restores the production install at the end (the README's default). `tests_null.R` and `conformance.R` no longer set the dead environment variable. Evidence: `null suite: 56 checks passed` on the fixture build, and the restored production install answers (`production install answers`).

**The row-at-a-time requests, recorded for the build team (item 4).** `tt_choose`, `tt_score`, and `tt_tag` (and `tt_recognize_column`) ask the engine one request a row: they wait on the engine's bulk entry points and the packing encode the wire probe specified (the structured `state` object, one named question a row), which is the build team's work per the second review's leftover list. No host-side batching is possible before that encode exists; converting the loops without it would be the second bulk implementation the punch list forbids. The inventory above (punch-list item 2) stays the pointer.

**The evidence, full runs.** Offline, no stub: shim tests 3, null suite 56 checks, text checks 9, fast interrupt at 0.868 s, fork check, 10 of 10 examples, the conformance slice (86 ok with its recorded skips and divergences, case 74 included), recognize 34 checks, ownership, slide. With the loopback stub on 8215 (`STUB_DELAY_MS=300`, stopped afterwards and the port checked closed): `width: 1000 records, wall 9.67 s`, `stub: requests 1000 max_in_flight 32 connections 33`, and the wire interrupt frozen at 320 requests through +5 s. Exit 0. The crate's `Cargo.lock` gained the `indexmap`/`hashbrown`/`equivalent` entries the current dependency graph resolves (the contract now carries the core's parser); it is committed with the fix.

**The unmaintained crate under extendr, recorded (item 4).** `paste 1.0.15`
rides in through `extendr-api 0.8.2` (`cargo deny --offline check advisories`
in `libraries/r/thinkthen/src/rust` fails with `error[unmaintained]`,
RUSTSEC-2024-0436, "No safe upgrade is available!"), so the swap is
upstream's to make: pin as the lock does now, bump extendr-api when it moves
(the advisory names pastey and with_builtin_macros as alternatives), and
record the explicit `ignore` or block at the merge when deny coverage
extends over the workspaces. Same ticket as the PostgreSQL side.

## 2026-09-23: the third review's R items, each with its probe

Item 3 (NUL byte aborted R, HIGH): every error message the file grammar hands
back escapes interior NUL bytes as the four characters `\u0000` beside the
percent doubling (`carry` in the Rust half), and both `tt_annotate_kinds` and
the annotate row conversion refuse a name that carries a NUL before building
any R string; the one `expect` in the row builder is now a proper usage
error. Probe (`/tmp/rr3/probe_nul.R`, fixture-free, THINKTHEN_NULL): pre-fix
both files - a NUL in a question name and a NUL in an unknown key - printed
`fatal runtime error: failed to initiate panic, error 5, aborting` with a
core dump, exit 134; post-fix both print clean `thinkthen_usage` errors
(`` `questions.na\u0000me` uses lowercase letters, digits, and underscores ``),
the process survives, exit 0.

Item 20a (LC_ALL=C sent mangled bytes): a native-marked string whose raw
bytes are not valid UTF-8 refuses before any translation, because under a
non-UTF-8 locale there is no meaning to translate; latin1- and UTF-8-marked
strings keep their locale-independent conversion. Probe
(`/tmp/rr3/probe_wire_locale.R` against the stub, THINKTHEN_BASE_URL with the
`/v1` path the gate uses): pre-fix `latin1: answered FALSE` and
`native-invalid: answered FALSE` (both sent); post-fix the latin1 string
still answers (its conversion is well-defined) and the invalid native bytes
refuse: `the evidence carries native-marked bytes that are not valid UTF-8
under this locale; convert it with enc2utf8() or iconv() first`.

Item 20b (deadline = NA silently meant none): extendr maps an NA real to
absence, so `deadline = NA` answered with no budget. Every deadline argument
is now an Robj checked at the boundary (`deadline_of`): NULL is absence,
-1 is the one no-deadline number, zero stays spent, NA refuses by name.
Probe (`/tmp/rr3/probe_deadline_na.R`): pre-fix `deadline=NA: answered FALSE`;
post-fix logical NA refuses with `the deadline is seconds as a number, -1 for
no deadline, or NULL`, `NA_real_` refuses with `the deadline is NA: pass -1
for no deadline, seconds as a number, or NULL`, `-1` answers (none), `-2`
still refuses as negative.

Item 20c (the error hook fired on an interrupt): R's own interrupt machinery
runs a user's `options(error = ...)` hook while the guarded check consumes
the pending interrupt. The hook is held aside for the length of every
`.tt_call` and restored on exit, and `.tt_interrupt` delivers a real
interrupt - the process sends itself SIGINT and steps into the unguarded
check - so an uncaught interrupt halts like a genuine Ctrl-C instead of
raising a synthetic condition through `stop()`. Probes (`/tmp/rr3/dbg2.R`
and `probe_hook2.R`): pre-fix `ERROR-HOOK-ON-INTERRUPT fired` twice and the
script continued; post-fix `outer: interrupt, after ok` with the hook silent,
and `interrupt_fast.sh` stays green (`the interrupt landed at 0.905 s, within
a tick of the signal, and the session answered after it`).

Item 18 (annotate overwrote input columns): a question name landing on any
input column - the `on` column included - refuses before any request is paid.
Probe (`/tmp/rr3/probe_overwrite.R`): pre-fix `columns: body | body values
now: FALSE/FALSE` (the input text gone); post-fix `error: annotate cannot add
a question named 'body': the input already has a column by that name; rename
one`.

Full gate for this surface at this commit: `./check.sh` exit 0 (86 ok lines,
wire suite against the stub on 8215, `production install answers`).

## 2026-09-23: the seventh review's R items

R7-11: the shim refused integer deadlines (`-1L`, `5L`) because `deadline_of` read only doubles. It now reads a length-one integer as the same number, and `NA_integer_` refuses as NA. The null suite checks `-1L`, `5L`, `0L`, `-2L`, and `NA_integer_`. At 14f12f5 the suite stopped on `-1L` with "the deadline is seconds as a number, -1 for no deadline, or NULL". It now passes 76 checks.

R7-13: an uncaught Ctrl-C during a call skipped the user's `options(error = ...)` hook, and the script went on with exit 0. A Ctrl-C during plain R code runs the hook once and then goes on. Decision: match plain R. `.tt_call` still holds the hook aside while the guarded checks and the crossing run, because the guarded check itself would fire it (the third review's item 20c). It now restores the hook before it delivers the interrupt, as it already did before it raised an engine error. `interrupt_hook.sh` runs the same child with a plain sleep loop and with a two-million-record column, signals each, and requires the same lines. At 14f12f5 the uncaught case gave `exit 0 / AFTER` against plain R's `exit 0 / HOOK RAN / AFTER`. After the fix both give the hook line once, and the caught case gives `CAUGHT / AFTER` with no hook on both. `interrupt_fast.sh`, `hook_check.R`, and the R1-13 probe stay green. The synthetic fallback in `.tt_interrupt`, used only when the real signal never lands, still holds the hook aside. Ian can overturn this; the other choice keeps the hook silent for every interrupt and documents that it differs from plain R.
