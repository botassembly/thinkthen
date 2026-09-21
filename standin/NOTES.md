# standin NOTES

## 2026-09-21 — R1: the recognize and relate replay

The stand-in answers both functions from the recordings, never by inventing
an answer.

- `src/replay.rs` embeds `data/recognize-replay.json` (generated from
  `experiments/225-recognize-harvest-package` by
  `../conformance/tools/build_recognize_cases.py`) and implements the
  matching rules: a text the recordings do not hold is a usage error naming
  the text; a rule the recordings do not hold for that text is a usage error
  naming the rule and the covered set; a rule's named end must be one of the
  asked kinds; relate refuses a kind field and named rule ends because the
  recordings carry no kinds; the record limit refuses past 255.
- The recordings carry the number on a name under Jev's own field name; the
  replay maps it to the interim ruled field `number`.
- Two relate arms (R03 per-subject, R04 pairs) share one text; the replay
  prefers the pairs form, the ruled method, and the per-subject case is
  pinned in the conformance file and skipped in the replay test, recorded in
  `../conformance/DIVERGENCES.md`.
- The trait methods `recognize_opts` and `relate_opts` call
  `Error::guard(options)` first and then the replay; `relate_opts` also runs
  `guard_relate_records` so the limit holds even when a caller enters
  directly.

Commands and output:

```
$ cargo test --test recognize_replay
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test
lib: 4 passed; wire: 3 passed; recognize_replay: 7 passed
```

The seven: every conformance case replays exactly (41 recognize cases and
R01/R02/R04; R03 skips as marked); the door JSON carries `source`, `target`,
`probability`, and `number` and never `from`/`to`/`confidence`; the deck's
0.9-bar relate call returns the two sound edges; the 255 refusal; missing
texts and unrecorded rules name themselves; the any-kind end replays on the
C36 case; relate refuses kinds it cannot honour.

`cargo check --all-targets` is clean with no warnings.

The check script's R-surface slice now fails on the 45 new cases because
that runner parses every case's question before it switches on the verb;
the guard is the R surface lane's one-line fix, filed in
`../conformance/DIVERGENCES.md`.
