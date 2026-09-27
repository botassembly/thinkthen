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
- The shared reader already counts a `relate` line whose `meta.failed_questions` is above 0 as failed (`core/measure/answer.rs` line 360). diff therefore drops its pair, as it drops any failed answer.
- The shared reader already refuses a key item whose kind or relation the line's question lacks, with audit's row `key line N names a level, label, or unit the question does not have`.
- The shared reader already refuses a `recognize` or `relate` line without `question`, such as a line saved without `--details`. diff prints `thinkthen: diff: first run line N holds an answer diff cannot grade; diff grades decide and choose`, or `second run` for B, and exits 2 (`cli/measure.rs`, `MeasureError::Ungradable`).

## Design

### Reading and pairing

Every input is a `--details` line, because diff reads the kinds, relations and run cut from `question`. A line without it is refused as today, with the sentence under "What happens today". diff pairs answers by answer name and record id, as today. Failed answers leave, as today, and a `relate` line with failed questions counts as failed, as today. A `recognize` or `relate` answer pairs only with an answer of the same verb. One diff holds only `recognize` pairs, only `relate` pairs, or only pairs of other verbs. Any mix exits 2, by the new failure rows below.

Each side's item set is its names or edges at or above that side's cut. Side A takes `--threshold` or the run cut. Side B takes `--compare-threshold`, else `--threshold`, else the run cut. A cut is one number at or above the line's run cut. A band, or a cut below the run cut, exits 2 with audit's sentence under the `thinkthen: diff:` prefix. Two cuts on one run need no second run, as for `decide`.

### Matching the two sides

`--match strict`, the default, pairs an A name with a B name of the same `start`, `end` and `kind`. `--match overlap` pairs names of the same kind whose places overlap. Edges pair as audit pairs them: the same relation, source name and kind, and target name and kind. An edge of a relation the question marks `either` also pairs with its endpoints swapped, as `audit.md` says under "Names and edges". `--match` changes nothing for edges.

Pairing is one to one and follows audit's order. Each side's kept items are taken by `strength` for names or `probability` for edges, high to low, ties in output order. Each A item in that order takes the first unmatched B item, in B's same order, that it pairs with.

After pairing:

- **Lost** is an unpaired A item.
- **Gained** is an unpaired B item.
- **Changed kind** is an unpaired A name and an unpaired B name with the same `start` and `end` under `strict`, or overlapping places under `overlap`, and different kinds. They count as one changed-kind entry, not as a loss and a gain. These entries pair one to one in the same order: each unpaired A name takes the first unpaired B name it can. Edges have no changed kind.

A record changes when it has any lost, gained or changed-kind item, or any key item matched on one side only. A changed record prints a row.

### With a key

The key is read as audit reads it, and `kinds.jq` filters work as they do for audit. The key is checked against both sides' questions. Each side reads the key under its own question. A side whose question has no kinds reads every key name as the kind `ENTITY`, by ticket 0147's audit rule, so the kind check never fires for that side. A key item whose kind or relation a side with kinds or relations lacks exits 2 with audit's row under the role `key`: `thinkthen: diff: key line N names a level, label, or unit the question does not have`. For each record with a key value, each side is matched to the key by audit's "One match each" rule, under the same `--match`. The side's kept items are taken strongest first, and each takes the first unmatched key item, in key order, that it matches. A key item is matched on a side when that rule pairs it. A side's extras are its items the rule leaves unmatched, audit's `false_yes`. Items matched on both sides and items matched on neither are concordant. McNemar runs on the key items matched only in B against those matched only in A. `mcnemar_on` is `"key names"` or `"key edges"`. Items on a side that match no key item are that side's extras. They are counted beside the test and do not enter it.

### Output

A changed record prints one row, in A's order. The row's members are `id`, `name`, `lost`, `gained`, `changed_kind` and `key`, in that order. `key` is `{"matched":[A,B],"extra":[A,B]}` for that record: the key items each side matched and each side's extras. `from`, `to`, `probability` and `effect`, which rows of other verbs print, are absent.

```json
{"id":"r1","name":null,"lost":[{"text":"Revolver Paul McCartney","start":3,"end":26,"length":23,"kind":"person","strength":0.81}],"gained":[{"text":"Revolver","start":3,"end":11,"length":8,"kind":"work","strength":0.9},{"text":"Paul McCartney","start":12,"end":26,"length":14,"kind":"person","strength":0.97}],"changed_kind":[],"key":{"matched":[0,2],"extra":[1,0]}}
```

A name that keeps its place and changes kind prints in `changed_kind`:

```json
{"id":"r2","name":null,"lost":[],"gained":[],"changed_kind":[{"from":{"text":"Abbey Road","start":0,"end":10,"length":10,"kind":"work","strength":0.74},"to":{"text":"Abbey Road","start":0,"end":10,"length":10,"kind":"place","strength":0.88}}],"key":null}
```

Each `changed_kind` entry is `{"from": A item, "to": B item}`. Items print in the command's own shape, whole. Today `items.rs` keeps only each item's kind and place, so the reader must now keep each item's whole object and its output index beside it. Within `lost`, `gained` and `changed_kind`, items keep their line's output order. Names print by `start`, then `end`, as `recognize` prints them. Edges keep the order `relate.md` gives them. A `changed_kind` entry takes its A item's place in that order. `key` is null without a key value for the record.

The summary line keeps every member it prints today. For an item verb, `moves` is an empty list. `labeled` counts records with a key value. `right_a` and `right_b` count key items matched on each side. `gained` and `lost` count key items matched only in B and only in A. The summary adds `key_items`, `items_gained`, `items_lost`, `items_changed_kind`, `extra_a` and `extra_b`. `key_items` counts the key items of the labeled records. They are null for other verbs, and the key members are null without a key. A reader ignores members it does not know, as `diff.md` already says.

`--table` prints a changed row as `ID[/NAME]  +G -L ~K`, then one line per item: `  + TEXT [START,END) KIND strength S`, `  - …`, or `  ~ TEXT [START,END) KIND -> KIND`. When a changed-kind entry's two places differ, as under `overlap`, its line prints both: `  ~ TEXT [S,E) KIND -> TEXT [S,E) KIND`. `S` prints with four decimals, as ticket 0147 keeps `strength`. An edge line prints `RELATION SOURCE (KIND) -> TARGET (KIND) p P`, with two decimals as today. The count line starts as today, with `A -> B`, the rule form, or the two-cuts form, and goes on with `: C of N changed; items gained G, lost L, changed kind K`. With a key it adds `; key names matched MA -> MB of T; extras XA -> XB` and the McNemar clause as today. For `relate` the clause reads `key edges` in place of `key names`. The only-in clause stays as today.

### Warnings and failures

The two warnings stay as they are. The question digest covers the recognize and relate cut, so two runs at different cuts warn, as they do for `decide`. Three failure rows are new. diff walks the pairs in A's order. The pairing row fires on the first pair whose two verbs differ when one of them is `recognize` or `relate`. Two other verbs that differ, such as `decide` and `choose`, pair as today and print a row. Otherwise the mix row fires on the first pair whose verb class differs from the first pair's class. The classes are `recognize`, `relate` and every other verb. `ROLE line N` names that pair's B line, or its A line when the pair has no B, as for two cuts on one run.

| Case | Exit | Standard error |
| --- | ---: | --- |
| A `recognize` or `relate` answer paired with an answer of another verb | 2 | `thinkthen: diff: ROLE line N pairs recognize or relate with another verb` |
| `recognize` or `relate` pairs beside pairs of other verbs | 2 | `thinkthen: diff: ROLE line N mixes recognize or relate with other verbs` |
| `recognize` pairs beside `relate` pairs | 2 | `thinkthen: diff: ROLE line N mixes recognize with relate` |

## Decisions

The owner's calls. Ian can overturn each.

1. **Changed kind is its own entry.** A name that keeps its place and changes kind is one change. Counting it as a loss and a gain would double it.
2. **Key McNemar runs per key item, not per record.** A record holds many names. A record-level right or wrong would hide most moves.
3. **Extras stay out of the test.** McNemar needs paired outcomes on one fixed set, and extras have no partner on the other side. They print beside it.
4. **`gained` and `lost` in the summary keep their meaning.** They count key items that became matched and key items that stopped matching, as they count right answers that were gained or lost for other verbs.
5. **One kind of verb per diff.** Mixing an item verb with a yes/no verb in one pair has no meaning, so it exits 2. A diff that holds item pairs beside other pairs, or `recognize` pairs beside `relate` pairs, also exits 2. Its summary would add counts of different things, such as key names and key edges, or right answers and matched items.

## Edge cases

| Case | Expected |
| --- | --- |
| Same run twice at the run cut | `changed` 0 |
| B finds one more name | One row, one gained |
| A name moves from `person` to `work` at the same place | One changed-kind entry |
| Under `overlap`, `Abbey Road` in A and `Abbey Road Studios` in B, both `place` | Paired, no row. With the key name `Abbey Road Studios`, both sides match it, so still no row |
| The same, with the key name `Studios` | Paired, but the key name matches in B only. One row, with no item lists filled and `key` of `{"matched":[0,1],"extra":[1,0]}` |
| Under `overlap`, `Abbey Road` `[0,10)` as `work` in A and `Abbey Road Studios` `[0,18)` as `place` in B | One changed-kind entry. Its table line is `  ~ Abbey Road [0,10) work -> Abbey Road Studios [0,18) place` |
| Under `strict`, the same | One lost and one gained |
| Under `overlap`, one A name overlaps two B names of its kind, and the stronger B name comes second in output order | The stronger B name pairs. The first B name is gained |
| The same with two B names of equal strength | The first in output order pairs. The second is gained |
| A name at the cut in A and just under it in B | Lost |
| A name just under the cut in A and at it in B | Gained |
| Two A names at one place, `work` at 0.9000 and `person` at 0.6000, and two B names there, `place` at 0.7000 and `event` at 0.8000 | Two changed-kind entries, paired strongest first: `work` -> `event` and `person` -> `place` |
| Two cuts on one run, 0.5 and 0.8 | Rows list the names between the cuts as lost |
| `--compare-threshold 0.3` on a run cut at 0.5 | Exit 2, `thinkthen: diff: recognize and relate take a single --threshold at or above the cut they ran with` |
| `--threshold 0.4:0.6` over recognize | Exit 2, `thinkthen: diff: recognize and relate take a single --threshold at or above the cut they ran with` |
| A record with no names on either side | No row |
| Key item matched in B only | Counts toward `gained` and the test |
| Key with no value for a record | That record adds to no key count |
| `relate` edge whose source kind changes | One lost and one gained edge |
| `either` edge printed `Ann`→`Bob` in A and `Bob`→`Ann` in B | Paired, no row |
| The same edges under a directed relation | One lost and one gained edge |
| `relate` line in B with `meta.failed_questions` of 1 | The pair leaves, as today. A regression guard |
| A key name of a kind that side B's question lacks | Exit 2, `thinkthen: diff: key line N names a level, label, or unit the question does not have` |
| A with kinds `person`, B with no kinds, both finding `Ada` at `[0,3)` | One changed-kind entry, `person` -> `ENTITY` |
| The same, with the key `{"entities":[{"kind":"person","start":0,"end":3}]}` | A matches as `person`, and B reads the key name as `ENTITY` and matches. `key` is `{"matched":[1,1],"extra":[0,0]}`. No refusal |
| A `decide` answer paired with a `choose` answer | A row, as today. No refusal |
| A recognize line saved without `--details` | Exit 2, `thinkthen: diff: first run line N holds an answer diff cannot grade; diff grades decide and choose`, as today |
| A recognize answer paired with a decide answer | Exit 2, the pairing row |
| A recognize pair beside a decide pair | Exit 2, the mix row |
| A recognize pair beside a relate pair | Exit 2, the recognize-with-relate row |
| One side's record failed | The pair leaves, as today |

## Proof

### Outside-in tests

Each drives `thinkthen diff` on saved `--details` result lines in `tests/diff.rs`. Diff sends nothing, so no recording is needed beyond saved lines.

1. **Two recognize runs.** Two hand-written three-record runs in the ticket 0147 output shape, and a hand-written key for the three records. The test pins every row and the summary line exactly, with and without the key. A third run with no kinds, diffed against A with the key, pins the `person` -> `ENTITY` rows.
2. **Two cuts on one real run.** `crates/thinkthen/tests/fixtures/measure/diff-recognize-key-run.jsonl` holds the output of ticket 0147's replayed key run at five kinds. The builder saves it once from that replay and commits it as a fixture. The diff test replays nothing. It diffs the file at its run cut of 0.5 and at a higher cut of 0.8, and pins the summary line. Every lost name's `strength` falls between the two cuts.
3. **Relate.** Each run file holds three `relate --details` lines, one per entity set, in the same order in A and B. Each line's `id` is its line number, so lines pair by id. Lines 1 and 2 carry the pinned edges: line 1 gains one edge and loses one, and line 2 holds the `either` edge printed both ways. B's line 3 carries `meta.failed_questions` of 1, and the test pins that its pair leaves. The test pins the rows and the key test.
4. **McNemar.** `thinkthen diff` runs end to end over two saved recognize runs and a key. The runs give exactly 3 key names matched only in A and 9 matched only in B. A holds extra names, so counting extras would move the result. The test pins `mcnemar_p` at 0.145996, the existing exact function's value, and pins `extra_a` and `extra_b`.
5. **Refusals.** A cut below the run cut, a band, a mixed-verb pair, a recognize pair beside a decide pair, a recognize pair beside a relate pair, a key kind that side B's question lacks, and a recognize line without `--details` each exit 2 with their exact sentences and print nothing on standard output. A `decide` answer paired with a `choose` answer still prints its row, as today.
6. **The table.** `--table` over test 1 pins every line.

Each answers the four questions. They protect diff's reading of name and edge sets, the key test, the cut rule and the refusals. Returning `Said::Unresolved` for item verbs, as today, fails tests 1 to 4. No existing test covers an item verb in `diff`. None needs a test-only hook.

### Edge-case table

One table test in `core/measure/diff.rs` runs every row of "Edge cases" that needs no command line, with inputs and exact lost, gained and changed-kind lists. The failed-questions row, the key rows, the verb rows and the `--details` row need the line reader, so tests 1, 3 and 5 carry them.

### Deliberate breaks

| Break | Row that turns red |
| --- | --- |
| Keep `Said::Unresolved` for recognize | Test 1 |
| Count a changed kind as a loss and a gain | Edge row: `person` to `work` |
| Ignore `--match overlap` | Edge row: `Abbey Road` under `overlap` |
| Pair many to one under `overlap` | Edge row: the stronger B name comes second in output order |
| Put extras into McNemar | Test 4 |
| Drop the `either` swap | Edge row: `either` edge printed both ways |
| Pair B names in output order, not strongest first | Edge row: the stronger B name comes second in output order |
| Keep a `relate` line with failed questions | Test 3 |
| Allow recognize pairs beside decide pairs | Test 5 |
| Check the key against side A's question only | Test 5 |
| Print a row only for lost, gained or changed-kind items | Edge row: the key name `Studios` |
| Read B at A's cut | Test 2 |
| Accept a cut below the run cut | Test 5 |
| Send `decide` pairs through the item path | The existing `decide` goldens in `tests/diff.rs` |

## Pages

- `specification/diff.md`: a section "Names and edges" with the reading, matching, key test, the item row members, summary members, table lines and the three new failure rows. Status adds ticket 0165.
- `spec/diff.md`: one block over two recognize runs.
- `CHANGELOG.md`: one line.

## Ratchet

The ceiling is whatever ticket 0147 leaves. The estimate is +390 lines, from +270 to +550.

- Grows: item sets per side and their pairing (about 90), the reader keeping each item's whole object (about 30), the `either` swap and the verb-mix refusals (about 20), the key test per item (about 40), rows, summary and table (about 70), and the tests (about 150).
- Shrinks: nothing much. `items.rs` already holds the reader and the matcher, and diff reuses them. Before raising the ceiling, the builder checks whether audit's key tally and diff's side-against-side pairing can share one function.

## Stop rules

1. Stop until ticket 0147 lands with the output shape and `strength`.
2. Stop if the ceiling would pass 600 lines over the ceiling ticket 0147 leaves.
3. Stop if any existing diff golden changes.
4. Stop if matching two sides needs a rule audit's matcher does not give. Report it with options.
5. Stop if a deliberate break stays green.
6. Stop if test 2's cut of 0.8 drops no name from the saved run. Report the run's strengths and pick another cut with the reviewer.

## Routing

Owner and builder: Claude. Reviewer: a fresh read-only Claude session for the design and the diff. The change raises the ceiling and adds output members, so the diff review names what it checked.

## Complexity

Contract 2; state and timing 0; reach 1; proof 1; cost of error 1; total 5. The risk is a McNemar count that reads well and counts the wrong set. Tests 1 and 4 pin it.

## Deferred gaps

- `compare`, the `jq` transform, stays for yes/no verbs.
- Record-level effects (`gained`, `lost`, `resolved`, `withdrawn`) stay undefined for item verbs. Each row carries its key counts instead.
- `annotate` answers that hold names are out of scope.
- Comparing recognize's `relations` stays out, as audit leaves them ungraded. diff compares a recognize line's names only.

## What Ian can overturn

- Decisions 1 to 5.
- The row and summary member names.
- The table line forms.

## Closes

None.

## Evidence

- Starts from: Ian's ruling of 2026-09-26, "audit and diff should be fully supported". `specification/diff.md`, settled by ticket 0114. `core/measure/answer.rs` line 424 and `core/measure/items.rs` at `origin/main` `19ca8302`. Ticket 0135, which built audit's item grading. Ticket 0147 and ADR 0056 for the output shape and `strength`.
- Keeps: Pairing by answer name and record id. The refusal of a line without `--details`. A row for two other verbs that differ. Dropping a `relate` line with failed questions as a failed answer. The key's kind and relation check. Every `decide`, `choose`, `tag`, `score`, `rank`, `find` and `annotate` behavior, golden and failure row. The two warnings. The exact McNemar function. audit's readers, matching and cut refusal.
- Changes: diff reads name and edge sets under each side's cut, reports gained, lost and changed-kind items, and runs McNemar on key items. The summary gains six members. Three failure rows are new.
- Proof: Six outside-in tests over saved lines, one of them saved once from a replay, one edge-case table, and fourteen deliberate breaks with the row each turns red.
- Defers: `compare` for item verbs. Record-level effects for item verbs. Names inside `annotate` answers.
