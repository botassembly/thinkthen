---
flow: build
priority: 165
opens: specification/diff.md spec/diff.md crates/thinkthen/src/core/measure crates/thinkthen/src/cli crates/thinkthen/tests/diff.rs crates/thinkthen/tests/fixtures/measure sdlc/ratchet.json sdlc/records sdlc/tickets CHANGELOG.md
---

# 0165: diff reads recognize and relate

Status: ready for review. Its only dependency is ticket 0147's output shape, which ADR 0056 now fixes with `strength` as each name's score. It builds after ticket 0147 lands. A fresh read-only reviewer accepts this ticket before it is built. Owner: Claude.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

`thinkthen diff` compares two `recognize` runs, two `relate` runs, or two cuts on one run. For each record it says which names or edges were gained, which were lost, and which names changed kind. With a key, it counts the key names each side found and runs McNemar on the key names only one side found.

Ian ruled on 2026-09-26: "audit and diff should be fully supported". Ticket 0147 keeps `audit`'s full support for `recognize`. This ticket gives `diff` the same reach.

## What happens today

Each line is at `origin/main` `19ca8302`.

- `specification/diff.md` names no `recognize` or `relate` behavior. It says diff "shares its readers, rule text, and failure rows with audit".
- `diff` reads `recognize` and `relate` lines through audit's readers. `Answer::said` in `core/measure/answer.rs` line 424 returns `Said::Unresolved` for both verbs. Two different runs therefore print no changed row and `changed` of 0.
- `audit` already reads each line's names or edges, their scores and the run cut in `core/measure/items.rs`. It matches them against a key under `--match strict` or `--match overlap`, and it refuses a band or a cut below the run cut.

## Design

### Reading and pairing

diff pairs answers by answer name and record id, as today. Failed answers leave, as today. A `recognize` or `relate` answer pairs only with an answer of the same verb. A paired answer of another verb exits 2, by the new failure row below.

Each side's item set is its names or edges at or above that side's cut. Side A takes `--threshold` or the run cut. Side B takes `--compare-threshold`, else `--threshold`, else the run cut. A cut is one number at or above the line's run cut. A band, or a cut below the run cut, exits 2 with audit's sentence under the `thinkthen: diff:` prefix. Two cuts on one run need no second run, as for `decide`.

### Matching the two sides

`--match strict`, the default, pairs an A name with a B name of the same `start`, `end` and `kind`. `--match overlap` pairs names of the same kind whose places overlap. Pairing is one to one in each side's order, as audit pairs said names with key names. Edges pair as audit pairs them: the same relation, source name and kind, and target name and kind. `--match` changes nothing for edges.

After pairing:

- **Lost** is an unpaired A item.
- **Gained** is an unpaired B item.
- **Changed kind** is an unpaired A name and an unpaired B name with the same `start` and `end` under `strict`, or overlapping places under `overlap`, and different kinds. They count as one changed-kind entry, not as a loss and a gain. Edges have no changed kind.

A record changes when it has any lost, gained or changed-kind item.

### With a key

The key is read as audit reads it, and `kinds.jq` filters work as they do for audit. For each record with a key value, each key item is matched or not on each side, under the same `--match`. Items matched on both sides and items matched on neither are concordant. McNemar runs on the key items matched only in B against those matched only in A. `mcnemar_on` is `"key names"` or `"key edges"`. Items on a side that match no key item are that side's extras. They are counted beside the test and do not enter it.

### Output

A changed record prints one row, in A's order:

```json
{"id":"r1","name":null,"lost":[{"text":"Revolver Paul McCartney","start":3,"end":26,"length":23,"kind":"person","strength":0.81}],"gained":[{"text":"Revolver","start":3,"end":11,"length":8,"kind":"work","strength":0.9},{"text":"Paul McCartney","start":12,"end":26,"length":14,"kind":"person","strength":0.97}],"changed_kind":[],"key":{"matched":[0,2],"extra":[1,0]}}
```

Items print in the command's own shape. `key` is null without a key value for the record.

The summary line keeps every member it prints today. For an item verb, `moves` is an empty list. `labeled` counts records with a key value. `right_a` and `right_b` count key items matched on each side. `gained` and `lost` count key items matched only in B and only in A. The summary adds `items_gained`, `items_lost`, `items_changed_kind`, `extra_a` and `extra_b`. They are null for other verbs, and the key members are null without a key. A reader ignores members it does not know, as `diff.md` already says.

`--table` prints a changed row as `ID[/NAME]  +G -L ~K`, then one line per item: `  + TEXT [START,END) KIND p P`, `  - …`, or `  ~ TEXT [START,END) KIND -> KIND`. An edge line prints `RELATION SOURCE -> TARGET p P`. The count line reads `A -> B: C of N changed; items gained G, lost L, changed kind K`. With a key it adds `; key names matched MA -> MB of T; extras XA -> XB` and the McNemar clause as today.

### Warnings and failures

The two warnings stay as they are. The question digest covers the recognize and relate cut, so two runs at different cuts warn, as they do for `decide`. One failure row is new:

| Case | Exit | Standard error |
| --- | ---: | --- |
| A `recognize` or `relate` answer paired with an answer of another verb | 2 | `thinkthen: diff: ROLE line N pairs recognize or relate with another verb` |

## Decisions

The owner's calls. Ian can overturn each.

1. **Changed kind is its own entry.** A name that keeps its place and changes kind is one change. Counting it as a loss and a gain would double it.
2. **Key McNemar runs per key item, not per record.** A record holds many names. A record-level right or wrong would hide most moves.
3. **Extras stay out of the test.** McNemar needs paired outcomes on one fixed set, and extras have no partner on the other side. They print beside it.
4. **`gained` and `lost` in the summary keep their meaning.** They count key items that became matched and key items that stopped matching, as they count right answers that were gained or lost for other verbs.
5. **One verb per pair.** Mixing an item verb with a yes/no verb in one pair has no meaning, so it exits 2.

## Edge cases

| Case | Expected |
| --- | --- |
| Same run twice at the run cut | `changed` 0 |
| B finds one more name | One row, one gained |
| A name moves from `person` to `work` at the same place | One changed-kind entry |
| Under `overlap`, `Abbey Road` in A and `Abbey Road Studios` in B, both `place` | Paired, no row |
| Under `strict`, the same | One lost and one gained |
| Under `overlap`, one A name overlaps two B names of its kind | The first B name pairs. The second is gained |
| A name at the cut on one side and just under it on the other | Lost or gained |
| Two cuts on one run, 0.5 and 0.8 | Rows list the names between the cuts as lost |
| `--compare-threshold 0.3` on a run cut at 0.5 | Exit 2, audit's sentence with the `diff` prefix |
| `--threshold 0.4:0.6` over recognize | Exit 2, the same sentence |
| A record with no names on either side | No row |
| Key item matched in B only | Counts toward `gained` and the test |
| Key with no value for a record | That record adds to no key count |
| `relate` edge whose source kind changes | One lost and one gained edge |
| A recognize answer paired with a decide answer | Exit 2, the new row |
| One side's record failed | The pair leaves, as today |
| `decide` goldens | Byte for byte unchanged |

## Proof

### Outside-in tests

Each drives `thinkthen diff` on saved result lines in `tests/diff.rs`. Diff sends nothing, so no recording is needed beyond saved lines.

1. **Two recognize runs.** Two hand-written three-record runs in the ticket 0147 output shape. The test pins every row and the summary line exactly, with and without the 200-sentence key filtered by `kinds.jq`.
2. **Two cuts on one real run.** Ticket 0147's replayed key run at five kinds, at its run cut and at a higher cut. The test pins the summary line. Every lost name's `strength` falls between the two cuts.
3. **Relate.** Two relate runs over one entity set. The test pins gained and lost edges and the key test.
4. **McNemar.** Discordant counts of 3 and 9 give `mcnemar_p` 0.145996, pinned against the existing exact function.
5. **Refusals.** A cut below the run cut, a band, and a mixed-verb pair each exit 2 with their exact sentences and print nothing on standard output.
6. **The table.** `--table` over test 1 pins every line.

Each answers the four questions. They protect diff's reading of name and edge sets, the key test, the cut rule and the refusals. Returning `Said::Unresolved` for item verbs, as today, fails tests 1 to 3. No existing test covers an item verb in `diff`. None needs a test-only hook.

### Edge-case table

One table test in `core/measure/diff.rs` runs every row of "Edge cases" that needs no command line, with inputs and exact lost, gained and changed-kind lists.

### Deliberate breaks

| Break | Row that turns red |
| --- | --- |
| Keep `Said::Unresolved` for recognize | Test 1 |
| Count a changed kind as a loss and a gain | Edge row: `person` to `work` |
| Ignore `--match overlap` | Edge row: `Abbey Road` under `overlap` |
| Pair many to one under `overlap` | Edge row: one A name overlaps two B names |
| Put extras into McNemar | Test 1 with the key |
| Read B at A's cut | Test 2 |
| Accept a cut below the run cut | Test 5 |
| Change any `decide` golden | Edge row: `decide` goldens |

## Pages

- `specification/diff.md`: a section "Names and edges" with the reading, matching, key test, rows, summary members, table lines and the new failure row. Status adds ticket 0165.
- `spec/diff.md`: one block over two recognize runs.
- `CHANGELOG.md`: one line.

## Ratchet

The ceiling is whatever ticket 0147 leaves. The estimate is +300 lines, from +200 to +450.

- Grows: item sets per side and their pairing (about 90), the key test per item (about 40), rows, summary and table (about 70), and the tests (about 150).
- Shrinks: nothing much. `items.rs` already holds the reader and the matcher, and diff reuses them. Before raising the ceiling, the builder checks whether audit's key tally and diff's side-against-side pairing can share one function.

## Stop rules

1. Stop until ticket 0147 lands with the output shape and `strength`.
2. Stop if the ceiling would pass 600 lines over the ceiling ticket 0147 leaves.
3. Stop if any existing diff golden changes.
4. Stop if matching two sides needs a rule audit's matcher does not give. Report it with options.
5. Stop if a deliberate break stays green.

## Routing

Owner and builder: Claude. Reviewer: a fresh read-only Claude session for the design and the diff. The change raises the ceiling and adds output members, so the diff review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 1; total 5. The risk is a McNemar count that reads well and counts the wrong set. Tests 1 and 4 pin it.

## Deferred gaps

- `compare`, the `jq` transform, stays for yes/no verbs.
- Record-level effects (`gained`, `lost`, `resolved`, `withdrawn`) stay undefined for item verbs. Each row carries its key counts instead.
- `annotate` answers that hold names are out of scope.

## What Ian can overturn

- Decisions 1 to 5.
- The row and summary member names.
- The table line forms.

## Closes

None.

## Evidence

- Starts from: Ian's ruling of 2026-09-26, "audit and diff should be fully supported". `specification/diff.md`, settled by ticket 0114. `core/measure/answer.rs` line 424 and `core/measure/items.rs` at `origin/main` `19ca8302`. Ticket 0135, which built audit's item grading. Ticket 0147 and ADR 0056 for the output shape and `strength`.
- Keeps: Pairing by answer name and record id. Every `decide`, `choose`, `tag`, `score`, `rank`, `find` and `annotate` behavior, golden and failure row. The two warnings. The exact McNemar function. audit's readers, matching and cut refusal.
- Changes: diff reads name and edge sets under each side's cut, reports gained, lost and changed-kind items, and runs McNemar on key items. The summary gains five members. One failure row is new.
- Proof: Six outside-in tests over saved lines and one replayed run, one edge-case table, and eight deliberate breaks with the row each turns red.
- Defers: `compare` for item verbs. Record-level effects for item verbs. Names inside `annotate` answers.
