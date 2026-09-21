## Review

**Verdict: do not merge `surfaces` into main on the strength of the records as they stand.** The experiment's own claims are unusually well-kept — 18 cited commit hashes all resolve, the "not run" lists are real, and the vocabulary sweep held — but three records-level defects block a clean merge: the ruled question-file spelling for a relation's ends does not work in the shipped parser, two divergent conformance files both call themselves "the one file", and the branch's copy of the rulings issue now disagrees with main's copy in both directions. Read-only review: no commands, builds, or tests were run (read/grep/find/ls only); no files written.

Snapshot used: branch `surfaces` head `0801a329f5a6…` (matches the pin), base `b11a2b03…`, thinkthen main `db19349…`, mktg main `95f17ee…`, date 2026-09-21.

### Findings, by consequence

**F1 — The question file spells a relation's ends `from`/`to`, not the ruled `source`/`target`.**
Classification: confirmed defect. Violated promise: the accepted ruling "a relation's ends are `source` and `target`, everywhere … the command's own JSON and the question file, so no door converts anything" (`sdlc/planning/recognize-design.md:188-190`, `sdlc/planning/relate-design.md:101-103`) and the contract's own doc (`contract/src/lib.rs:883-889`).
Evidence: the parser requires `from`/`to` and has no `source`/`target` path — `relation_from_value`, `contract/src/lib.rs:696-719` ("`from` and `to` required", `end("from")?`, `end("to")?`), its tests use `"from":"person","to":"organization"` (`contract/src/lib.rs:1931-1933`); case 72's question carries `"from": "*", "to": "*"` (`conformance/conformance.json`, case `72-relate-R04-pairs-10`). The gap is already recorded inside a lane, not as a finding: `libraries/python/NOTES.md:326-329` ("the landed parser reads `from`/`to` … the test uses the parser's spelling and names the gap"). FINDINGS.md:37 repeats the false "question file … no door converts anything".
Reproduction not run; the code path is deterministic. Smallest fix: accept `source`/`target` in `relation_from_value`, keep `from`/`to` only as the replay-boundary legacy spelling, re-key the conformance questions, and make the three pages say one thing.

**F2 — Two different files are "the one conformance file", with different case ids and coverage.**
Classification: confirmed inconsistency, merge-blocking risk. Violated promise: ADR 0017's shared-case-contract amendment names `conformance/cases.json` with twenty-five initial cases (`sdlc/planning/adr/0017-libraries-over-one-bound-core.md:158-162`); FINDINGS.md:36-39 and `SURFACES.md` present the branch's 72-case file as the one file every surface reads.
Evidence: branch `conformance/conformance.json:2-3` (`case_count: 72`, ids `01-decide-yes-cut` … `72-relate-R04-pairs-10`, no `defect` case by principle); main `/home/ian/workspace/repos/thinkthen/conformance/cases.json:2-4` (`case_count: 27`, ids `01-decide-yes-captured` … `25-defect-fault`, including a `defect` fault case the ADR amendment calls a schema contract); the branch's own Phase A record validated "20 cases" for `conformance/conformance.json` (worktree copy of the rulings issue, the `731a058` bullet). Three counts coexist (20, 25/27, 72) for one named artifact.
Smallest fix: decide the file name and union the case sets before the merge, and add the missing `defect` case or record why the branch drops it.

**F3 — The deck's `thinkthen_relations` call cannot run as drawn on any of the three databases, and no finding was filed for it.**
Classification: confirmed defect plus claim overreach. Violated promise: the update issue's read order says of `recognize-surfaces.md` "This page is the acceptance test. Each call must run as written"; its closing says "four findings, all filed".
Evidence: the drawn line is in the recognize SQL block (`SELECT * FROM thinkthen_relations(body, '@names.json')`, `decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`, "all three, beta"). DuckDB: "The relations call cannot run as drawn either … the C API table functions take literal parameters only", working form is the scalar list (`databases/duckdb/NOTES.md:90-94`). PostgreSQL: the working form needs `LATERAL` (`databases/postgresql/NOTES.md:133-137`). SQLite: "**Not run, honestly:** the beta `thinkthen_relations` line in the deck's SQL block … nothing was invented for it" (`databases/sqlite/NOTES.md:95`). mktg carries three findings files, none for this.
Smallest fix: file the finding in mktg (or paste the DuckDB lane note's finding 2 verbatim) and add it to FINDINGS.md's list; then change the deck line or mark it beta-unbuilt.

**F4 — `conformance/DIVERGENCES.md` still calls the name's number interim `number`.**
Classification: confirmed inconsistency (record vs shipped code). Violated promise: the one-rule page's ruling (`sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`, "the computed number stays, and its name is `strength`") and FINDINGS.md:37 ("`Entity` with `strength`").
Evidence: `conformance/DIVERGENCES.md`, R1 bullet 1 ("The entity number's field name is interim … the contract, the stand-in, and every expectation here carry the neutral field `number` … the free comparison … settles the final name") versus `contract/src/lib.rs:873-880` ("Settled by Ian on 2026-09-21: the field is `strength`") and every entity in `conformance/conformance.json` (`"strength"`), plus the rename commits `78204cb`, `14f418e`, `2356976`.
Smallest fix: rewrite that bullet as history and point at the rename commits.

**F5 — A stale 1 GiB cache cap and a stale rule-5 sentence survive the 100 MB ruling.**
Classification: confirmed inconsistency. Violated promise: ADR 0017:79 and :83-85 (100 MB, XDG, each database page tells the reader to raise it) and the rulings issue:13-15.
Evidence: `sdlc/planning/databases/README.md:49` ("the 1 GiB cap and the prune ship with the location"); `sdlc/planning/databases/postgres.md:75` (same); ADR 0017:132 says "rule 5 of `databases/README.md` says the disk cache waits for a named folder" while the shipped rule 5 says the opposite (cache on by default at the XDG home). Issue `2026-09-20-feedback-to-the-experiment-team-after-both-harvests.md:110` also says 1 GiB, but that is a dated statement about ADR at `49fb7ad`, so it is history, not an error.
Smallest fix: two number edits and one sentence in ADR 0017's consequences bullet.

**F6 — "The findings, all pinned and filed" is not true as written, and the update issue's four-item list does not match the four files.**
Classification: confirmed inconsistency (claim). Violated promise: FINDINGS.md:41 ("The findings, all pinned and filed"); the update issue's closing paragraph.
Evidence: mktg `sdlc/issues/` holds exactly three files (Ruby score `3041f17`, `located_in` `30a7edb`, DuckDB `relate` `95f17ee`); the update issue's list of four names the `located_in` and DuckDB items but also "case 72 lost its `form` field … and is restored" (no file, no commit, no lane record; `conformance.json` case 72 does carry `"form": "pairs"`) and the `confidence`→`strength` overturn (a ruling change recorded in the one-rule page, not a finding); the list omits the Ruby-score finding that exists, and omits the `thinkthen_relations` failure (F3).
Smallest fix: name the file or commit for the case-72 item, or drop it from "filed"; add the missing items to the list.

**F7 — "three sample findings filed, one defect fixed" overstates the filing, and FINDINGS item 3 contradicts itself.**
Classification: inconsistency (claim). Evidence: the rulings issue's closing headline; FINDINGS.md:24-27 says "Every slide sample runs as drawn with the answer in its comment" and then names a finding whose comment does not hold (Ruby score 2.0 vs recorded 1.7, `mktg/sdlc/issues/2026-09-21-the-ruby-slide-pins-a-score…`) and one where the comment is the real backend's answer and not the stand-in's (`libraries/python/NOTES.md:51-59`, "The band comment does not reproduce"). Only the Ruby finding is filed in mktg; the TypeScript defect was fixed in-branch (`998b3a1`) and the band comment is recorded in the lane note.
Smallest fix: replace "filed" with the recorded locations, and split the sentence: samples run as drawn; two comments are known not to reproduce.

**F8 — `reset_usage` is a public name on three surfaces that no ruling admits, and nothing enforces the name list.**
Classification: confirmed inconsistency plus design gap. Violated promise: ADR 0017 pick 1 ("The public list is the eight verbs …, `question`, and `details` … No surface renames a verb or adds one"; only `decide_many`/`thinkthen_decide_many` are admitted by name, :95 and :108-113).
Evidence: `functions.toml:92`, `libraries/python/thinkthen/__init__.py:43,53`, `libraries/typescript/index.mjs:26`, `libraries/r/thinkthen/R/thinkthen.R:443`; the contract trait does not declare it (`contract/src/lib.rs:13-19`), and Rust, C, and the three databases do not expose it. No check enumerates public names: `scripts/check_surfaces.sh` runs per-surface checks and the conformance slice; `generate_functions.py --check` guards only the two generated name files.
Smallest fix: admit `reset_usage` in ADR 0017 (or drop it), and add the name-list check the ADR's wording already assumes.

**F9 — Settled questions still read as open, and two pages carry no `Status:` line.**
Classification: inconsistency (record hygiene). Evidence: `sdlc/planning/relate-design.md:86` still lists "The `probability` and `confidence` question above" as open for the build team after the one-rule page settled it; `relate-design.md:78` still shows rows `(name, from_id, to_id, probability)` (the SQLite lane flagged it, `databases/sqlite/NOTES.md:97`); `recognize-design.md:167` leaves `thinkthen_relations` open while its table `:149` lists the call for all three databases and the deck draws it; the one-rule page and `2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md` are the only issues in scope with no `Status:` line (the pandas page ends "2026-09-21, closed:" in prose). Smallest fix: mark the settled items, add the two Status lines.

**F10 — The branch and main hold two diverging copies of the same issue file, with disjoint additions.**
Classification: risk (merge integrity). Evidence: the worktree copy of `sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md` adds the Phase A/B record (its lines ~42-68) and does not contain the Polars ruling or "The surfaces experiment closes"; main's copy has those two sections and no Phase A block. A merge that takes either side whole deletes the other's record. Smallest fix: reconcile the file by hand in the merge ticket and keep both blocks.

### Strengths

- Every commit hash cited in FINDINGS.md resolves in the branch reflog (`ef3767c`, `69e9385`, `731a058`, `7145073`, `b622e5a`, `28e4025`, `998b3a1`, `8fc2db7`, `1ac9250`, `1af5290`, `78204cb`, `14f418e`, `2356976`, `ad1882f`, `26cea03`, base `b11a2b0`, head `0801a32`), and the three mktg hashes (`3041f17`, `30a7edb`, `95f17ee`) are real mktg commits; `95f17ee` is mktg's head.
- The count arithmetic is honest and self-consistent: 27 + 40 + 1 + 4 = 72, `case_count` equals the array length (`conformance.json:2-3`), the validator asserts it (`conformance/tools/validate_conformance.py:74`), and the six kinds including `deadline` match the contract enum exactly (`contract/src/lib.rs:94-111`).
- The ruled picks really are in the code: `THINKTHEN_UNSURE` and `thinkthen_decide_many` (`contract/include/thinkthen.h:51,92`), `score` returning the position with the nearest level in details (`functions.toml:36`), Rust blocking with no `.await` (`sdlc/planning/libraries/rust.md:20`), `thinkthen_warm` in all three databases (`databases/*/src`), and the vendor's `confidence` only in detailed output (`crates/thinkthen-core/src/find.rs:222-232`).
- The three mktg findings each cite a real path and a real error string, and their statuses are accurate: Ruby's is genuinely closed (the deck comment now reads `0 is "Routine.", 2 is "Immediate."`, `surfaces.md:107-109`), while the other two stay open with the decks unchanged.
- The "not run, honestly" lists are true against the lane notes (no `defect` case, R's user-installed handler, macOS database artifacts, plugin expression), and `NOTES-maintainability.md` reports the test as a negative result — the count did not drop — which is the kind of honest record this lens rarely sees.
- The one-rule page's numbers match the underlying experiment exactly (`experiments/227-name-number/report.md`: 0.8398/0.7608 computed, 0.7610/0.7217 minIN, 0.8239/0.7613 minAll), and the marketing vocabulary table agrees word for word (`mktg/products/thinkthen/vocabulary.md:171-181`).

### Remediation roadmap

1. Decide the question-file spelling and make parser, contract doc, both design pages, the deck, and the conformance questions agree (F1).
2. Reconcile the conformance artifact: one name, one unioned case set, the `defect` case accounted for (F2); hand-merge the rulings issue both ways (F10).
3. File the `thinkthen_relations` finding and correct the "all filed" sentences (F3, F6, F7).
4. Fix the stale records: `DIVERGENCES.md` interim name (F4), the 1 GiB/rule-5 lines (F5), the settled-but-open items and missing Status lines (F9).
5. Decide `reset_usage` and add the name-list check (F8).
6. Leave for the build team's merge ticket: the cache cap, prune, `status`, and the XDG rule-text exception, which the branch itself records as unbuilt.

### Commands a supervisor should run (I ran none)

- `cd /home/ian/workspace/worktrees/thinkthen-surfaces/conformance && python3 tools/validate_conformance.py conformance.json` (offline; expect OK over 72 cases)
- `cd /home/ian/workspace/worktrees/thinkthen-surfaces && python3 scripts/generate_functions.py --check` (name-list drift; currently 15 rows, not 13)
- `cd /home/ian/workspace/worktrees/thinkthen-surfaces && bash scripts/check_surfaces.sh` (builds contract/stand-in; wire tests self-skip without the loopback stub)
- `git -C /home/ian/workspace/worktrees/thinkthen-surfaces log --oneline b11a2b0..HEAD` and `git -C /home/ian/workspace/repos/thinkthen rev-parse HEAD` / `git -C /home/ian/workspace/repos/mktg rev-parse HEAD` to confirm the pinned heads
- `git -C /home/ian/workspace/repos/thinkthen merge-tree` (or a trial merge in a scratch worktree) to expose the two-sided edits to the rulings issue and the `conformance/` name collision

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Ten findings with file:line evidence, classified, prioritized by consequence: F1 contract/src/lib.rs:696-719 vs :883-889 and recognize-design.md:188-190; F2 conformance/conformance.json:2-3 vs repos/thinkthen/conformance/cases.json:2-4 and ADR 0017:158-162; F3 databases/duckdb/NOTES.md:90-94, databases/sqlite/NOTES.md:95, databases/postgresql/NOTES.md:133-137; F4 conformance/DIVERGENCES.md R1 bullet 1 vs contract/src/lib.rs:873-880; F5 sdlc/planning/databases/README.md:49, postgres.md:75, ADR 0017:132; F6 FINDINGS.md:41 vs the three files in mktg/sdlc/issues; F7 rulings issue closing vs FINDINGS.md:24-27 and libraries/python/NOTES.md:51-59; F8 functions.toml:92 and libraries/python/thinkthen/__init__.py:43,53; F9 relate-design.md:78,86; F10 the two copies of the rulings issue."
    }
  ],
  "changedFiles": [],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "cd /home/ian/workspace/worktrees/thinkthen-surfaces/conformance && python3 tools/validate_conformance.py conformance.json",
      "result": "not-run",
      "summary": "Recommended: no shell available in this review; validator asserts case_count == len(cases) per conformance/tools/validate_conformance.py:74"
    },
    {
      "command": "python3 scripts/generate_functions.py --check",
      "result": "not-run",
      "summary": "Recommended: guards only libraries/python/src/generated.rs and libraries/typescript/index.mjs; would show 15 rows now, not the 13 recorded at 39cadca"
    },
    {
      "command": "bash scripts/check_surfaces.sh",
      "result": "not-run",
      "summary": "Recommended: builds contract and stand-in, validates the conformance file, runs each landed surface's check.sh; wire tests self-skip without the loopback stub"
    },
    {
      "command": "git -C /home/ian/workspace/worktrees/thinkthen-surfaces log --oneline b11a2b0..HEAD",
      "result": "not-run",
      "summary": "Recommended: commit-hash existence was verified from .git/logs/refs/heads/surfaces instead"
    }
  ],
  "validationOutput": [
    "All 18 commit hashes cited in FINDINGS.md resolve in .git/logs/refs/heads/surfaces (head 0801a329f5a6ddb8e9678f0b7c30898d450dd3d6 matches the snapshot pin; base b11a2b03... matches).",
    "mktg 3041f17, 30a7edb, 95f17ee are real commits in .git/logs/HEAD of mktg; 95f17ee is the mktg head.",
    "conformance/conformance.json: case_count 72 matches the pin arithmetic (27 + 40 + 1 + 4) and error_kinds lists the six kinds including deadline.",
    "Counts of the three mktg findings files confirmed by directory listing: three files, not four.",
    "Cited experiment and page paths exist: experiments/222-recognize-demo/output.json, 225-relate-demo/edges-0.5.jsonl, 226-graph-demo/arm/run.jsonl, 227-name-number/report.md, mktg/products/thinkthen/vocabulary.md."
  ],
  "residualRisks": [
    "F1 defect proven by code reading, not by executing a file door (no shell available here); a one-line run would confirm the usage error on 'source'/'target'.",
    "F2's reconciliation depends on which artifact ticket 0052 actually landed; I inferred main's 25-to-27 growth from the file and the ADR amendment rather than from the ticket's diff.",
    "F3's SQLite claim rests on the lane's own 'not run' note; I did not execute any database.",
    "The branch's copy of the rulings issue was compared against main's copy by reading both files; a real merge dry run is still owed.",
    "Wire and packaging claims (clean-container installs, macOS cross-builds, cancel proofs) were not re-run and are taken as recorded in the lane notes."
  ],
  "noStagedFiles": true,
  "diffSummary": "Read-only records-and-claims review; no files changed, no tests run.",
  "reviewFindings": [
    "blocker: worktrees/thinkthen-surfaces/contract/src/lib.rs:696-719 - the question-file relation parser requires 'from'/'to' while contract/src/lib.rs:883-889 and both design pages rule 'source'/'target' for the question file; the ruled spelling fails (confirmed defect).",
    "blocker: conformance/conformance.json vs repos/thinkthen/conformance/cases.json - two different 'one conformance file' artifacts (72 cases with ids 01-decide-yes-cut... vs 27 with ids 01-decide-yes-captured ... 25-defect-fault), and ADR 0017:158-162 names cases.json with twenty-five cases.",
    "blocker: mktg/sdlc/issues holds three findings files, not four; the update issue's 'four findings, all filed' includes a case-72 item with no file or commit, and FINDINGS.md:41's 'all pinned and filed' omits the thinkthen_relations as-drawn failure recorded in databases/duckdb/NOTES.md:90-94.",
    "major: conformance/DIVERGENCES.md R1 still declares the name's number interim 'number' while contract/src/lib.rs:873-880 and conformance.json ship 'strength'.",
    "major: sdlc/planning/databases/README.md:49 and postgres.md:75 still carry a 1 GiB cap after the 100 MB ruling, and ADR 0017:132 still attributes the superseded rule-5 wording to databases/README.md.",
    "minor: reset_usage is a public name on Python, TypeScript, and R that no ruling admits, and no check enumerates public names to enforce 'no surface adds one'.",
    "minor: relate-design.md:86 lists the settled probability/confidence question as open; recognize-design.md:167 leaves thinkthen_relations open while the deck draws it; the one-rule and pandas pages carry no Status line.",
    "no blockers found in: commit-hash existence, the 72-case arithmetic, the six error kinds with deadline, thinkthen_warm on all three databases, the 'not run' lists, or the one-rule page's numbers against experiments/227-name-number/report.md."
  ],
  "manualNotes": "Review-only instructions won over the progress-file habit, so no progress.md was written; nothing was executed and no artifacts were created. The parent's premise of four mktg findings files is wrong on the record: three exist, and the mismatch is itself finding F6. The strongest single evidence the parent may want next is a merge dry run over sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md, which has disjoint additions on branch and main."
}
```