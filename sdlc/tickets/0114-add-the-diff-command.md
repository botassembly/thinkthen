---
flow: build
priority: 114
opens: crates/thinkthen/src/core crates/thinkthen/src/cli crates/thinkthen/tests specification spec sdlc/scripts sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0114: Add the diff command

Status: design draft; review pending. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Build `thinkthen diff`. It compares two runs over the same records, or two cuts on one run. It lists each answer that changed, says which way it moved, and runs the exact McNemar test. It sends no request and reads no key.

Ian's ruling of 2026-09-24 (`sdlc/issues/2026-09-24-audit-and-diff-move-into-0-1.md`) moves diff into 0.1, after audit. The 2026-09-23 ruling in `sdlc/issues/2026-09-23-a-measurement-tier-for-thinkthen-audit-and-diff.md` folds "show what changes when the cut moves" into diff across two cuts on one run.

The definition is the `diff` half of `scripts/tools/measure.py` in the public `botassembly/beatles-bench` repository at commit `be7cea2e4aa41097e7f629e35b62dadedeaca544`, with its tests and README at that commit. Where this ticket and the prototype disagree, the golden files decide.

This ticket builds on ticket 0113. It reuses the result reader, the key reader, the graded answer, the rule, the rounding, the fixtures, the golden comparison helper, the failure mapping, and the policy entry that 0113 lands.

## Decisions

Each of these is the agent's decision, and Ian can overturn any of them.

1. The command line copies the prototype's diff grammar, including `--table`.
2. Output rows carry no `schema` member, for the reason 0113 gives.
3. diff pairs answers by answer name and record id only. It does not check that the two runs saw the same record text or the same question. The `compare` transform does check both under ADR 0021, and `specification/diff.md` points to `thinkthen transform show compare` for that check. Adding those members would break the goldens. A later change starts in the prototype and its goldens.
4. `McNemar on right answers` counts the gained and lost pairs only, as the prototype's code and the `diff-choose` golden do. See "The McNemar count" below.
5. `diff` routes before `Environment::read`, as `audit` does. Root help lists `diff` right after `audit`.
6. A record that appears twice under one answer name in one run is a usage error. The prototype keeps the last one silently.

## Command line

```text
thinkthen diff A [B] [--key KEY] [--threshold RULE] [--compare-threshold RULE] [--id POINTER] [--table]
```

- `A` and `B` are files of saved result lines, read as ticket 0113 reads RESULTS. `-` reads standard input for at most one input.
- With `B`, the two runs are compared. Without `B`, A is compared with itself under a second rule, and `--compare-threshold` is required.
- A reads under `--threshold`. B reads under `--compare-threshold`, or under `--threshold` when that is absent. Without a rule, an answer stays as it was printed, and the output says `"as run"`. Both options take the settled threshold grammar.
- `--key KEY` is an answer key in 0113's format. Its `part` members are ignored.
- `--id POINTER` works as in 0113, default `/id`.
- `--table` prints the same results for a person instead of JSONL.

The golden command lines, run from the fixture folder, are exactly:

```text
thinkthen diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4
thinkthen diff small/decide.jsonl small/decide-b.jsonl --key small/decide-key.jsonl
thinkthen diff small/decide.jsonl small/decide-b.jsonl
thinkthen diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl
thinkthen diff 249/control.jsonl --key 249/key.jsonl --compare-threshold 0.42
thinkthen diff 249/control.jsonl 249/soft.jsonl --key 249/key.jsonl
```

Root help row: `Show which saved answers changed between two runs or two cuts.` Short and long help open with the same sentence. Long help also carries these sentences: `diff sends no request and reads no key.` and `Two cuts on one run cost nothing, because the probabilities are already saved.` Help says "not sure" for an answer inside a band. It never prints `unresolved`, and it passes the vocabulary check of ticket 0082.

## The math

Every definition below is the prototype's. `said`, the key's value, the outcome, and the confidence come from 0113's `core/measure/answer.rs`. The confidence is `p` for `decide` and the top probability for `choose`, or null without probabilities.

**Pairing.** Read the answers of A, and of B when given. Without B, B is A. Leave out failed answers. An answer's pair key is `(answer name, record id)`. `only_a` counts the distinct pair keys of A missing from B, and `only_b` counts those of B missing from A.

**Each pair.** Walk A's answers in file order and skip each one without a partner in B. For each pair `(x, y)`:

- `records` adds one.
- `from = said(x, rule A)` and `to = said(y, rule B)`.
- With a key, the key's value comes from `x`. When it is not null: `labeled` adds one, `right_a` adds one when `from` is right, and `right_b` adds one when `to` is right.
- When `from` equals `to`, nothing more happens.
- Otherwise `changed` adds one, the move `(from, to)` adds one, and the pair prints a row.

**The effect** of a changed pair with a labeled key, from the outcomes `oa` and `ob`:

| `oa` | `ob` | effect |
| --- | --- | --- |
| wrong | right | `gained` |
| right | wrong | `lost` |
| wrong | wrong | `changed` |
| tied or unresolved | right or wrong | `resolved` |
| right or wrong | tied or unresolved | `withdrawn` |
| any other pair | | `changed` |

Without a key, or with an unlabeled record, the effect is null. `gained` and `lost` count the effects of those names.

**A row** prints `{id, name, from, to, probability, key, effect}` in that order. `name` is the `annotate` answer name or null. `probability` is `[confidence of x, confidence of y]`. `key` is the key's value (`"yes"` or `"no"` for `decide`, the option for `choose`) or null.

**The summary** is the last line, `{"summary": S}`. `S` prints in this order: `records`, `changed`, `only_a`, `only_b`, `moves`, `labeled`, `right_a`, `right_b`, `gained`, `lost`, `mcnemar_on`, `mcnemar_p`, `compare`, `a`, `b`.

- `moves` lists `{from, to, count}` sorted by count descending, then by `from`, then by `to`, in code point order.
- Without a key, `labeled`, `right_a`, `right_b`, `gained`, and `lost` are null.
- `compare` is `"runs"` with B and `"cuts"` without it.
- `a` and `b` are `"as run"`, a cut number, or the band text as typed.

**The McNemar test** is exact and two-sided. With a key, it runs on `(gained, lost)`, and `mcnemar_on` is `"right answers"`. Without a key, when every answer of A, failed ones included, is `decide`, it runs on `(yes_no, no_yes)`: the changed pairs that moved from yes to no and from no to yes. Then `mcnemar_on` is `"yes answers"`. Otherwise both members are null. For counts `a` and `b`:

```text
n = a + b
p = 1.0 when n = 0
tail = Σ for i = 0..min(a, b) of C(n, i) / 2ⁿ
p = min(1, 2 · tail)
```

The prototype sums exact integers. The port adds `mcnemar` to `core/measure.rs`. It computes each term in log space, starting from `ln C(n, 0) − n·ln 2 = −n·ln 2` and adding `ln((n − i)/(i + 1))` for each step. It sums the exponentials. This holds for any `n` without overflow and without a big-integer dependency. A unit test compares it with exact integer sums, done in `u128`, for every `n` up to 120 and every split. It agrees to a relative `1e-12`. The test also pins `mcnemar(1, 0) = 1.0`, `mcnemar(2, 0) = 0.5`, `mcnemar(0, 0) = 1.0`, and `mcnemar(5, 0) = 0.0625`.

**The McNemar count.** A textbook McNemar on right answers counts every discordant pair: right in A and not in B, and the reverse. The prototype counts only `gained` and `lost`. It leaves out a pair that became right from tied or not sure (`resolved`) and a right one that became tied or not sure (`withdrawn`). The `diff-choose` golden shows it: `right_a` 2, `right_b` 4, `gained` 1, `lost` 0, and `mcnemar_p` 1.0. The discordant count would give 0.5. The port follows the golden. `specification/diff.md` states the rule in these words. Any change starts in the prototype and its goldens.

**Rounding** follows 0113: every float rounds to six places, and integers stay integers.

## Output and failures

diff builds every line before it prints. On success it writes the changed rows in A's order, then the summary, one JSON object per line, and exits 0. Two runs with nothing in common print only the summary.

`--table` prints the prototype's `diff_table` text. Each changed row prints as `ID[/NAME]  FROM -> TO  p PA -> PB`, with `  key KEY: EFFECT` appended when the key has a value. Probabilities print with two decimals, and `-` stands for null. The count line then follows:

- It starts with `A -> B`, or `A -> B (at RULE_A and RULE_B)` when either side has a rule, or `RULE_A -> RULE_B` for two cuts.
- It goes on with `: C of N changed`, then `; FROM -> TO COUNT` for each move.
- With a key it adds `; gained G, lost L (RA -> RB right of LABELED)`.
- With a test it adds `; McNemar p P on ON`, where `P` has three decimals.
- When a run holds unpaired answers it adds `; only in A X, only in B Y`.

Rounding in the table is half to even on the exact binary value, as in 0113.

On any failure diff prints nothing on standard output and one line on standard error. It reuses 0113's failure table with the prefix `thinkthen: diff:`, and it names inputs `first run`, `second run`, and `key` in place of `results` and `key`. It adds these rows:

| Case | Exit | Standard error |
| --- | ---: | --- |
| No B and no `--compare-threshold` | 2 | `thinkthen: diff: diff needs a second run or --compare-threshold` |
| The same answer name and record twice in one run | 2 | `thinkthen: diff: first run line N repeats a record for one answer` (or `second run line N`) |
| More than one input is `-` | 2 | `thinkthen: diff: only one input may be standard input` |

Each sentence is pinned exactly by a test.

## Where the code lives

- `crates/thinkthen/src/core/measure.rs`: add `mcnemar` beside 0113's statistics.
- `crates/thinkthen/src/core/measure/diff.rs` (new): pairing, the effect, the moves, the summary, and the output structs in the member order above. It is pure.
- `crates/thinkthen/src/cli/diff.rs` (new): the Clap arguments, reading A, B, and KEY through 0113's reader, calling the core, the table, and the failure mapping.

Extend 0113's `policy.py` entry to hold `cli/diff.rs` to the same bans as `cli/audit.rs`. Plant one forbidden reference in the policy self-test and prove it is refused. Move a helper only when both commands need it. Duplicate none.

## Fixtures and provenance

Ticket 0113 copies all fixtures, the six diff goldens included, and records their provenance in `crates/thinkthen/tests/fixtures/measure/README.md`. This ticket turns on the six diff goldens.

It also captures four outputs from the prototype at the same commit, standard output only, and adds them to that README with their commands and SHA-256 values:

- `golden/extra/diff-annotate.jsonl` from `diff small/annotate.jsonl --key small/annotate-key.jsonl --compare-threshold 0.75`. No upstream golden diffs `annotate` answers. This one pins pairing by answer name and two `withdrawn` effects.
- `golden/table/diff-decide-cuts.txt` from `diff small/decide.jsonl --key small/decide-key.jsonl --compare-threshold 0.4 --table`.
- `golden/table/diff-decide-nokey.txt` from `diff small/decide.jsonl small/decide-b.jsonl --table`.
- `golden/table/diff-choose.txt` from `diff small/choose.jsonl small/choose-b.jsonl --key small/choose-key.jsonl --table`.

Running the prototype reads that repository and writes nothing to it.

## Acceptance

**Goldens.** For each of the six diff command lines and the extra annotate line, run the built binary from `tests/fixtures/measure/` and compare parsed JSON with 0113's helper and tolerance. The lists match in length and order. Objects match in member names and order. Strings, booleans, nulls, and integers are equal. Floats are floats and differ by at most `1e-6 + 1e-12`. Each run exits 0 with empty standard error. The three table captures match byte for byte.

**Hand-checked values.** Port the `Diff` class and the diff half of `LeaningNo` in `tests/test_measure.py` as Rust tests on the core:

- two cuts on one run: one row, `r3` from no to yes, `gained`; summary 6 records, 1 changed, p 1.0, `compare` `cuts`, `a` `as run`, `b` 0.4;
- two wordings: `r2` and `r3` both gained, probabilities `[0.7, 0.3]` on `r2`, `right_a` 4, `right_b` 6, `only_b` 1, p 0.5, and moves in the pinned order;
- no key: `mcnemar_on` `yes answers`, p 1.0, `gained` null;
- choose: `c2` from tied to green `resolved`, `c5` `gained`, p 1.0;
- the Beatles control against the cut 0.42 on the held-out key: `right_a` 87, `right_b` 89, and every move from no to yes;
- the control against the softer wording on the held-out key: `right_a` 87, `right_b` 84.

**No request, no key.** Bind a counting loopback listener named in `THINKTHEN_BASE_URL`. Set `THINKTHEN_API_KEY` to a canary. Make the normal configuration and cache locations unreadable. Run every golden line, every table line, and every failure case. The listener accepts zero connections. The canary appears in no standard output, standard error, Debug line, or temporary file. No cache, usage counter, lock, or folder is created or changed.

**Failures.** Each failure row has a test that pins its exit code, its exact standard-error line, and empty standard output. The secrecy test of 0113 gains every diff failure path.

**Help.** Edit the one root inventory assertion to insert `diff` after `audit`. Pin the root row, the short and long introductions, and the two long-help sentences. The vocabulary check passes with zero unsanctioned hits.

**Red-green.** Write each test first and watch it fail for its stated reason. Then plant each bug below in the finished code, one at a time. Record in the build record which named test turns red. A planted bug that no test catches gets a new test before landing.

1. McNemar one-sided, without the doubling.
2. McNemar summed to `min(a, b) − 1`.
3. The `min(1, ·)` clamp dropped.
4. `gained` and `lost` swapped.
5. `from` and `to` swapped in the moves.
6. `resolved` counted as `gained`.
7. `only_a` and `only_b` swapped.
8. Pairing by record id alone, ignoring the answer name.
9. The no-key test run on `gained` and `lost`.
10. B's failed answers paired.
11. B read under `--threshold` when `--compare-threshold` is given.

**Gates.** Run focused tests, the policy self-test, the fixture checksum test, formatting, Clippy, the exact ratchet, and `git diff --check`. Then run `sdlc/scripts/install`, `lint`, `test`, and `spec` in sequence with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset. No live or paid call runs.

## Specification pages

Add `specification/diff.md` with Status **Settled** once this ticket lands. It states the command line, pairing, the effect table, the summary, the McNemar rule with the count above, the table form, and the failure rows. It cites the prototype commit and points to the `compare` transform for a comparison that checks record text and question identity. Add its row to `specification/README.md`. Add an executable `spec/diff.md` that runs the two-cut table on `small/decide.jsonl` and pins its count line, `as run -> 0.4: 1 of 6 changed; no -> yes 1; gained 1, lost 0 (4 -> 5 right of 6); McNemar p 1.000 on right answers`, and pins the missing-second-run refusal with its exit code.

## Budgets and the ratchet

- Production Rust: two new files (`core/measure/diff.rs`, `cli/diff.rs`) and at most four existing files touched, likely `core/measure.rs`, `cli/mod.rs`, `cli/args/command.rs`, and `cli/audit.rs` for a shared reader. At most 450 nonblank production lines.
- Rust tests: at most 500 nonblank lines across at most two new test files and the edited inventory assertion.
- Scripts: `policy.py` and its self-test, at most 20 nonblank lines.
- Fixtures: the four captures and the README update.
- Prose: `specification/diff.md`, one index row, `spec/diff.md`, and command help.
- Every Rust file stays under the 500-nonblank-line ceiling.
- Dependencies: none. The log-space sum replaces a big-integer crate. A new dependency stops the build for a second review that names what it checked.

`sdlc/ratchet.json` rises by the measured Rust increase in the commit that needs it. That commit message says what grew, why it earns its lines, and where the builder looked for duplication first: at least 0113's reader, answer, and table code. This ticket raises the ceiling and widens the public surface, so a second agent reviews it and names what it checked.

Stop and re-score if the work exceeds any budget, needs a dependency, changes a golden, or reaches a behavior outside this page.

## Scope and exclusions

Allowed: the diff command and its help, the core pairing and test, the four captures, the policy extension, the specification and executable pages, the inventory assertion edit, and the ratchet.

Excluded: checking record text or question identity across runs; a probability-shift member; diffing `tag`, `score`, `find`, `recognize`, or `relate` answers; reading a recording or cache folder directly; any API in Rust, C, language packages, or databases; a `schema` member; any change to the goldens; edits to the Beatles Bench repository; live or paid calls; release artifacts.

## Dependencies and order

Depends on the ten functions done: ticket 0086 (public Rust API) landed on main. Depends on ticket 0113 landed, because it shares `core/measure.rs`, `core/measure/answer.rs`, the reader, the fixtures, and the test helper. The queue in `sdlc/planning/one-line-plan-2026-09-24.md` places diff after audit and before the release build. The two do not build in parallel.

## Complexity

Contract 2; State/timing 0; Reach 1; Proof 2; Cost of error 1; Total 6. Final level: 2. Reasons: a new public command with exact output and a statistical test ported from exact integers to floating point. The runtime holds no state and sends nothing.
