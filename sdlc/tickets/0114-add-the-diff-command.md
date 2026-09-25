---
flow: build
priority: 114
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0114: Add the diff command

Status: landed on main 2026-09-24 after code review ACCEPT at `0cd64fbd`; see `sdlc/records/0114-build-diff-command.md`. The production re-score past 450 lines is approved by the queue owner, and Ian can overturn it. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it. The first design review is `sdlc/records/0113-0114-design-review.md`. This page is rewritten whole after it.

## Design

`thinkthen diff A [B]` compares two runs over the same records, or two cuts on one run. It lists each answer that changed, says which way it moved, and runs the exact McNemar test. It sends no request and reads no key.

Ian's ruling of 2026-09-24 (`sdlc/issues/closed/2026-09-24-audit-and-diff-move-into-0-1.md`) moves diff into 0.1, after audit. The 2026-09-23 ruling folds "show what changes when the cut moves" into diff across two cuts on one run.

The definition is the `diff` half of `scripts/tools/measure.py` in the private `botassembly/beatles-bench` repository at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`. The golden files decide where this page and the prototype disagree.

diff stands on ticket 0113. It uses 0113's core (`core/measure.rs`, `answer.rs`, `key.rs`), the neutral reader and failure type in `cli/measure.rs`, the fixtures, and the golden helper in `tests/support/measure.rs`. It imports nothing from `cli/audit.rs`. It adds `core/measure/diff.rs` for pairing and the summary, and `cli/diff.rs` for the arguments and the table. One function, `discordant`, decides which pairs the McNemar test counts. A change to that rule touches only that function, its goldens, and its pages.

## Decisions

Each is the agent's decision. Ian can overturn any of them.

1. The command line copies the prototype's diff grammar, `--table` included.
2. Output rows carry no `schema` member, as in 0113.
3. diff pairs answers by answer name and record id only. It does not check that the two runs saw the same record text or question. The `compare` transform checks both under ADR 0021, and `specification/diff.md` points to it. Extra members would break the goldens.
4. The McNemar test with a key counts only wrong-to-right and right-to-wrong pairs, as the prototype's code and the `diff-choose` golden do. The prototype's README says "on right answers", and the prototype's owner filed `sdlc/issues/2026-09-24-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md` in Beatles Bench. The goldens decide until that issue is ruled. "Switching the McNemar rule" gives the steps.
5. `diff` routes before `Environment::read`. Root help lists `diff` right after `audit`.
6. diff refuses a record twice under one answer name in one run. The prototype keeps the last one.
7. The table keeps the prototype's words, `unresolved` and `tied` included, for the reason 0113's decision 7 gives.

## Open item for Ian: the prototype is private

0113's open item covers both tickets. While Beatles Bench stays private, the captures this ticket adds are recorded in the fixture README under "the prototype measurement script", and `specification/diff.md` cites "the prototype measurement script at commit `be7cea2e`". If Ian makes it public, `specification/diff.md`'s citation gains the repository name and URL, and the fixture README changes once under 0113.

## Command line

```text
thinkthen diff A [B] [--key KEY] [--threshold RULE] [--compare-threshold RULE] [--id POINTER] [--table]
```

- `A` and `B` hold saved result lines. At most one input, `KEY` included, may be `-`.
- With `B`, two runs are compared. Without `B`, A is compared with itself, and `--compare-threshold` is required.
- A reads under `--threshold`. B reads under `--compare-threshold`, or under `--threshold` when that is absent. Without a rule an answer stays as printed, and the output says `"as run"`. Both take the settled threshold grammar.
- `--key KEY` is read by 0113's key reader. A bad `part` value is refused. Valid parts play no role in diff.
- `--id POINTER` works as in 0113, default `/id`.
- `--table` prints the results for a person instead of JSONL.

The golden command lines, run from `tests/fixtures/measure/`:

| Output | Command line | Test |
| --- | --- | --- |
| `golden/diff-decide-cuts.jsonl` | `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4` | `diff_goldens::decide_cuts` |
| `golden/diff-decide-wordings.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl --key small/decide-key.jsonl` | `diff_goldens::decide_wordings` |
| `golden/diff-decide-nokey.jsonl` | `diff small/decide.jsonl small/decide-b.jsonl` | `diff_goldens::decide_nokey` |
| `golden/diff-choose.jsonl` | `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl` | `diff_goldens::choose` |
| `golden/diff-249-cuts.jsonl` | `diff 249/control.jsonl --key 249/key.jsonl --compare-threshold 0.42` | `diff_goldens::beatles_cuts` |
| `golden/diff-249-soft.jsonl` | `diff 249/control.jsonl 249/soft.jsonl --key 249/key.jsonl` | `diff_goldens::beatles_soft` |
| `golden/extra/diff-annotate.jsonl` | `diff small/annotate.jsonl --key small/annotate-key.jsonl --compare-threshold 0.75` | `diff_goldens::annotate` |
| `golden/extra/diff-249-cuts-nokey.jsonl` | `diff 249/control.jsonl --compare-threshold 0.42` | `diff_goldens::beatles_cuts_nokey` |

Root help row: `Show which saved answers changed between two runs or two cuts.` Short and long help open with it. Long help also carries `diff sends no request and reads no key.`, `Two cuts on one run cost nothing.`, and `The probabilities are already saved.` It says "not sure" for an answer inside a band.

## The math

Every definition is the prototype's. The answer under a rule, the key's value, and the outcome come from 0113. The confidence is `p` for `decide` and the top probability for `choose`, or null without probabilities.

**Pairing.** Read A, and B when given. Without B, B is A. Leave out failed answers. The pair key is `(answer name, record id)`. `only_a` counts the distinct pair keys of A missing from B, and `only_b` the reverse.

**Each pair.** Walk A's answers in file order and skip each one without a partner. For each pair `(x, y)`:

- `records` adds one.
- `from` is x's answer under rule A, and `to` is y's answer under rule B.
- With a key, the key's value comes from x. When it is not null, `labeled` adds one, `right_a` adds one when `from` is right, and `right_b` adds one when `to` is right. `discordant(oa, ob)` then adds to `right_to_other` or `other_to_right`.
- When `from` equals `to`, nothing more happens.
- Otherwise `changed` adds one, the move `(from, to)` adds one, and the pair prints a row. Without a key, a move from yes to no adds to `yes_no`, and from no to yes to `no_yes`.

**The effect** of a changed pair with a labeled key:

| `oa` | `ob` | effect |
| --- | --- | --- |
| wrong | right | `gained` |
| right | wrong | `lost` |
| wrong | wrong | `changed` |
| tied or unresolved | right or wrong | `resolved` |
| right or wrong | tied or unresolved | `withdrawn` |
| any other pair | | `changed` |

Without a key, or for an unlabeled record, the effect is null. `gained` and `lost` count those effects.

**`discordant(oa, ob)`** lives in `core/measure/diff.rs` and is the one place the McNemar rule lives. Today it returns `other_to_right` for wrong to right and `right_to_other` for right to wrong. It returns nothing for every other pair. So today `other_to_right` equals `gained` and `right_to_other` equals `lost`.

**A row** prints `{id, name, from, to, probability, key, effect}`. `name` is the `annotate` answer name or null. `probability` is `[confidence of x, confidence of y]`. `key` is `"yes"` or `"no"` for `decide`, the option for `choose`, or null.

**The summary** is the last line, `{"summary": S}`. `S` prints `records`, `changed`, `only_a`, `only_b`, `moves`, `labeled`, `right_a`, `right_b`, `gained`, `lost`, `mcnemar_on`, `mcnemar_p`, `compare`, `a`, `b`.

- `moves` lists `{from, to, count}` by count descending, then `from`, then `to`, in code point order.
- Without a key, `labeled`, `right_a`, `right_b`, `gained`, and `lost` are null.
- `compare` is `"runs"` with B and `"cuts"` without.
- `a` and `b` print `"as run"`, the cut as a number, or the band as typed.

**McNemar.** With a key: `mcnemar(other_to_right, right_to_other)`, and `mcnemar_on` is `"right answers"`. Without a key, when every answer of A is `decide`, failed ones included: `mcnemar(yes_no, no_yes)`, and `mcnemar_on` is `"yes answers"`. Otherwise both are null. For counts `a` and `b`:

```text
n = a + b; p = 1.0 when n = 0
tail = Σ for i = 0..min(a, b) of C(n, i) / 2ⁿ
p = min(1, 2 · tail)
```

`mcnemar` in `core/measure.rs` sums the terms in log space. It starts at `−n·ln 2` and adds `ln((n − i)/(i + 1))` at each step. That holds for any `n` without a big-integer dependency. A unit test compares it with exact `u128` sums for every split up to `n = 120` within a relative `1e-12`. The review measured a worst error of 3.0e-14. The test also pins `(0,0)` 1.0, `(1,0)` 1.0, `(2,0)` 0.5, `(5,0)` 0.0625, `(39,28)` 0.221549, and `(36,34)` 0.904975.

**Printing** follows 0113: six-place rounding, integers kept, and `python_float_text` for rules in the table.

## Switching the McNemar rule

If the prototype's owner rules for every discordant pair, the switch runs in this order:

1. Upstream rewrites its goldens. The pinned commit and every SHA-256 in the fixture README move to the new commit.
2. Recapture `golden/extra/diff-annotate.jsonl` and `golden/table/diff-choose.txt`. In `diff-annotate`, a3 moves from right to not sure and becomes discordant. In `diff-choose`, p becomes 0.5.
3. `discordant` also counts tied or not sure to right, and right to tied or not sure. Nothing else in the code changes. `gained` and `lost` stay effects.
4. `specification/diff.md`, the choose hand test, and the `discordant` test change in the same commit. The `discordant` test then expects `resolved` and `withdrawn` pairs to count.

## Output and failures

diff builds every line before printing. On success it prints the changed rows in A's order, then the summary, and exits 0.

`--table` prints the prototype's `diff_table` text. A changed row prints `ID[/NAME]  FROM -> TO  p PA -> PB`, plus `  key KEY: EFFECT` when the key has a value. Probabilities print with two decimals, and `-` stands for null. The count line follows:

- It starts `A -> B`, or `A -> B (at RULE_A and RULE_B)` when either side has a rule, or `RULE_A -> RULE_B` for two cuts.
- It goes on `: C of N changed`, then `; FROM -> TO COUNT` for each move.
- With a key it adds `; gained G, lost L (RA -> RB right of LABELED)`.
- With a test it adds `; McNemar p P on ON`, `P` with three decimals.
- With unpaired answers it adds `; only in A X, only in B Y`.

Rounding in the table is half to even on the rounded value, as in 0113.

On failure diff prints nothing on standard output and one line on standard error. It renders 0113's failure rows through `cli/measure.rs` with the prefix `thinkthen: diff:` and the roles `first run`, `second run`, and `key`. It adds these rows, each pinned by a test:

| Case | Exit | Standard error |
| --- | ---: | --- |
| No B and no `--compare-threshold` | 2 | `thinkthen: diff: diff needs a second run or --compare-threshold` |
| One answer name and record twice in one run | 2 | `thinkthen: diff: ROLE line N repeats a record for one answer` |
| More than one input is `-` | 2 | `thinkthen: diff: only one input may be standard input` |

## Where the code lives

- `core/measure.rs`: add `mcnemar`.
- `core/measure/diff.rs` (new): pairing, `discordant`, the effect, the moves, the summary, and the row structs. It is pure.
- `cli/diff.rs` (new): Clap arguments, the table, and the call into the core.
- `cli/measure.rs`: gain the diff roles and the three failure rows, if 0113 did not already carry them.

Extend 0113's generalized policy check to `cli/diff.rs`, with the same banned words and the same early-return order check for `Diff` in `cli/mod.rs`. The self-test plants one forbidden reference and a late early return, and each must be refused.

## Fixtures and provenance

0113 copies every fixture, the six diff goldens included, and records their provenance. This ticket captures four outputs from the prototype at `be7cea2e`, read with `git show be7cea2:scripts/tools/measure.py` into a scratch file, under Python 3.12.3. It adds them to the fixture README with their commands and SHA-256 values.

- `golden/extra/diff-annotate.jsonl`, from the table above. No upstream golden diffs `annotate` answers. It pins pairing by answer name and two `withdrawn` effects.
- `golden/extra/diff-249-cuts-nokey.jsonl`, from the table above. It shows 67 moves from no to yes and `mcnemar_p` 0.0 on yes answers.
- `golden/table/diff-decide-cuts.txt`, `diff-decide-nokey.txt`, and `diff-choose.txt`: the lines `diff-decide-cuts`, `diff-decide-nokey`, and `diff-choose` with `--table`.

## Acceptance

**Goldens.** Run each line of the table above from `tests/fixtures/measure/` and compare with 0113's helper and tolerance. Each run exits 0 with empty standard error. The table captures match byte for byte.

**Hand-checked values.** Port the `Diff` class and the diff half of `LeaningNo` from `tests/test_measure.py` as core tests:

- Two cuts on one run: one row, `r3` from no to yes, `gained`; 6 records, 1 changed, p 1.0, `cuts`, `as run`, 0.4.
- Two wordings: `r2` and `r3` gained, `r2` probabilities `[0.7, 0.3]`, `right_a` 4, `right_b` 6, `only_b` 1, p 0.5, moves in the pinned order.
- No key: `yes answers`, p 1.0, `gained` null.
- Choose: `c2` tied to green `resolved`, `c5` `gained`, p 1.0.
- The Beatles control at 0.42 on the held-out key: `right_a` 87, `right_b` 89, every move from no to yes.
- The control against the softer wording on the held-out key: `right_a` 87, `right_b` 84.
- `discordant` returns nothing for `resolved` and `withdrawn` pairs.

**No request, no key.** As in 0113: a counting loopback listener in `THINKTHEN_BASE_URL`, a canary `THINKTHEN_API_KEY`, and unreadable configuration and cache locations. Run every golden line, table line, and failure case. Zero connections, no canary anywhere, and no file created or changed.

**Failures.** Each failure row has a test for its exit code, exact line, and empty standard output. 0113's secrecy test gains every diff failure path.

**Help and vocabulary.** Edit the one root inventory assertion to insert `diff` after `audit`. Pin the root row, both introductions, and the three long-help sentences. Add `diff` to the `scan_help` loop in `sdlc/scripts/demos`. The check finds zero unsanctioned hits.

**Red-green.** Write each test first and watch it fail for its stated reason. Then plant each bug below, one at a time, and record which named test turns red. A bug no test catches gets a new test before landing.

1. McNemar one-sided, without the doubling.
2. McNemar summed to `min(a, b) − 1`.
3. The `min(1, ·)` clamp dropped.
4. `gained` and `lost` swapped.
5. `from` and `to` swapped in the moves.
6. `resolved` counted as `gained`.
7. `only_a` and `only_b` swapped.
8. Pairing by record id alone.
9. The no-key test run on the discordant counts. `extra/diff-249-cuts-nokey` catches it: p 0.0 becomes 1.0.
10. B's failed answers paired.
11. B read under `--threshold` when `--compare-threshold` is given.

**Gates.** Focused tests, then `sdlc/scripts/install`, `lint`, `test`, and `spec` in sequence with the key and base address unset, and `git diff --check`. No live or paid call.

## Specification pages

Add `specification/diff.md`, Status **Settled** on landing. It states the command line, pairing, the effect table, the summary, the McNemar rule and its switch, the table form, and the failures. It cites the prototype as the open item allows and points to the `compare` transform. Add its index row. Add `spec/diff.md`. It pins the two-cut table's count line, `as run -> 0.4: 1 of 6 changed; no -> yes 1; gained 1, lost 0 (4 -> 5 right of 6); McNemar p 1.000 on right answers`, and the missing-second-run refusal with its exit code.

## Departures

No golden reaches these. Each is the agent's decision, and Ian can overturn it. 0113's departures on failures, bad numbers, JSON, the pointer, rule text, key values, and state names also hold here. diff adds decision 6, the refused repeat.

## Builder note: keep the formats open

`sdlc/issues/closed/2026-09-24-audit-and-diff-needs-for-graded-agent-runs.md` lists four needs for grading agent runs after 0.1. None enters 0.1, and nothing here changes a golden or an accepted input. Keep three things open.

- diff reads the key and the runs through 0113's readers, which ignore members they do not use. A test pins one extra member in a run line.
- The repeat check of decision 6 keys on (answer name, record id) in one function, so a sample key can join it later.
- `specification/diff.md` says that readers of diff output should ignore members they do not know, because later versions may add them.

## Budgets and the ratchet

- Production Rust: two new files and at most three existing files touched (`core/measure.rs`, `cli/mod.rs`, `cli/args/command.rs`), plus `cli/measure.rs` if its roles need a line. At most 450 nonblank lines.
- Rust tests: at most 500 nonblank lines in at most two new test files, and the edited inventory assertion.
- Scripts: `policy.py`, its self-test, and the `demos` loop, at most 25 nonblank lines.
- Fixtures: the five captures and the README update.
- Every Rust file stays under 500 nonblank lines.
- Dependencies: none. The log-space sum replaces a big-integer crate. A new one stops the build for a second review.

`sdlc/ratchet.json` rises by the measured Rust increase in the commit that needs it. That message says what grew, why it earns its lines, and where the builder looked for duplication: at least 0113's reader, answer, key, and table code. The ticket raises the ceiling and widens the public surface, so a second agent reviews it and names what it checked. Stop and re-score past any budget, on any dependency, or on any golden change.

## Scope and exclusions

Excluded: checking record text or question identity across runs, a probability-shift member, other verbs, reading a recording folder directly, library or database surfaces, a `schema` member, changing a golden, editing Beatles Bench, live calls, and release artifacts.

## Dependencies and order

Depends on the ten functions done (0086 landed) and on 0113 landed, because it shares the core, `cli/measure.rs`, the fixtures, and the helper. The queue in `sdlc/planning/one-line-plan-2026-09-24.md` places diff after audit and before the release build. The two do not build in parallel.

## Complexity

Contract 2; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 6. Final level: 2.

## Evidence

Builder note, 2026-09-24. Workspace decision `2026-09-24-experiments-reduce-risk.md` asks every product ticket to name these five parts. This note changes no design.

- Starts from: The `diff` half of the prototype `scripts/tools/measure.py` in the private Beatles Bench repository at `be7cea2e`, its README, and the six diff goldens in `tests/fixtures/audit/golden/`. Those files are unchanged through the 2026-09-24 head `b0bf4ae`. The Beatles Bench issue `sdlc/issues/2026-09-24-diff-mcnemar-leaves-out-pairs-that-become-right-from-not-sure.md` asks for a wider McNemar rule. `experiments/249-jev-answer-audit/README.md` compares the control and softer wordings behind the `diff-249` goldens. `experiments/212-thinkthen-repeat/RESULTS.md` and `experiments/235-beatles-judgment/README.md` show identical requests moving probabilities and flipping answers. `experiments/235-beatles-judgment/README.md` and `experiments/243-beatles-workarounds/README.md` replay a stricter cut from a recording at no cost, the `--compare-threshold` case. `repos/jev-experiments`: none found.
- Keeps: diff keeps the prototype's grammar, pairing by answer name and record id, the effect table, the summary members, and the table form. McNemar counts only wrong-to-right and right-to-wrong pairs, as the `diff-choose` golden does.
- Changes: diff refuses a record repeated under one answer name in a run, where the prototype keeps the last one. It inherits 0113's departures on failures, numbers, JSON, pointers, and key values.
- Proof: The six goldens, the extra captures, and the three table captures match. Hand tests come from the prototype's `Diff` tests. Exact integer sums check the log-space McNemar up to n = 120. Each planted bug in "Acceptance" turns a test red, and 0113's no-request and secrecy tests cover every diff path.
- Defers: The wider McNemar rule waits for Ian's ruling on the Beatles Bench issue, and "Switching the McNemar rule" gives the steps. Checking record text or question identity across runs stays with the `compare` transform. A probability-shift member and the agent-run needs wait until after 0.1.

Amended 2026-09-24: the ADR 0017 amendment of that date on main renames the width setting to the throttle. This ticket names no width. Any public name or text it writes uses the throttle.
