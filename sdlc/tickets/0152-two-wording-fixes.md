---
flow: build
priority: 152
opens: crates/thinkthen/src/public/options.rs crates/thinkthen/tests/public_controls.rs libraries/python/tests/test_inputs.py crates/thinkthen/src/core/measure/answer.rs crates/thinkthen/src/core/measure/audit.rs crates/thinkthen/src/core/measure/rows.rs crates/thinkthen/src/cli/audit/table.rs crates/thinkthen/src/public/annotated.rs crates/thinkthen/src/public/engine.rs crates/thinkthen/src/public/results.rs crates/thinkthen/transforms crates/thinkthen/tests/audit.rs crates/thinkthen/tests/audit_verbs.rs crates/thinkthen/tests/diff.rs crates/thinkthen/tests/version.rs crates/thinkthen/tests/support/measure.rs crates/thinkthen/tests/fixtures/measure/README.md transforms transforms/counts/example.sh transforms/rows/record.sh specification/decide.md specification/choose.md specification/channels.md specification/threshold.md specification/find.md specification/filter.md specification/records.md specification/result.md specification/annotate.md specification/audit.md specification/diff.md specification/README.md spec/audit.md demos/README.md demos/FINDINGS.md demos/01-refund-gate/README.md demos/02-route-a-ticket/README.md demos/13-pick-a-threshold/README.md demos/16-triage-pipeline/README.md demos/16-triage-pipeline/self-test demos/19-no-or-could-not-ask/README.md demos/25-check-the-judge/README.md demos/41-tune-a-question-file/README.md conformance/README.md conformance/record-values.json libraries/c/include/thinkthen.h libraries/rust/examples/choose.rs sdlc/scripts/demos sdlc/scripts/demos-self-test sdlc/planning/adr/0017-libraries-over-one-bound-core.md sdlc/ratchet.json sdlc/records sdlc/tickets sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md sdlc/issues
---

# 0152: A huge deadline prints a short number, and "not sure" replaces "unresolved"

Status: complete. Part B passed fresh code review and focused validation on 2026-09-27. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Part A landed on 2026-09-26 after a fresh read-only code review; its record is `sdlc/records/0152-build-part-a.md`. Part B builds after ticket 0146 lands. Owner: Codex under Ian's handover.

Review route: a fresh read-only Codex reviewer checks Part B's final diff. Ian's handover routes this accepted ticket to Codex; the settled behavior stays fixed.

## Outcome and authority

A user who passes a deadline of `1e300` reads a refusal of about 90 characters. Today the sentence holds a 301-digit number. A user who reads the `audit` table, a `specification/` page, a demo page, `transforms/README.md`, `conformance/README.md`, the C header, the public Rust docs, the Rust example, or a built-in transform meets "not sure" for the answer inside a band. A program that reads `audit`, `diff`, or a transform meets one machine word, `unsure`. That word already names the answer in the Rust library (`Answer::Unsure`), the C header (`THINKTHEN_UNSURE`), DuckDB's answer text, the site's catalog, and every conformance case name but one. That one, `null-and-unresolved` in `conformance/record-values.json`, becomes `null-and-unsure` here. No verb's output or exit code changes.

The authority is row H2 of `sdlc/planning/backlog-0-1-2026-09-26.md`, goal 4 (honest docs), and the coordinator's brief for 0152. The ticket settles the last two items, 1 and 16, of `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`. Ticket 0138 left both open because other tickets held their files then. Those tickets (0134, 0135, 0137) have landed.

The ticket builds in two parts. Each part lands whole, with its own proof and record.

- **Part A** fixes item 1, the deadline number. It touches three files and builds now.
- **Part B** fixes item 16, the word. It touches many pages that ticket 0146 also edits, so it builds after 0146 lands. "Collisions and build order" gives the rule.

## Items checked against main

Checked at `origin/main` `a2fe448d` on 2026-09-26 by reading the source and a search of every tracked file.

| Item | State on main | This ticket |
| --- | --- | --- |
| 1. A `1e300` deadline prints 301 digits | open. `CallOptions::deadline_seconds` formats the raw `f64` with `{value}` (`crates/thinkthen/src/public/options.rs:129`). `libraries/python/tests/test_inputs.py:80` pins the long form. The milliseconds sentence at `options.rs:156` takes an `i64`, which prints at most 20 characters | Part A |
| 16. "unresolved" names the not-sure answer | open. 199 tracked files say it. 91 of them sit in `sdlc/` history and 8 in `probes/`. The rest are pages, transforms, tests, fixtures, and source. The machine contract carries it in three places: the `audit` row member `unresolved`, the answer token `"unresolved"` in `audit` disagreements and `diff` rows, and the keys and values of seven built-in transforms. No verb's result carries it | Part B |

## Evidence

- Starts from: The wording issue's items 1 and 16, rechecked at `origin/main` `a2fe448d`. Ticket 0138's deferred gaps 1 and 2. ADR 0041, which sets the deadline cap and refusal and asks for no digits. ADR 0017 section 6 item 4, which made "unsure" the public machine word and kept "unresolved" in the specification's grammar. Ticket 0082, which defined `unresolved` as the formal name in one sentence of `specification/decide.md` and pinned it in `tests/version.rs`. The product vocabulary in the marketing repository, which says "not sure" everywhere and lists "unresolved" among the words not to use. The site's catalog (`site/src/data/catalog.mjs`), which already keys the answer `unsure` and labels it "not sure". DuckDB's `answer_word`, which already prints `unsure`. The prototype fixtures under `crates/thinkthen/tests/fixtures/measure/`, which print `unresolved` and stay byte copies.
- Keeps: Every verb's output, result schema, and exit code. Exit 3 for a not sure answer. `null` as the not-sure value. The deadline cap, the refusal kind, and the NaN and infinity spellings. The milliseconds sentence. The prototype fixtures, byte for byte, with their checksums. The `tied` state and every other `audit` and `diff` member. Internal Rust identifiers such as `Outcome::Unresolved`.
- Changes: The seconds deadline sentence prints a number longer than 20 characters in exponent form. The machine word `unresolved` becomes `unsure` in `audit` JSON, `diff` JSON and tables, and the built-in transforms. The `audit` table's count line says "not sure". Pages, demo prose, transform comments, and public Rust docs say "not sure". Two checks keep the old word out. ADR 0017 item 4 gains an amendment.
- Proof: Part A's two outside-in tests with plants P1 and P2. Part B's golden, table, zero-hit, and demos-vocabulary checks with plants P3 to P8. "Proof" lists each.
- Defers: Internal identifiers and comments, the probes' scripts, `sdlc/` history, the sweep's `resolved` names, the marketing repositories' copies, and the prototype's own spelling. "Deferred gaps" lists each.

## Design

### Part A: the deadline number (item 1)

`deadline_seconds` in `crates/thinkthen/src/public/options.rs` formats its value through one private function, `shown`. `shown` returns the plain `Display` form when that form holds at most 20 characters. Otherwise it returns the `LowerExp` form, `{value:e}`. The refusal sentence stays the same apart from the number.

Twenty characters is the widest plain form a whole-number budget reaches. `u64::MAX` has 20 digits, and the milliseconds path's `i64::MIN` has 20 characters with its sign. Every value a user types as an ordinary number keeps its plain form. `4294967296` stays `4294967296`. NaN, `inf`, and `-inf` are short, so they print as they do now.

`deadline_millis` does not change, because an `i64` never passes 20 characters. `deadline_after` does not change, because it prints whole seconds from a `u64`.

Every surface that passes a seconds deadline to `deadline_seconds` prints the same sentence. The search found one test that pins the long form, in Python. Ruby and R check only the error kind. TypeScript, C, and SQLite refuse a huge deadline with their own sentences, which already print short forms (`1e+300` in TypeScript).

### Part B: one word for the not-sure answer (item 16)

#### The machine word

`unresolved` becomes `unsure` wherever a program reads it:

1. The `audit` row member `unresolved` becomes `unsure`. The field of `Row` in `core/measure/audit.rs` takes the new name, and `rows.rs` and `cli/audit/table.rs` follow it. The internal `Counts.unresolved` field keeps its Rust name, with a serde rename so nested `suggested` and `held` counts also serialize as `unsure`. The `Outcome` and `Said` variant names stay.
2. `Said::text` in `core/measure/answer.rs` returns `"unsure"` for the no-answer state. That token reaches `audit` disagreements and `diff`'s `from`, `to`, and `moves`, in JSON and in the `diff` table.
3. The built-in transforms rename the key and value. In each of `band`, `calibration`, `compare`, `counts`, `score`, `sweep`, and `triage`, the key `unresolved` becomes `unsure` and `accuracy_unresolved` becomes `accuracy_unsure`. Sweep's internal `unique_unresolved` and `right_unresolved` become `unique_unsure` and `right_unsure`. `compare`'s flip names read `no to unsure` and `unsure to yes`. `triage`'s reason reads `unsure`. `crates/thinkthen/transforms/*.jq` and `transforms/*/*.jq` stay identical copies. The comments in `transforms/counts/example.sh` (line 2) and `transforms/rows/record.sh` (line 28) say not sure. Their tests and expected files (`transforms/compare/test.sh`, `transforms/sweep/test.sh`, `transforms/sweep/decision-expected.json`, `transforms/sweep/decision-empty-expected.json`, `transforms/triage/cases.jsonl`, `transforms/triage/expected.jsonl`) take the new names.

#### The printed and page words

- The `audit` table's count line reads `N right, N wrong, N not sure, N tied`. It is a sentence for a person, so it takes the vocabulary word. The `diff` table's `FROM -> TO` columns print answer tokens, the same tokens as the JSON, so a move reads `unsure -> yes`. An option name sits in the same column, so the column cannot print a phrase with a space and stay readable.
- Pages say "not sure" in prose and `unsure` where they name the member, the token, or a key. That covers the twelve `specification/` pages in the frontmatter, `spec/audit.md`, `transforms/README.md`, the transform comments, `conformance/README.md`, the C header comment, the Rust example `libraries/rust/examples/choose.rs`, the public rustdoc in `public/annotated.rs`, `public/engine.rs`, and `public/results.rs`, and the demo pages and `demos/FINDINGS.md`. `thinkthen transform show` prints each transform's comments, so those comments count as printed lines.
- `specification/decide.md` replaces the definition sentence with: `` `unsure` is the machine name for a not sure answer, in `audit`, `diff`, and the built-in transforms. ``
- `specification/audit.md` and `diff.md` drop the sentence that the table keeps the prototype's word. They say the port prints `unsure` where the prototype printed its older word for a not sure answer, and the audit table's count line says not sure. The state-name rule at `audit.md:240` stays true to the prototype: the prototype reads an option spelled with its older word, or `tied`, as that state, and the port keeps each one an option. The page describes that option without writing the older word.
- Two examples use "unresolved" in its plain sense, a failure still open. The `annotate.md` and `result.md` example question `unresolved` becomes `open`, asking "Is this still open?". `filter.md`'s example evidence reads "This mentions an open action." `specification/` then holds no "unresolved" at all, which the zero-hit check needs.
- The later `specification/types.md` page uses "not sure" in prose. The executable `spec/annotate.md` example uses the same `open` question, and its two local request fixtures take SHA-256 filenames computed from the changed requests. Their responses do not change.
- The conformance case `null-and-unresolved` in `conformance/record-values.json` becomes `null-and-unsure`. No surface names the case id, by a search of every tracked file.
- Demo 02's shell variable reads `label=not_sure`, as `choose --help` already teaches with `pick=not_sure`.
- ADR 0017 gains an amendment dated at the build. Item 4 keeps "unsure" as the machine word. The specification now uses it too, and pages say "not sure". It notes that this reverses the item's "the specification keeps unresolved" clause.

#### The prototype fixtures

The fixtures under `tests/fixtures/measure/golden/` stay byte copies of the prototype, with the checksums `every_fixture_keeps_its_checksum` pins. `support/measure.rs` gains one function, `ported`, next to `without_added`. It rewrites only the audit count spelling on the expected side before a test compares:

- in audit JSON lines, the count member `"unresolved":` becomes `"unsure":`, including nested `suggested` and `held` counts;
- in an `audit` table, ` unresolved, ` becomes ` not sure, `;
- in the existing `extra/diff-annotate.jsonl` golden, two captured below-cut `kind` states and their summary move change explicitly in `diff.rs`. The porter leaves every user option string alone. No current `diff` table golden holds the old state token.

The existing audit golden, specific audit comparison, and audit replay test compare against the ported fixture. `replay/audit.jsonl` holds the prototype's spelling too. The diff goldens compare directly, except the named `extra/diff-annotate.jsonl` state changes above. `tests/fixtures/measure/README.md` records the rename beside the members ticket 0125 added. A compiled `diff` regression keeps an option literally named `unresolved` unchanged in output and through the porter.

#### The checks that keep the old word out

1. `tests/version.rs` replaces `the_specification_defines_unresolved_once_and_keeps_the_closed_wording` with `no_page_or_transform_says_unresolved`. It finds the new definition sentence once, in `decide.md`. It finds no "unresolved", in any case, in any page under `specification/`, in `transforms/README.md`, or in any built-in transform under `crates/thinkthen/transforms/`. It keeps the old test's other assertions: the banned model phrases, the index line, the demos line, the README's four outcomes, and the `score` sentence.
2. `sdlc/scripts/demos` moves `unresolved` from the help-word rule to the fixed-phrase rule. The check then fails on a green demo page, the root `README.md`, `demos/README.md`, and every built help. The comment above it changes to match. `demos-self-test` gains one planted page and pins its whole line.

## Decisions

1. **Rename the machine word to `unsure` now.** The alternative keeps `unresolved` as the key and records it in the vocabulary. The issue offers both. Before 0.1 no outside program reads `audit`, `diff`, or a transform, so a rename costs only this repository's files, and Part B already opens most of them for the prose. After 0.1 the same rename breaks callers and needs a versioned member or a deprecation period. The public API, the C header, DuckDB, the site, and every conformance case name but one already say `unsure`. Keeping `unresolved` would leave two machine words for one answer. `not_sure` was the other candidate. It matches the vocabulary's spoken words, but no shipped interface uses it, and ADR 0017 item 4 already ruled `unsure`.

   Authority: backlog row H2 says this ticket records whether the key `unresolved` stays, so the ticket makes the choice. Ian accepted ADR 0017 on 2026-09-21 with "We can always fix it." The amendment to its item 4 follows that acceptance. The lander reports the amendment in the daily status as a choice Ian can overturn.
2. **The `audit` count line says "not sure", and the `diff` table prints `unsure`.** The count line is a sentence. The `diff` column holds tokens beside option names.
3. **Keep the prototype fixtures as byte copies and port them in the test.** A recapture would break their recorded checksums and their tie to the prototype. The test already strips members the port added, so one more documented rewrite follows the same rule.
4. **Print a deadline over 20 characters in exponent form.** The issue offered this or naming only the cap. The exponent form keeps the user's own number in the sentence. The 20-character bound keeps every ordinary number plain.
5. **Keep internal identifiers.** `Outcome::Unresolved`, `Said::Unresolved`, internal comments, and test names reach no user. Renaming them would touch core files ticket 0146 edits and add churn with no reader.
6. **Build in two parts.** Part A shares no page with 0146 and ships at once. Part B waits for 0146, because 0146's stop rule 7 halts it if another in-flight ticket opens a file it needs.
7. **Hold `specification/` at zero hits.** Rename the two plain-sense examples, and describe the prototype's spelling as its older word for a not sure answer. An absolute zero-hit check is simpler to hold than a list of allowed sentences.

## Edge cases

### Part A: `deadline_seconds`

| Input | Sentence after "a deadline of " | Result |
| --- | --- | --- |
| `1e300` | `1e300 seconds is not -1, 0, or a positive budget of at most 4294967295 seconds` | Usage, 92 characters, no send |
| `-1e300` | `-1e300 seconds ...` | Usage |
| `-1e-300` | `-1e-300 seconds ...` (plain form holds 302 characters) | Usage |
| `f64::MAX` | `1.7976931348623157e308 seconds ...` | Usage, 109 characters |
| `1e20` | `1e20 seconds ...` (plain form holds 21 characters) | Usage |
| `1e19` | `10000000000000000000 seconds ...` (20 characters, plain) | Usage |
| `4294967296` | `4294967296 seconds ...` | Usage, unchanged |
| `4294967295.5` | `4294967295.5 seconds ...` | Usage, unchanged |
| `-2`, `-0.5` | `-2 seconds ...`, `-0.5 seconds ...` | Usage, unchanged |
| NaN, `inf`, `-inf` | `NaN seconds ...`, `inf seconds ...`, `-inf seconds ...` | Usage, unchanged |
| `-1`, `0`, `0.5`, `4294967295` | none | Accepted, unchanged |
| `deadline_millis(i64::MAX)` | `9223372036854775807 milliseconds ...` | Usage, unchanged |

### Part B: the word

| Input | Output after Part B |
| --- | --- |
| `decide` under a band, probability inside it | `null`, exit 3, unchanged |
| `audit` JSON row | member `unsure` in the place `unresolved` held, same count |
| `audit --table` count line | `4 right, 2 wrong, 0 not sure, 0 tied` |
| `audit` over `rank` as run | every answer counts as `unsure` (`/unsure: 6` in `audit_verbs.rs`) |
| `audit` disagreement or `diff` move for a band's middle | token `unsure` |
| `diff --table` move | `r3  unsure -> yes  p 0.45 -> 0.45` and `; unsure -> yes 1` |
| `choose` exact tie in `audit` | `tied`, unchanged |
| A `choose` option named `unsure`, `unresolved`, or `tied` | stays an option, as the port rule says. In output the token `unsure` can match an option name, as `unresolved` could before |
| `thinkthen transform show counts \| jq` over a run | `{"yes":18,"no":18,"unsure":4}` |
| `band` transform | `unsure` and `accuracy_unsure` |
| `sweep` transform, choose mode | `resolved`, `unsure`, `ties`, `accuracy_resolved`, `accuracy_unsure` |
| `compare` transform | flip names `no to unsure`, `unsure to no`, `unsure to yes`, `yes to unsure` |
| `triage` transform, a null value | `{"action":"review","reason":"unsure"}` |
| A saved result row with a null value | unchanged. No result row ever carried the word |

## Proof

Every test runs the built command, the public Rust API, the built Python extension, a transform through `jq`, or a rung script. None needs a test-only export, flag, or hook.

| Test | Part | Proof | Planted fault that turns it red |
| --- | --- | --- | --- |
| `public_controls.rs` `deadline_numbers_follow_the_host_table_and_the_last_call_wins` | A | New rows pin the whole sentence for `1e300`, `-1e-300`, and `f64::MAX` as written literals. The existing rows keep `4294967296`, NaN, and the infinities | P1: format with `{value}` again. The `1e300` row reads 301 digits. P2: always format with `{value:e}`. The `4294967296` row reads `4.294967296e9` |
| `libraries/python/tests/test_inputs.py` `test_deadlines_follow_adr_0041` | A | The `1e300` line pins `UsageError a deadline of 1e300 seconds ...`. The count of sends stays 2 | P1 |
| `audit.rs` `old_goldens_hold` and `diff.rs` `goldens_match`, over the ported fixtures | B | Every `audit` JSON golden, table capture, and `diff` golden matches after the rename | P3: return `"unresolved"` from `Said::text` again. The `extra/diff-annotate.jsonl` golden goes red. P4: restore the `unresolved` member name. Every `audit` JSON golden goes red. P5: restore ` unresolved,` in the table line. The three table captures go red |
| The `diff.rs` table row for `small/decide-band.jsonl` (line 134), `audit_verbs.rs` rank row, `spec/audit.md` | B | Each pins the new token or line | P3 turns the `diff.rs` row red. P4 turns the rank row red. P5 turns `spec/audit.md` red under the `spec` rung |
| `version.rs` `no_page_or_transform_says_unresolved` (replaces the one-definition test) | B | The definition sentence appears once, in `decide.md`. No page under `specification/`, `transforms/README.md`, or built-in transform says "unresolved" | P6: put "unresolved" back in one prose line of `specification/channels.md`. P7: put `unresolved` back as the key in `crates/thinkthen/transforms/counts.jq` |
| `transforms/*/test.sh` and the demos that pin transform output (13, 16, 25) | B | Each pins the renamed keys and values | P7 also turns `transforms/counts` and demo 25 red |
| `demos-self-test`, a new planted page | B | A green page whose prose says "unresolved" fails with `demos: page/README.md:9: "unresolved" breaks the fixed-phrase rule` | P8: leave `unresolved` under the help-word rule. The planted page passes |

The four questions for each new or changed test:

- **What behavior does it protect?** Part A's rows protect a refusal a person can read, and plain numbers where the number is ordinary. Part B's goldens protect the one machine word across `audit` and `diff` and the prototype tie. The zero-hit and demos checks protect the vocabulary in every page and transform a user reads.
- **What credible regression fails it?** P1 to P8. P1, P3, P4, P5, and P8 are today's code. P2 is the easy overcorrection. P6 and P7 are the slip a later page or transform makes.
- **Why does no existing test catch it?** No Rust row covers a value past 20 characters, and the Python test pins the long form. The goldens pin the old word. The one-definition test pins the old sentence and looks at no other use. The demos check allows the word on pages.
- **Does it need a test-only hook?** No. `ported` rewrites the expected side of a recorded external fixture, the same way `without_added` does. It computes nothing the code under test computes.

## Budgets

Nonblank lines, measured with `grep -c .` against `origin/main` at each part's merge.

- Part A: `crates/thinkthen/src` at most 10 added. `crates/thinkthen/tests` at most 12 added. `libraries/python/tests` at most 1 changed.
- Part B: `crates/thinkthen/src` at most 6 added, net. `crates/thinkthen/tests` at most 45 added, net. No test file crosses the lint's 500 nonblank lines. `version.rs` holds 248 today and `support/measure.rs` holds 415.
- Part B: the transforms, pages, demos, and fixtures change words in place. They add at most 15 nonblank lines together, net, of which the ADR 0017 amendment takes at most 6.
- Part B: `sdlc/scripts/demos` and `demos-self-test` add at most 6.
- `sdlc/ratchet.json` moves to the measured total in the commit that changes the code, if the total grows.

## Stop rules

1. Stop Part B if ticket 0146 has not landed on main.
2. Stop Part B before editing a file that another in-flight ticket opens. Hand back the file and the ticket. "Collisions and build order" names the known ones.
3. Stop if either part changes a verb's output, the result schema, or an exit code.
4. Stop if a library or SQL surface test outside Part A's Python line pins a changed sentence or member.
5. Stop if the change needs a file under `site/`.
6. Stop before crossing a budget.
7. Stop if any plant stays green.
8. Stop if `every_fixture_keeps_its_checksum` needs a changed checksum.

## Collisions and build order

### Files shared with 0146

Ticket 0146 (`ticket/0146-command-batches-decide-filter-rank`) opens `cli/`, several specification pages, `demos`, the whole `crates/thinkthen/tests` folder, and `sdlc/issues`. This ticket shares these files with it:

- Part A: `crates/thinkthen/tests/public_controls.rs`, inside 0146's `crates/thinkthen/tests`. 0146 names no change there, because its pins go on command runs and this test calls the library. `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md`, inside 0146's `sdlc/issues`. 0146 does not edit that issue.
- Part B: `specification/decide.md`, `filter.md`, `records.md`, `result.md`, and `channels.md`. `spec/audit.md`. Under `demos/`: `README.md`, `FINDINGS.md`, and the pages and script of demos 01, 02, 13, 16, 19, 25, and 41. Under `crates/thinkthen/tests`: `audit.rs`, `audit_verbs.rs`, `diff.rs`, `version.rs`, `support/measure.rs`, and `fixtures/measure/README.md`. `crates/thinkthen/src/public/results.rs`. The wording issue file.
- Both: `sdlc/ratchet.json`, if the source total grows. `sdlc/records` and `sdlc/tickets`, which every ticket shares with new files only. `sdlc/issues`, where Part B adds one new site issue and moves the wording issue to `closed/`.

### Files shared with other tickets in flight

- 0147 (`ticket/0147-recognize-adr`, ready) opens `specification/audit.md`, `channels.md`, `records.md`, and `result.md`. Part B shares all four.
- 0148 (`ticket/0148-engine-settings-everywhere`, ready, builds after 0146) opens `libraries/python`, `libraries/c`, `crates/thinkthen/src/public/engine.rs`, `conformance/README.md`, ADR 0017, and `sdlc/ratchet.json`. Part A shares `libraries/python/tests/test_inputs.py`. Part B shares `public/engine.rs`, `libraries/c/include/thinkthen.h`, `conformance/README.md`, and ADR 0017. Either part shares `sdlc/ratchet.json` if its source total grows. Part B shares `sdlc/issues` with one new file and one move.

### The order

1. Part A builds now in the lane the coordinator names. It lands before 0146 or 0148 starts to build. If either starts first, Part A waits for it to land and merges `origin/main`.
2. 0146 builds and lands.
3. Part B builds once 0146 has landed and neither 0147 nor 0148 is building. If 0147 or 0148 has landed, Part B merges `origin/main` first. If either is still waiting, the coordinator picks the order. Part B's shared lines are word swaps, so the later ticket merges them with little work.

## Marketing's files

`site/` holds no "unresolved" today. The site's catalog already keys the answer `unsure` and labels it "not sure". The site reads `specification/settings.md` raw, and this ticket leaves that page alone. No `site/` file needs the change.

Beatles Bench belongs to marketing, and two of its pages print the old lines. Nobody in this repository edits them. In Part B's landing commit, the lander files one issue in this repository's `sdlc/issues/` on main. The issue names:

- `functions/audit/README.md`, whose `audit` table line says `0 unresolved`. After Part B, `audit` prints `0 not sure` there.
- `functions/diff/README.md` line 44, which says `unresolved -> no 44`. After Part B, `diff` prints `unsure -> no 44` there.
- The vocabulary note: the product vocabulary can add `unsure` as the machine word for "not sure". It needs no other change.

The coordinator records the follow-up issue. No external message is authorized in this handoff.

## Scope and exclusions

Excluded: every verb's output and exit code, the result schema, `site/`, `probes/`, `sdlc/` history, old ADRs apart from the ADR 0017 amendment, internal Rust identifiers and comments, the prototype fixture bytes, and every library and SQL surface except Part A's Python line and Part B's C header comment. No live or paid call. `sdlc/scripts/live` never runs for this ticket.

## Routing

Builder: Codex in the retained command lane for Part B. Reviewer: a fresh read-only Codex reviewer for Part B's code.

## Complexity

Contract 2; state and timing 0; reach 3; proof 2; cost of error 1; total 8. Final level: 2. Part A is one function. Part B renames a machine word across audit, diff, and transforms, and sweeps prose across many pages. The goldens, the zero-hit check, and the demos check guard it.

## Deferred gaps

1. Internal identifiers keep the old word: `Outcome::Unresolved`, `Said::Unresolved`, the internal count field, test function names, and comments in `core/` and `cli/`. None reaches a user.
2. `probes/*/job.sh` and `probes/06-hostile-text/analyse.sh` keep the word. They are the measurements behind past rulings.
3. `sdlc/` history keeps the word: records, tickets, closed issues, planning pages, and ADRs other than 0017.
4. The sweep transform's `resolved`, `right_resolved`, and `accuracy_resolved` keep their names. `audit` calls the same count `answered`. Aligning them is a separate choice.
5. The prototype's own output still says `unresolved`. The test ports it.
6. Beatles Bench's two pages and the vocabulary note belong to marketing. The coordinator keeps the follow-up issue named in "Marketing's files"; no external message is part of this handoff.
7. Question-file tests that name a question `unresolved` (`tests/backend/annotate.rs`, `core/question_set/tests.rs`) keep it. There it is a question name.

## What Ian can overturn

- Decision 1: the machine word becomes `unsure` before 0.1. The other choices keep `unresolved` and record it in the vocabulary, or use `not_sure`. Before 0.1 the overturn costs one revert. After 0.1 any rename breaks callers.
- Decision 2: the `audit` count line says "not sure" while the `diff` table prints the token `unsure`.
- Decision 4: a deadline past 20 characters prints in exponent form. The other choice names only the cap.
- Decision 5: internal identifiers keep the old word.
- Decision 6: Part B waits for 0146.

## What the build taught us

- The audit golden exposed `Counts.unresolved` inside the serialized `suggested` and `held` objects. A serde field rename kept the internal Rust name while making every public member `unsure`. Tracing nested serialization before the edit would have found it sooner.
- J1 added `specification/types.md`, executable `spec/annotate.md`, and two digest-named local request fixtures after this ticket's first inventory. A current public-surface search and the fixture filename formula belonged in the preparation. The prototype measure fixtures remained byte copies.
- Executable pages need the built command on `PATH`. The first targeted invocation missed that setup; the corrected invocation passed. One full-demo attempt exposed an extra usage-counter warning in unchanged demo 12 under the normal home directory. Its cause is unproven; the changed pages passed targeted checks.
- Fresh review found that the first golden porter rewrote every `unresolved` string, including a user option. The correction limits shared rewriting to audit count members and the count line. The existing diff capture names its two below-cut states and summary move exactly; a compiled diff case pins an option literally named `unresolved`. Preparation missed that option/value boundary.
- The remaining limits are the internal names, historical artifacts, and marketing copies listed above. No broader mutation campaign or full port ladder was run under Ian's current focused-proof ruling.

## Closes

- Part A marks item 1 of `sdlc/issues/2026-09-25-command-wording-and-help-fixes-before-0-1.md` fixed by this ticket.
- Part B marks item 16 fixed. That closes the issue. The lander writes the closing status line and moves the file to `sdlc/issues/closed/` in Part B's landing commit. The same commit files the marketing issue that "Marketing's files" describes.
