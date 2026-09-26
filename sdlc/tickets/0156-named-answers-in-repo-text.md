---
flow: build
priority: 156
opens: sdlc/scripts/named-answers.mjs sdlc/scripts/lint demos/01-refund-gate/README.md spec/decide.md specification/channels.md specification/decide.md specification/choose.md crates/thinkthen/src/cli/args/command.rs databases/duckdb/README.md databases/postgresql/README.md databases/postgresql/check.sh databases/sqlite/README.md sdlc/records sdlc/tickets sdlc/issues
---

# 0156: The named-answers rule reaches the queue's own pages

Status: ready for review. Owner: Claude. It builds only after the module changes under "Requests to the marketing lead" land on main, and after tickets 0151, 0152 Part B and 0153 land. "Build order" gives the rule.

Review route: a fresh read-only Claude session reviews this design and the final diff. Codex does not review this ticket unless Ian routes it.

## Outcome and authority

Every code block in the pages the queue owns stores each ThinkThen answer under a name for its meaning before it uses it, as the site's samples now do. A lint step holds the rule. It imports the site's rule module and keeps no copy of it.

Ian ruled on 2026-09-26 that every code sample names its answer. `sdlc/issues/2026-09-26-site-samples-assert-on-unnamed-answers.md` records the ruling. The marketing lead fixed `site/` at `648a965f`. It then moved the rule into `site/scripts/named-answers.mjs` at `e0a6150a`, and that file's contract names "the builder's lint rung" as its second reader. `sdlc/planning/backlog-0-1-2026-09-26.md` lists the ruling's reach over `demos/` and the library READMEs as question 4 for Ian. The coordinator's brief of 2026-09-26 authorizes this ticket to extend the rule to the repository text the queue owner owns. If Ian has not ruled on question 4, this ticket waits on his answer.

## What happens today

Survey run on 2026-09-26 at `origin/main` `e0a6150a`. A throwaway script in the scratchpad, `t0156/survey.mjs`, walked every tracked `.md` file under the five folders, split out each fenced block, and handed each block whose tag the module accepts to `namedAnswerProblems`. Nothing in the tree changed.

| Folder | Pages | Code blocks | Skipped by tag | Untagged | direct | unnamed | generic | bare-exit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `demos/` | 23 | 88 | 0 | 0 | 4 | 0 | 0 | 0 |
| `specification/` | 27 | 39 | 54 | 0 | 2 | 0 | 0 | 1 |
| `spec/` | 8 | 53 | 0 | 0 | 2 | 0 | 1 | 0 |
| `libraries/` | 12 | 9 | 1 | 0 | 0 | 0 | 0 | 0 |
| `databases/` | 7 | 5 | 0 | 0 | 0 | 5 | 0 | 0 |
| Total | 77 | 194 | 55 | 0 | 8 | 5 | 1 | 1 |

Tags seen: `bash` 138, `sh` 45, `sql` 5, `python` 3, `r`, `ruby` and `ts` 1 each, `json` 41, `text` 13, `toml` 1. No block opens without a tag, and no page uses `~~~`. The `.md` files under `libraries/` and `databases/` other than the READMEs hold no fenced block.

The fifteen problems, one row each:

| # | Place | Rule | Text today | Verdict |
| --- | --- | --- | --- | --- |
| 1 | `databases/duckdb/README.md:28` | unnamed | `SELECT id FROM tickets WHERE thinkthen_decide('Does the writer ask for a refund?', body);` | Fix |
| 2 | `databases/duckdb/README.md:29` | unnamed | `SELECT id, thinkthen_choose('Which team owns this?', body, ['billing', 'shipping']) FROM tickets;` | Fix |
| 3 | `databases/duckdb/README.md:49` | unnamed | `SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM staff', [...]);` | Fix |
| 4 | `databases/postgresql/README.md:30` | unnamed | `SELECT id FROM tickets WHERE thinkthen_decide('@refund.json', body) IS NULL;` | Fix |
| 5 | `databases/sqlite/README.md:8` | unnamed | `SELECT id, body FROM reviews WHERE thinkthen_decide('Is this a complaint?', body);` | Fix |
| 6 | `demos/01-refund-gate/README.md:12` | direct | `if thinkthen decide 'Does the customer ask for money back?' \` | Fix |
| 7 | `demos/01-refund-gate/README.md:34` | direct | the same `if`, over `question.txt` | Fix |
| 8 | `demos/28-what-a-run-cost/README.md:67` | direct | `printf '%s' "$(printf '%s' "$example" \| jq -r '.body')" \` | False positive |
| 9 | `demos/41-tune-a-question-file/README.md:12` | direct | `printf '%s' "$(jq -r 'select(.id == "C-01") \| .body' claims.jsonl)" \` | False positive |
| 10 | `spec/decide.md:151` | generic | `out=$(printf '' \| thinkthen decide 'reports a payment failure' --jsonl --dry-run)` | Fix |
| 11 | `spec/decide.md:231` | direct | `test -z "$(printf 'x' \| thinkthen decide ... --url ftp://127.0.0.1/v1 2>/dev/null)"` | Fix |
| 12 | `spec/recognize.md:13` | direct | `printf 'Maria Chen ...' \| env ... thinkthen recognize ... --replay "$(git rev-parse --show-toplevel)/..."` | False positive |
| 13 | `specification/channels.md:70` | direct | `if thinkthen decide 'the customer asks for a refund' --quiet < message.txt; then` | Fix |
| 14 | `specification/decide.md:61` | direct | `if thinkthen decide 'The customer explicitly requests a refund.' --threshold 0.1:0.9 --quiet < message.txt; then` | Fix |
| 15 | `specification/decide.md:76` | bare-exit | `case $? in`, after the same command | Fix |

Rows 8, 9 and 12 are false positives. The module's Bash `direct` rule fires on a `test`, `[`, `[[`, `echo` or `printf` line that holds any `$(` when the whole statement holds a ThinkThen call. In these three lines the command substitution runs `jq` or `git`, and the `printf` feeds the call's input. No answer is read in place.

Two help samples break the rule too. They sit in doc comments, which this check does not read. `crates/thinkthen/src/cli/args/command.rs:48` reads `case $? in` in the `decide` help. `command.rs:110` names the `choose` answer `pick`, which the module lists as generic, and its exit code `rc`. `specification/decide.md:72` says the help shows the `case` block, and `specification/choose.md:53` names `$rc`. Fixing the page without the help would make the page misquote the help.

## Design

### The check

A new Node script, `sdlc/scripts/named-answers.mjs`, runs from the lint rung. It imports `namedAnswerProblems` from `../../site/scripts/named-answers.mjs` and never copies it.

1. **Pages.** `git ls-files -co --exclude-standard -- demos specification spec libraries databases` lists the pages, and the check keeps the paths that end in `.md`. `-co` takes a new page before `git add`, as `sdlc/scripts/surfaces` does. `.gitignore` rules still hold. Pages under `crates/`, `site/`, `sdlc/` and the root are not read.
2. **Fences.** A fence opens on a line of optional spaces, then three or more backticks or tildes. It closes on a line of optional spaces, then the same character repeated at least as many times, then nothing but spaces. The block is the lines between them. The tag is the first word of the opening line's info string, in lower case. The check strips no indent. The module's patterns already allow leading spaces. A fence with no close before the end of the page fails.
3. **Tags.** A block with no tag is skipped. A block tagged `text`, `console`, `json` or `toml` is skipped. These are output, transcripts and data, which a reader does not run as code. Any other tag goes to the module as its language name. The check keeps no list of language names. The module's language list is the only one. Before it hands a block over, the check asks the module about the tag with empty code. If the module throws, the block's tag names no language the rule reads, and the check reports it. This step needs the module's unknown-language throw, which the marketing lead is adding (request R0).
4. **Report.** Each problem prints one line to standard error and makes the exit code 1:
   - A module problem: ``named answers: PATH:LINE: RULE: MESSAGE``. LINE is the page line, which is the fence line plus the problem's line in the block. RULE and MESSAGE are the module's own.
   - An unknown tag: ``named answers: PATH:LINE: the rule reads no `TAG` block; tag data or output as text, console, json, or toml``. LINE is the fence line.
   - An unclosed fence: ``named answers: PATH:LINE: this fence never closes``.
   A clean run prints one line to standard output, ``named answers: N code blocks in M pages name their answers; K skipped by tag``, and exits 0.

### The self-test

`node sdlc/scripts/named-answers.mjs --self-test` runs before the real check in lint. It writes no file. It follows the `--self-test` form of `sdlc/scripts/tickets` and `children`.

- **One plant for each folder.** It lists the real pages as the check does. For each of the five folders it takes the first page in sorted order and appends one planted block to that page's text in memory. It scans the whole real set twice: once as it is, and once with the planted page. The planted run must print exactly the lines of the first run plus the one expected line, pinned whole. The plants are:

  | Folder | Tag | Planted block | Expected rule |
  | --- | --- | --- | --- |
  | `demos` | `bash` | `if thinkthen decide 'Q?' --quiet < m.txt; then echo yes; fi` | direct |
  | `specification` | `sh` | `thinkthen decide 'Q?' --quiet < m.txt`, then `case $? in` | bare-exit |
  | `spec` | `bash` | `answer=$(thinkthen decide 'Q?' < m.txt)` | generic |
  | `libraries` | `python` | `assert tt.decide(question, message)` | direct |
  | `databases` | `sql` | `SELECT id FROM t WHERE thinkthen_decide('Q?', body);` | unnamed |

- **Fence and tag rows.** One small page in memory for each row, scanned alone, with its whole output pinned. Each row holds the Bash plant `answer=$(thinkthen decide 'Q?' < m.txt)`:

  | Row | Opening and closing | Expected |
  | --- | --- | --- |
  | skip tag | `` ```text `` and `` ``` `` | no line |
  | no tag | `` ``` `` and `` ``` `` | no line |
  | tildes | `~~~bash` and `~~~` | the generic line |
  | long fence | ` ````bash `, a line of `` ``` `` before the plant, and ` ```` ` | the generic line at the plant, so the short line did not close the block |
  | indented | `   ```bash` inside a list item, and `   ``` ` | the generic line |
  | unknown tag | `` ```yaml `` and `` ``` `` | the unknown-tag line |
  | unclosed | `` ```bash `` and no close | the unclosed line |

### The lint rung

`sdlc/scripts/lint` gains three lines after `python3 sdlc/scripts/tickets`:

```sh
# Ticket 0156: Ian's named-answers rule over the queue's own pages.
node sdlc/scripts/named-answers.mjs --self-test
node sdlc/scripts/named-answers.mjs
```

The step sits before `cargo`, so a planted page fails lint in seconds.

### The fixes

Each fix keeps the page's lesson and its commands. Only the names change.

- **Demo 01, rows 6 and 7.** The exit code is the lesson, so the call goes into a function named for its meaning, as `site/WRITING.md` allows. Each block defines it, because `mustmatch test` runs each block as its own test:

  ```bash
  set -euo pipefail

  asks_for_money_back() {
    thinkthen decide 'Does the customer ask for money back?' \
      --quiet --replay recording/
  }

  if asks_for_money_back < message.txt
  then
  ```

  The rest of each block is unchanged. The second block reads `question.txt`.
- **`specification/channels.md`, row 13.** The same form: `asks_for_refund() { thinkthen decide 'the customer asks for a refund' --quiet; }` on three lines, then `if asks_for_refund < message.txt; then`.
- **`specification/decide.md`, row 14.** The same form, with `requests_refund` and `--threshold 0.1:0.9`.
- **`specification/decide.md`, row 15, and the `decide` help.** The command line ends `&& refund_code=0 || refund_code=$?`, and the block reads `case $refund_code in`. This form also survives `set -e`, which the help's own warning asks for. The help's two lines at `command.rs:46` and `:48` change the same way, so the page still quotes the help. The prose at `decide.md:72` says "a `case` block on the named exit code".
- **The `choose` help.** `command.rs:110` to `:114` names the answer `team` and the exit code `team_code`: `team=$(thinkthen choose ...) && team_code=0 || team_code=$?`, then `case $team_code in 0) ;; 3) team=not_sure ;; *) exit "$team_code" ;; esac`, then `case $team in ...`. `specification/choose.md:53` says the first `case` reads `$team_code`.
- **`spec/decide.md`, row 10.** `no_records_output=$(...)`, then `test -z "$no_records_output"`.
- **`spec/decide.md`, row 11.** The block names the output and the exit code, and pins the exit code it captured, as `CLAUDE.md` asks of a block that catches a failure:

  ```bash
  refusal_code=0
  refusal_stdout=$(printf 'x' | thinkthen decide 'asks for a refund' --dry-run --url ftp://127.0.0.1/v1 2>/dev/null) || refusal_code=$?
  test -z "$refusal_stdout"
  echo "$refusal_code" | mustmatch "2"
  ```

  `specification/backends.md:51` fixes exit 2 for a refused scheme. The block's second line, the stderr check, stays.
- **SQL, rows 1 to 5.** The call takes an alias named for its meaning, and a filter reads the alias from a subquery, as the site's `filter/postgresql.sql` does. The outer query selects only the id, so each row calls the function once.
  - DuckDB: `SELECT id FROM (SELECT id, thinkthen_decide('Does the writer ask for a refund?', body) AS asks_refund FROM tickets) WHERE asks_refund;` on three lines. `thinkthen_choose(...) AS team`. `thinkthen_relate(...) AS works_for`, as the site's relate sample aliases its table.
  - PostgreSQL: `SELECT id FROM (SELECT id, thinkthen_decide('@refund.json', body) AS asks_refund FROM tickets) AS judged WHERE asks_refund IS NULL;` on three lines. PostgreSQL before 16 needs the subquery alias. `check.sh`'s `slide_sample` runs the same statement, so the README's claim that the sample runs as drawn stays true. Its expected output does not change. The sentence before the block says `check.sh` runs it, in place of "The slide sample", because the deck lives in another repository.
  - SQLite: `SELECT id, body FROM (SELECT id, body, thinkthen_decide('Is this a complaint?', body) AS is_complaint FROM reviews) WHERE is_complaint;` on three lines.
- **Rows 8, 9 and 12.** No change, once request R1 lands. If the coordinator rules that the build goes ahead without R1, the fallback rewrites each line so no `$(` shares the line with the pipe into the call. Demo 28 pipes `printf '%s' "$example" | jq -j '.body'` into the call. Demo 41 pipes `jq -j 'select(.id == "C-01") | .body' claims.jsonl` into it. `jq -j` prints no newline, and neither C-01 body ends in one (checked with `od -c` in the survey), so the bytes sent match the recordings. `spec/recognize.md` names the recording first: `recording="$(git rev-parse --show-toplevel)/crates/thinkthen/tests/fixtures/recognize-225/C01"`, then `--replay "$recording"`.

## Requests to the marketing lead

The marketing lead owns the module. This ticket edits no file under `site/`. The coordinator relays these two.

- **R0, in progress.** `namedAnswerProblems` throws on a language it does not accept, in place of returning an empty array. The check's unknown-tag step depends on it. The contract comment and the `unknown` rows of `named-answers.test.mjs` change with it.
- **R1, a false positive.** The Bash `direct` rule for a `test`, `[`, `[[`, `echo` or `printf` line tests the whole statement for a call. It should test only the text inside the line's `$(...)`. Three correct lines fail today: rows 8, 9 and 12 above. Suggested rows for the table test:
  - pass: `printf '%s' "$(jq -r .body m.jsonl)" | thinkthen decide "$q"`
  - pass: `printf 'x' | thinkthen recognize --replay "$(git rev-parse --show-toplevel)/r"`
  - fail, unchanged: `echo "$(thinkthen decide "$q" < mail.txt)"`

An observation, not a request: the generic list lacks `rc`, `status` and `code`, and the form `&& rc=0 || rc=$?` never reaches the generic check, since the name does not open the line. Nine demo lines use `rc` and five `spec/` lines use `code=$?`. Ian's ruling asks for a name of meaning. "Deferred gaps" carries it.

## Decisions

The agent decided each of these within the coordinator's brief. Ian can overturn any of them.

1. **The lint rung.** The check reads text and needs no binary. Lint already runs the other text checks: `tickets`, `children`, `pages` and the ratchet. `spec` builds and runs the binary first, so a bad page would wait for a build. The site lead runs lint before landing too, so a module change that breaks this check shows up in that run.
2. **Every tracked `.md` under the five folders.** The brief names `demos/`, `specification/`, `spec/`, and the library and database READMEs. Every other `.md` under `libraries/` and `databases/` holds no fenced block today, so one folder rule reads the same blocks and never misses a new page. The root `README.md`, `transforms/`, `conformance/`, `probes/` and `profiles/` hold no problem today and stay out, per the brief.
3. **Untagged blocks are skipped.** The brief rules it. None exists today.
4. **Four skip tags: `text`, `console`, `json`, `toml`.** Three appear today, and the brief names `console`. Any other tag the module refuses fails the check, so a new tag gets a decision, never a silent skip.
5. **No exception mechanism.** After the fixes and R1, no block needs one. A block that shows output or data takes a skip tag. A per-block marker would be a second way to skip, and none is needed.
6. **A function or a named exit code, never a bare call in `if`.** Where the exit code is the lesson, the page keeps it and names it, as the ruling says. A function keeps `if` as the lesson of demo 01 and `channels.md`. `refund_code` keeps the four-way `case` of `decide.md`.
7. **The help samples change with the pages.** `specification/decide.md` quotes the help's `case` block, and `choose.md` names the help's `$rc`. Changing the pages alone would break those quotes. The help reaches more readers than any page, and the ruling covers it. The check does not read doc comments. "Deferred gaps" carries that.
8. **Wait for R1 instead of rewriting correct lines.** The three lines are right under the ruling. Rewriting them to dodge a false positive would hide the module bug from the site too. The fallback stands ready if the coordinator rules otherwise.
9. **SQL filters read the alias from a subquery.** PostgreSQL cannot read a select alias in `WHERE`. DuckDB and SQLite can, but a select list that keeps the alias beside a `WHERE` on it may call the function twice. One subquery form serves all three and matches the site.
10. **The check pins its own unknown-tag sentence.** It probes the module with empty code and never prints the module's error. A reworded throw on the site then cannot break the lint rung.

## Edge cases

| Input | Result |
| --- | --- |
| A `bash`, `sh`, `python`, `typescript`, `ts`, `ruby`, `r`, `rust`, `c` or `sql` block that names its answers | No line |
| The same block with an unnamed answer | `named answers: PATH:LINE: RULE: MESSAGE`, exit 1 |
| Tag in mixed case, such as `Bash` or `SQL` | Read in lower case, then as above |
| Info string with more words, such as `bash title=x` | The first word is the tag |
| `text`, `console`, `json` or `toml` block holding a call | Skipped |
| Block with no tag holding a call | Skipped |
| `yaml`, `py` or any tag the module refuses | The unknown-tag line at the fence, exit 1 |
| `~~~bash` fence | Read |
| ` ````bash ` fence holding a ` ``` ` line | The inner line does not close it. Read to the ` ```` ` |
| Fence indented inside a list item | Read, indent kept |
| A fence left open at the end of the page | The unclosed line, exit 1 |
| An untracked page under a scanned folder, not ignored | Read |
| An ignored page | Not read |
| A page under `crates/`, `site/`, `sdlc/` or the root | Not read |
| A `.md` under `libraries/` that is not a README | Read. None holds a block today |
| Three problems on one page | Three lines, in page order, one exit 1 |
| Clean tree | One summary line on standard output, exit 0 |
| A demo block whose function wraps `thinkthen decide` | No line. The call is not in the `if` |
| `refund_code=$?` after the call | No line. `case $refund_code` reads a name |
| SQL call in a subquery with `AS asks_refund`, filtered outside | No line |

## Proof

Each test runs through the real boundary, the script over the real pages.

| Test | What it checks | Planted fault that turns it red |
| --- | --- | --- |
| `named-answers.mjs --self-test`, folder plants | Each of the five folders is walked, and each language reaches the module | P1: drop `spec` from the folder list. The `spec` plant prints nothing. P2: add `sh` to the skip tags. The `specification` plant prints nothing |
| `named-answers.mjs --self-test`, fence rows | The fence and tag rules in "The self-test" | P3: open fences on backticks only. The tildes row prints nothing. P4: close a fence on any three backticks. The long-fence row prints the wrong line. P5: skip a tag the module refuses. The unknown-tag row prints nothing. P6: drop the end-of-page check. The unclosed row prints nothing |
| `named-answers.mjs` over the real pages | The fixed pages keep the rule | P7: in the lane, add one planted block to one real page in each folder, run the check, and record each of the five lines and exit 1. Restore and touch each page |
| `sdlc/scripts/lint` | The rung runs the check | P8: plant one block in `demos/01-refund-gate/README.md` and run `lint`. It stops at the named-answers step with exit 1 before `cargo`. Restore and touch |

The fixed pages keep their own proof. `spec` runs every `spec/` page and green demo, so demo 01 and `spec/decide.md` must pass with the new names. `databases/postgresql/check.sh` runs the new slide statement. The `test` rung covers the help change.

The four questions for the one new test, the self-test:

- **What behavior does it protect?** It protects the walk of each folder, the tag skip list, and the fence grammar.
- **What credible regression fails it?** A dropped folder, a lost `sh`, or a fence rule that swallows or splits a block. The real run on a clean tree stays green under each of them.
- **Why does no existing test catch it?** `site/scripts/named-answers.test.mjs` tests the module's rule on one block. Nothing tests the walk, the fences or the tags.
- **Does it need a test-only hook?** The `--self-test` flag, as `tickets` and `children` use. A clean real tree cannot show that a folder is read, so a plant must go in. The self-test plants into the real pages' text in memory and writes nothing.

## Budgets

Nonblank lines, counted with `grep -c .` over the diff.

- `sdlc/scripts/named-answers.mjs`: at most 120, self-test included.
- `sdlc/scripts/lint`: at most 3 added.
- The pages and `check.sh`: at most 30 added, net, together.
- `crates/thinkthen/src/cli/args/command.rs`: 0 net. The help lines change in place. `sdlc/ratchet.json` does not move.
- No dependency. No change under `site/`.

## Stop rules

1. Stop before starting if R0 has not landed on main.
2. Stop before starting if R1 has not landed on main, unless the coordinator rules the fallback.
3. Stop before starting if 0151, 0152 Part B or 0153 has not landed, or if a later ticket opens a file this one needs. "Build order" names them.
4. Stop if the survey on the merged main finds a problem outside the table in a page another unlanded ticket opens. Fix a new problem elsewhere in the same style, and list it in the record.
5. Stop before editing any file under `site/`.
6. Stop if any plant stays green.
7. Stop before crossing a budget.
8. Stop if a demo or `spec/` page turns red after its rewrite, and the fault is more than a typo in the rewrite.
9. Stop if an existing test pins a help line this ticket changes. None does today, by `grep` over `crates/thinkthen/tests`, `spec/` and `site/`.

## Scope and exclusions

Excluded: the module and every file under `site/`, which the marketing lead owns. The `examples.json` runner fixtures. Rust doc comments beyond the two help samples. `sdlc/` history, which records what was. `crates/thinkthen/tests/fixtures/`, whose demo pages are test inputs.

## Build order

Every file this ticket edits, and the tickets in flight that open it:

| File | Shared with | What the other ticket does there |
| --- | --- | --- |
| `demos/01-refund-gate/README.md` | 0146 (all of `demos/`), 0152 Part B | 0152 swaps "unresolved" for the not-sure word. 0146's `opens` names the folder |
| `spec/decide.md` | 0146 | Batching lines |
| `specification/channels.md` | 0146, 0147, 0152 Part B, 0153 | Batching, recognize, the word, and two hint sentences |
| `specification/decide.md` | 0146, 0152 Part B, 0154 | Batching, the word, and the ceiling |
| `specification/choose.md` | 0152 Part B | The word, on line 53 itself |
| `crates/thinkthen/src/cli/args/command.rs` | 0153 | One sentence in the `choose` doc comment, beside the sample |
| `databases/duckdb/README.md` | 0151 (built on its branch), 0148 (the folder) | The details line. Its branch leaves the three SQL lines as they are |
| `databases/postgresql/README.md`, `check.sh` | 0151 (the README), 0148 (the folder) | Details and settings |
| `databases/sqlite/README.md` | 0151 | The details line |
| `sdlc/scripts/lint` | none | |
| `sdlc/records`, `sdlc/tickets`, `sdlc/issues` | every ticket | New files and one move |

0150 touches none of these files. Demos 28 and 41 and `spec/recognize.md` change only under the R1 fallback, and 0152 Part B opens demo 41.

Order: 0156 builds after 0152 Part B lands, as the brief prefers, because Part B rewords the same pages. 0152 Part B waits for 0146, and 0153 waits for 0146, so 0146 lands before this ticket. 0151 is built on its branch and lands first. 0153 lands first, so the `choose` doc comment merges once. 0147, 0148 and 0154 may land either side. Whichever lands later merges `origin/main`, and every shared line here is a word or a line swap. The builder merges `origin/main` and reruns the survey before the first edit and again before the final run.

## Routing

Builder: Claude (Opus subagent), in the lane the coordinator names. Reviewer: a fresh read-only Claude session for design and for code. The coordinator relays R0 and R1 to the marketing lead.

## Complexity

Contract 1; state and timing 0; reach 2; proof 1; cost of error 1; total 5. Final level: 1. The reach is many shared pages. The main risk is a check that walks too little, which the folder plants guard.

## Deferred gaps

- The `examples.json` fixtures the surface runners compare against printed output. PostgreSQL has 9 of 12 examples with problems, SQLite 7 of 11, and R 10 of 10. The Python, Ruby and TypeScript files have none. Each example is one expression that the runner prints, not a sample a reader copies. `databases/postgresql/tests/examples.py` says the site draws its SQL tab from that file, but no file under `site/` names it today.
- Rust doc comments. The help samples change by hand, and no check reads them.
- The root `README.md`, `transforms/`, `conformance/`, `probes/` and `profiles/`. None has a problem today.
- Exit codes named `rc`, `status` or `code`, and the `&& rc=0 || rc=$?` form. The module passes both. Nine demo lines and five `spec/` lines use them.
- An untagged block is never read. None exists today.
- A writer can tag code as `text` to skip it. Review is the guard.
- Single-letter aliases such as the PostgreSQL README's `AS a` pass the module.

## What Ian can overturn

- The reach itself, backlog question 4.
- Decision 1: the lint rung.
- Decision 2: every `.md` under the five folders, and none elsewhere.
- Decisions 3 and 4: skip untagged blocks and four tags, and fail every other tag the module refuses.
- Decision 5: no exception mechanism.
- Decision 7: the help samples change in this ticket.
- Decision 8: wait for R1 instead of rewriting three correct lines.
- Each new name, such as `asks_for_money_back`, `refund_code` and `team`.

## Closes

- `sdlc/issues/2026-09-26-site-samples-assert-on-unnamed-answers.md`. The lander closes it with the landing commit. The status line names site commit `648a965f` for the site and this ticket for the queue's pages. It moves to `closed/`.
- Item 2 of `sdlc/issues/2026-09-26-three-items-from-the-named-answers-site-landing.md`. The lander marks it done with the landing commit. The issue stays open for items 1 and 3.

## Evidence

- Starts from: Ian's ruling of 2026-09-26 in the site issue. Site commit `648a965f`, which fixed `site/`. Module commit `e0a6150a` and its 46-row table test. `site/WRITING.md`'s paragraph on named answers, which allows a function named for its meaning. The survey on 2026-09-26 at `e0a6150a`: 194 code blocks in 77 pages, 15 problems, 3 of them false positives. The two help samples at `command.rs:48` and `:110`. The site's `filter/postgresql.sql` and `relate/duckdb.sql` for the SQL shapes. `specification/backends.md:51` for exit 2. `od -c` over the two C-01 bodies for the R1 fallback.
- Keeps: Every demo's and spec page's commands, inputs, recordings and assertions. Each page's lesson: `if` on an exit code in demo 01 and `channels.md`, the four-way `case` in `decide.md`. The PostgreSQL slide check's expected output. The module, byte for byte.
- Changes: A lint step runs the site's rule over every code block under the five folders, with a self-test. Twelve lines across nine pages name their answers. The `decide` and `choose` help samples name their answer and exit code, and two spec sentences follow them. `check.sh` runs the new PostgreSQL statement.
- Proof: The self-test's five folder plants and seven fence rows, with planted faults P1 to P6. P7's five real-page plants and P8's lint plant, recorded red. The `spec` rung over the rewritten pages, the `test` rung over the help, and the PostgreSQL surface check.
- Defers: The `examples.json` fixtures, Rust doc comments, the root README and the other folders, `rc`-style exit-code names, untagged blocks, and single-letter aliases.
