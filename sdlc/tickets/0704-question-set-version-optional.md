# 0704 — A question set's version is optional

Status: OPEN.

Milestone: 0.2.1

## Outcome

- Changes: `crates/thinkthen/src/core/question_set/parsing.rs`, `crates/thinkthen/tests/`, `specification/`, `site/src/`, `libraries/*/README.md`, `databases/*/README.md`.
- A question set without `version`, inline or in a saved file, parses as version 1. A present `version` other than the number 1 is still refused with the same message.
- Documentation examples drop `version`. The specification states that it is optional and defaults to 1.

### Proof

- A Rust test parses the same set with and without `"version": 1` and gets equal question sets; `"version": 2` and `"version": "1"` are still refused. It fails before the change on the missing-version case.
- One SQLite or PostgreSQL `thinkthen_annotate` example without `version` replays.

## Evidence

`core/question_set/parsing.rs` refuses a set without `version`: `thinkthen usage: version is the number 1`. Version 1 is the only version, so readers of a trigger or query see a field that does nothing for them. Marketing's PostgreSQL example shows the set without it. Ian approved making it optional on 2026-10-19.

This changes the question-set contract before the core freeze (0688), so every language picks it up with no extra work.

## Amendment, 2026-10-19

The no-version ruling (`sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`) supersedes "missing reads as 1". 0704 now removes `version` from question sets, recognize files and relate files outright: the member is gone from the parsers, the question-file and request schemas, MCP tool schemas, generated wrapper inputs, docs, site examples and fixtures, and a file that still carries it is refused with one sentence naming the member to delete. The question-set hash keeps its current constant so `questions_sha256` values do not change. Proof: Rust tests parse each of the three without the member and refuse each with it, with the exact sentence.

## Progress

- 2026-10-20 started
- 2026-10-20 next: Source 45dcac5fd834272e4bdc571d37ec148b046ab903 implements the amended no-version declaration contract and preserves the question-set hash. M5 receipts: 1792470882512-20816 at 231dd673b passed policy, failed formatting; 1792470908630-21925 formatted successfully. 1792470923778-22876 at ee0577543 stopped with exit 143 during tool preparation after stale generated schemas were found; its process group ended. 1792471017034-26418 at 4ef92d181 passed schema regeneration (expected schema-writer exit 101), all existing generators and formatting. 1792471304456-83133 at abdbefa92 stopped with exit 143 before tests to replace an unintended full native run with focus. 1792471333097-85167 at abdbefa92 passed old-parser red proof (exit 101 with missing-version relate failure), then all three selected declaration/schema/hash tests. 1792471725157-98874 at abdbefa92 failed Clippy type complexity; fixed with a Parser alias. 1792471864067-3790 at c23a8ec90 passed Linux lint, failed routine tests from the relation builder emitting version plus three stale test expectations. 1792472285851-17940 at 45dcac5fd passed six focused regressions after fixing the source and expectations. 1792472343407-22427 at 45dcac5fd passed the configured gate (Linux lint and tests, native tests, supporting scripts) and spec, then exited 1 at the unchanged SQLite string comparison of padded macOS wc output; this remains a whole-surface limitation. 1792473662997-93430 direct SQLite examples failed only a relative filter_source lookup from repo cwd. 1792473673421-94652 from databases/sqlite passed every existing example, including unversioned annotate. Wait for 0642 to land, rebase onto main, and request the coordinator cue before running the new required packet-currency check; the M5 slot is released to 0620. No oracle or shared runner edits.
