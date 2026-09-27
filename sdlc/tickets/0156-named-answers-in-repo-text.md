---
flow: build
priority: 156
opens: sdlc/scripts/named-answers.mjs sdlc/scripts/lint demos/01-refund-gate/README.md spec/decide.md specification/channels.md specification/decide.md specification/choose.md specification/score.md crates/thinkthen/src/cli/args/command.rs databases/duckdb/README.md databases/postgresql/README.md databases/postgresql/check.sh databases/sqlite/README.md sdlc/planning/backlog-0-1-2026-09-26.md sdlc/records sdlc/tickets sdlc/issues
---

# 0156: The named-answers rule reaches the queue's own pages

Status: done. Fresh independent Codex review accepted `e2f9fa58`, and the merged focused checks passed. The coordinator accepted it on 2026-09-26 after a fresh read-only review. Owner: Codex in lane codex-5. The site module changes R0/R1/R2 and tickets 0151, 0152 Part B and 0153 are on main.

Review route: the coordinator assigns a fresh independent Codex reviewer to the frozen final diff. The accepted design review remains on record.

## Outcome and authority

Every code block in the pages the queue owns stores each ThinkThen answer under a name for its meaning before it uses it, as the site's samples now do. A lint step holds the rule. It imports the site's rule module and keeps no copy of it.

Ian ruled on 2026-09-26 that every code sample names its answer. `sdlc/issues/2026-09-26-site-samples-assert-on-unnamed-answers.md` records the ruling. The marketing lead fixed `site/` at `648a965f`. It then moved the rule into `site/scripts/named-answers.mjs` at `e0a6150a`, and that file's contract names "the builder's lint rung" as its second reader. The authority for this ticket is the coordinator's ruling of 2026-09-26. Under it, the named-answers rule reaches `demos/`, the library and database READMEs, `specification/` and `spec/`. Ian can overturn it. Ian later ruled "make sure all config documented" and gave the coordinator the full queue, and he has not ruled against this reach. The ruling answers question 4 of `sdlc/planning/backlog-0-1-2026-09-26.md`, which asked whether the code-sample ruling reaches `demos/` and the library READMEs, and so moves that question off Ian's list. The ruling can be reversed cheaply, because it only adds a text check and renames variables in pages. The lander marks question 4 answered in that file, citing the coordinator's ruling and this ticket.

## What happens today

The original survey ran on 2026-09-26 at `origin/main` `e0a6150a`. A throwaway script in the scratchpad, `t0156/survey.mjs`, walked every tracked `.md` file under the five folders, split out each fenced block, and handed each block whose tag the module accepts to `namedAnswerProblems`. Nothing in the tree changed. The rows below preserve that preparation evidence.

Current-source preflight on 2026-09-27 merged `origin/main` `65aede94` before editing. The site's module accepts R0/R1/R2, including unknown-language refusal, the corrected `printf` pipeline handling, and `case` substitutions. The first run of the new scanner found 14 problems in the claimed pages: five unnamed SQL calls, two demo branches, two `spec/decide` lines, one `channels` branch, one `decide` branch and one bare exit, and two `case` substitutions. It saw 82 pages, 210 accepted-language blocks, and 57 skipped blocks. The older 15-problem table is a historical snapshot, not the current failure count.

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

Two more samples read an answer in place, and the module does not flag them today. `specification/choose.md:85` reads `case "$(thinkthen choose 'Which team owns this request?' ... --raw < message.txt)" in`, and `specification/score.md:77` reads `case "$(thinkthen choose 'How much disruption does this report?' ... --raw < ticket.txt)" in`. Request R2, which the marketing lead accepted, makes the module flag this form.

Two help samples break the rule too. They sit in doc comments, which this check does not read. `crates/thinkthen/src/cli/args/command.rs:48` reads `case $? in` in the `decide` help. `command.rs:110` names the `choose` answer `pick`, which the module lists as generic, and its exit code `rc`. `specification/decide.md:72` says the help shows the `case` block. Fixing the page without the help would make the page misquote the help.

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

- **One plant for each folder.** The self-test holds its own literal list of the five plant folders, separate from the list the check walks. It lists the real pages through the check's own walk. For each plant folder it takes the first walked page whose path starts with the folder name plus `/`, so `spec/` never matches `specification/`. A plant folder that yields no page fails the self-test with ``named answers: self-test: no page under FOLDER/``. It then takes that page and appends one planted block to that page's text in memory. It scans the whole real set twice: once as it is, and once with the planted page. The planted run must print exactly the lines of the first run plus one new line. That line must start with ``named answers: PATH:LINE: RULE: `` for the planted page, the plant's line and the expected rule, and a non-empty message must follow. The self-test never pins the module's message text. The plants are:

  | Folder | Tag | Planted block | Expected rule |
  | --- | --- | --- | --- |
  | `demos` | `bash` | `if thinkthen decide 'Q?' --quiet < m.txt; then echo yes; fi` | direct |
  | `specification` | `sh` | `thinkthen decide 'Q?' --quiet < m.txt`, then `case $? in` | bare-exit |
  | `spec` | `bash` | `answer=$(thinkthen decide 'Q?' < m.txt)` | generic |
  | `libraries` | `python` | `assert tt.decide(question, message)` | direct |
  | `databases` | `sql` | `SELECT id FROM t WHERE thinkthen_decide('Q?', body);` | unnamed |

- **Fence and tag rows.** One small page in memory for each row, scanned alone. A module problem is pinned as ``named answers: PATH:LINE: RULE: `` plus a non-empty message. The check's own sentences, for an unknown tag and an unclosed fence, are pinned whole. Each row holds the Bash plant `answer=$(thinkthen decide 'Q?' < m.txt)`:

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

`sdlc/scripts/lint` gains two commands after the page checks:

```sh
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
- **The `choose` help.** `command.rs:110` to `:114` names the answer `team` and the exit code `team_code`: `team=$(thinkthen choose ...) && team_code=0 || team_code=$?`, then `case $team_code in 0) ;; 3) team=not_sure ;; *) exit "$team_code" ;; esac`, then `case $team in ...`.
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
  - PostgreSQL's annotate line renames the alias `a` to `triage`: `SELECT id, triage->>'team', (triage->>'urgency')::float AS urgency FROM tickets, thinkthen_annotate('@form.json', body) AS triage ORDER BY urgency DESC;`. `check.sh:145` changes the same way and keeps its `, id` tie-break. Its expected output does not change.
  - SQLite: `SELECT id, body FROM (SELECT id, body, thinkthen_decide('Is this a complaint?', body) AS is_complaint FROM reviews) WHERE is_complaint;` on three lines.
- **`specification/choose.md:85` and `specification/score.md:77`.** Each names the answer first, then branches on the name. The branches do not change:

  ```sh
  team=$(thinkthen choose 'Which team owns this request?' billing shipping account --threshold 0.8 --raw < message.txt)
  case $team in
  ```

  ```sh
  disruption=$(thinkthen choose 'How much disruption does this report?' none workaround blocked --raw < ticket.txt)
  case $disruption in
  ```

  These two rewrites happen either way, whatever R2 does.
- **Rows 8, 9 and 12.** No change. The marketing lead accepted R1, and the build does not rewrite `demos/28:67`, `demos/41:12` or `spec/recognize.md:13`.

## Prerequisites from the marketing lead (landed)

The marketing lead owns the module. R0/R1/R2 are on main at the current-source preflight. This ticket edits no file under `site/`. The requests below record why the check can import it.

- **R0, landed.** `namedAnswerProblems` throws on a language it does not accept, in place of returning an empty array. The check's unknown-tag step depends on it. The contract comment and the `unknown` rows of `named-answers.test.mjs` changed with it.
- **R1, landed.** The marketing lead's `direct` rule flags `printf`, `echo` and `test` only when a ThinkThen call sits inside the `$(...)` they read. The old Bash rule tested the whole statement for a call. Three correct lines formerly failed: rows 8, 9 and 12 above. The module's table now includes these cases:
  - pass: `printf '%s' "$(jq -r .body m.jsonl)" | thinkthen decide "$q"`
  - pass: `printf 'x' | thinkthen recognize --replay "$(git rev-parse --show-toplevel)/r"`
  - fail, unchanged: `echo "$(thinkthen decide "$q" < mail.txt)"`
- **R2, landed.** A `case` whose word is a `$(...)` holding a ThinkThen call becomes a `direct` problem. The historical survey above names the two pages. This ticket rewrites both.

An observation, not a request: the generic list lacks `rc`, `status` and `code`, and the form `&& rc=0 || rc=$?` never reaches the generic check, since the name does not open the line. Nine demo lines use `rc` and five `spec/` lines use `code=$?`. Ian's ruling asks for a name of meaning. "Deferred gaps" carries it.

## Decisions

The agent decided each of these within the coordinator's brief. Ian can overturn any of them.

1. **The lint rung.** The check reads text and needs no binary. Lint already runs the other text checks: `tickets`, `children`, `pages` and the ratchet. `spec` builds and runs the binary first, so a bad page would wait for a build. The site lead runs lint before landing too, so a module change that breaks this check shows up in that run.
2. **Every tracked `.md` under the five folders.** The brief names `demos/`, `specification/`, `spec/`, and the library and database READMEs. Every other `.md` under `libraries/` and `databases/` holds no fenced block today, so one folder rule reads the same blocks and never misses a new page. The root `README.md`, `transforms/`, `conformance/`, `probes/` and `profiles/` hold no problem today and stay out, per the brief.
3. **Untagged blocks are skipped.** The brief rules it. None exists today.
4. **Four skip tags: `text`, `console`, `json`, `toml`.** Three appear today, and the brief names `console`. Any other tag the module refuses fails the check, so a new tag gets a decision, never a silent skip.
5. **No exception mechanism.** After the fixes and R1, no block needs one. A block that shows output or data takes a skip tag. A per-block marker would be a second way to skip, and none is needed.
6. **A function or a named exit code, never a bare call in `if`.** Where the exit code is the lesson, the page keeps it and names it, as the ruling says. A function keeps `if` as the lesson of demo 01 and `channels.md`. `refund_code` keeps the four-way `case` of `decide.md`.
7. **The help samples change with the pages.** `specification/decide.md` quotes the help's `case` block. Changing the page alone would break that quote. The `choose` help changes for the same ruling, and no page quotes its names. The help reaches more readers than any page, and the ruling covers it. The check does not read doc comments. "Deferred gaps" carries that.
8. **Wait for R1 instead of rewriting correct lines.** The three lines are right under the ruling. Rewriting them to dodge a false positive would hide the module bug from the site too. The marketing lead accepted R1.
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

The accepted proof plan below names fault plants. The build exercised the five folder plants and seven fence rows in memory over the real page walk, plus the initial 14-problem red run. P7 and P8 remain proposed fault injections; the current focused-batch policy does not require redundant real-file plants after the red run and self-test.

| Test | What it checks | Planted fault that turns it red |
| --- | --- | --- |
| `named-answers.mjs --self-test`, folder plants | Each of the five folders is walked, and each language reaches the module | P1: drop `spec` from the list the check walks. The self-test's own list still names `spec`, finds no page under `spec/`, and fails. P2: add `sh` to the skip tags. The `specification` plant prints nothing |
| `named-answers.mjs --self-test`, fence rows | The fence and tag rules in "The self-test" | P3: open fences on backticks only. The tildes row prints nothing. P4: close a fence on any three backticks. The long-fence row prints the wrong line. P5: skip a tag the module refuses. The unknown-tag row prints nothing. P6: drop the end-of-page check. The unclosed row prints nothing |
| `named-answers.mjs` over the real pages | The fixed pages keep the rule | P7, optional: plant one block in each real folder and observe each refusal |
| `sdlc/scripts/lint` | The rung runs the check | P8, optional: plant one block in demo 01 and observe lint stop before `cargo` |

The fixed pages keep their own proof. The builder ran `mustmatch test spec/decide.md` and demo 01 against the newly built command, and inspected the generated `decide` and `choose` help for the changed names. The PostgreSQL `slide_sample` check runs the new statement. The current focused-batch policy in `sdlc/planning/work-plan-2026-09-27.md` supersedes the original full-rung-per-ticket plan: the coordinator owns the next full integration checkpoint after independent review.

The four questions for the one new test, the self-test:

- **What behavior does it protect?** It protects the walk of each folder, the tag skip list, and the fence grammar.
- **What credible regression fails it?** A dropped folder, a lost `sh`, or a fence rule that swallows or splits a block. The real run on a clean tree stays green under each of them.
- **Why does no existing test catch it?** `site/scripts/named-answers.test.mjs` tests the module's rule on one block. Nothing tests the walk, the fences or the tags.
- **Does it need a test-only hook?** The `--self-test` flag, as `tickets` and `children` use. A clean real tree cannot show that a folder is read, so a plant must go in. The self-test plants into the real pages' text in memory and writes nothing.

## Budgets

Nonblank lines, counted with `grep -c .` over the diff.

- `sdlc/scripts/named-answers.mjs`: at most 120, self-test included. The build measures 120 nonblank lines.
- `sdlc/scripts/lint`: at most 3 added. The build adds 2.
- The pages and `check.sh`: at most 32 added, net, together. The build measures 25 net added lines.
- `crates/thinkthen/src/cli/args/command.rs`: 0 net. The help lines change in place. `sdlc/ratchet.json` does not move. The aggregate ratchet measures 76,686/76,686; that is a separate ceiling from these file and group estimates.
- No dependency. No change under `site/`.

## Stop rules

1. Stop before starting if R0 has not landed on main.
2. Stop before starting if R1 has not landed on main.
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
| `specification/choose.md` | 0152 Part B | The word, at lines 19 to 76. This ticket edits only the block at line 85 |
| `specification/score.md` | none | |
| `crates/thinkthen/src/cli/args/command.rs` | 0153 | One sentence in the `choose` doc comment, beside the sample |
| `databases/duckdb/README.md` | 0151 (built on its branch), 0148 (the folder) | The details line. Its branch leaves the three SQL lines as they are |
| `databases/postgresql/README.md`, `check.sh` | 0151 (the README), 0148 (the folder) | Details and settings |
| `databases/sqlite/README.md` | 0151, 0148 (the folder) | The details line, and settings |
| `sdlc/scripts/lint` | none | |
| `sdlc/records`, `sdlc/tickets`, `sdlc/issues` | every ticket | New files and one move |

0150 touches none of these files. Demos 28 and 41 and `spec/recognize.md` do not change.

At the `65aede94` preflight, 0146, 0151, 0152 Part B, 0153 and the site module prerequisites were on main. Ticket 0154 uses `args.rs`, while this ticket changes `args/command.rs`; its shared specification pages merge second. The builder merged `origin/main` and reran the survey before editing. The coordinator handles later shared-page merges at landing.

## Routing

Builder: Codex in codex-5. Reviewer: a fresh independent Codex session assigned by the coordinator. R0/R1/R2 landed in the marketing-owned module before this build.

## Complexity

Contract 1; state and timing 0; reach 2; proof 1; cost of error 1; total 5. Final level: 1. The reach is many shared pages. The main risk is a check that walks too little, which the folder plants guard.

## Deferred gaps

- The `examples.json` fixtures the surface runners compare against printed output. PostgreSQL has 9 of 12 examples with problems, SQLite 7 of 11, and R 10 of 10. The Python, Ruby and TypeScript files have none. Each example is one expression that the runner prints, not a sample a reader copies. `databases/postgresql/tests/examples.py` says the site draws its SQL tab from that file, but no file under `site/` names it today.
- Rust doc comments. The help samples change by hand, and no check reads them.
- The root `README.md`, `transforms/`, `conformance/`, `probes/` and `profiles/`. None has a problem today.
- Exit codes named `rc`, `status` or `code`, and the `&& rc=0 || rc=$?` form. The module passes both. Nine demo lines and five `spec/` lines use them.
- An untagged block is never read. None exists today.
- A writer can tag code as `text` to skip it. Review is the guard.
- Single-letter aliases pass the module. The PostgreSQL README's `a` alias was replaced with `triage` in this build; other occurrences remain outside this ticket.

## What Ian can overturn

- The reach itself: the coordinator's ruling of 2026-09-26, which answers backlog question 4.
- Decision 1: the lint rung.
- Decision 2: every `.md` under the five folders, and none elsewhere.
- Decisions 3 and 4: skip untagged blocks and four tags, and fail every other tag the module refuses.
- Decision 5: no exception mechanism.
- Decision 7: the help samples change in this ticket.
- Decision 8: wait for R1 instead of rewriting three correct lines.
- Each new name, such as `asks_for_money_back`, `refund_code` and `team`.

## Closes

- The queue-owned page gap under Ian's named-answer ruling. The site issue was already closed by Quick Fix qf-config-ruby-issue-status for its marketing-owned scope; this ticket completes the separate queue-owned reach.
- No item in `sdlc/issues/closed/2026-09-26-three-items-from-the-named-answers-site-landing.md` remains open. Item 2 was the issue move, already done by that Quick Fix; this build does not reopen it.

## Evidence

- Starts from: Ian's ruling of 2026-09-26 in the closed site issue. Site commit `648a965f`, which fixed `site/`. Module commit `e0a6150a` and its later R0/R1/R2 changes on main. `site/WRITING.md`'s paragraph on named answers, which allows a function named for its meaning. The historical survey at `e0a6150a`: 194 code blocks in 77 pages, 15 problems, 3 of them false positives. The refreshed preflight at `65aede94`: 82 pages, 210 accepted-language blocks, 57 skipped, 14 problems in claimed pages. The two help samples in `command.rs`. `specification/backends.md` for exit 2.
- Keeps: Every demo's and spec page's commands, inputs, recordings and assertions. Each page's lesson: `if` on an exit code in demo 01 and `channels.md`, the four-way `case` in `decide.md`. The PostgreSQL slide check's expected output. The module, byte for byte.
- Changes: A lint step runs the site's rule over every accepted-language code block under the five folders, with a self-test. The 14 current-page failures now name their answers. The PostgreSQL annotate alias becomes `triage`. The `decide` and `choose` help samples name their answer and exit code, and the specification follows them. `check.sh` runs the new PostgreSQL statement.
- Proof: The first scanner run failed on 14 existing samples, then the self-test's five in-memory folder plants and seven fence rows passed with the real pages clean. Focused `mustmatch` runs passed 24 `spec/decide` blocks and 3 demo 01 blocks with 1 skipped. The built command's help shows `refund_code`, `team_code`, and `team`. The PostgreSQL slide check result is recorded in the build record. Full batch rungs remain at the coordinator's checkpoint under the current focused-batch ruling.
- Defers: The `examples.json` fixtures, Rust doc comments beyond the two changed help samples, the root README and the other folders, `rc`-style exit-code names, untagged blocks, and remaining single-letter aliases. The two closed site issues are already resolved on main and are not deferred by this ticket.

## What the build taught us

- The [build record](../records/2026-09-27-0156-named-answers-build.md) holds the focused commands and measured limits.
- The old survey was useful as a file map, but current main had more pages and working R0/R1/R2 rules. Re-running the scanner before edits reduced the actual fix list to 14 problems and prevented edits to the three correct pipeline samples.
- A warm command from codex-4 at `bbf9d6ce` ran the documentation samples before this lane built its own command. That result did not prove the changed Rust help; a local `cargo build` and generated-help inspection did.
- The two named-answer issues were already closed by a separate Quick Fix. This build closes the queue-owned page gap and does not move or reopen those issues. The remaining `examples.json`, unscanned folders, generic exit-code names and Rust doc comments need a later owner if Ian broadens the rule; the coordinator owns routing and the next full-batch checkpoint.
