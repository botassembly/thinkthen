# 0611 — A use-case library of how-tos, open to search engines

Status: OPEN.

Milestone: 0.2.2

Source: Ian asked for this on 2026-10-01. The site content team handed it to ThinkThen on 2026-10-17 with its rulings and evidence intact. It starts after 0.2.1 ships.
## Why

Ian wants every use-case idea from 2026-10-01 on the site as its own how-to page. People search for how to use a decision model like Jev for a job. Those searches should land on these pages. The section stays out of the main menu, so it does not clutter it. The sitemap, the footer's sitemap columns and an index page reach it, so search engines find every page.

## Prior evidence

- Earlier site work made the site indexable. `robots.txt` allows all, and the sitemap lists every listed page. `site/src/pages/how-tos/` holds the existing how-tos.
- Function pages have one tab strip and replay every sample. Releases gate on the doc tests (`npm run test-docs`).
- Ian's rulings: prefer asserts to show results; keep examples simple, with no `--batch` in simple examples; Bash and SQL show their output.

## The ideas

Each line becomes one page. The title names the job and the tool, so it matches a search, for example "Triage alerts in Bash with a decision model".

Shell (Bash):
1. Alert triage: does this alert need a person now? Page, log, or queue on exit 3. `decide` with a band.
2. Failure routing: config, network, disk, code or flaky? A `case` runs each runbook. `choose`.
3. Grep by meaning: keep lines about the outage, whatever words they use. `filter`.
4. Login attack check: how suspicious is this session? Block high scores. `score`.
5. Flaky test or real break: retry or fail the build. `decide`.
6. Release notes from commits: feature, fix, breaking, docs. `tag`.
7. Check a sample sheet before a long pipeline run. `filter`.
8. Turn an incident email into JSON for `jq`. `annotate`.
9. Redact names and emails in a log before sharing it. `recognize`.
10. Fix the worst errors first: sort error clusters by harm to users. `rank`.
11. Retry or stop a pipeline job after out-of-memory or a timeout. `decide`.
12. Pre-commit guard: does this diff open a port to the internet? `decide`.
13. Secret check before upload. `decide`.
14. Hold a dependency upgrade whose changelog names a breaking change. `decide`.
15. Route a support ticket to its team. `choose`.
16. Quiet cron: is tonight's report normal? Email only when not. `decide`.
17. Pull the runbook step that fixes this error. `find`.
18. Sort a downloads folder into papers, invoices, datasets and decks. `choose`.
19. Label GitHub issues through `gh`. `tag`.
20. Rate scanner findings for this setup. `score`.

Data (SQL and data frames):
1. A WHERE clause by meaning. `decide` or `filter`.
2. Add a category column, then GROUP BY it. `choose`.
3. Pull fields from free text into columns. `annotate`.
4. Find duplicate records on candidate pairs from a join. `decide`.
5. Standardize messy values to a reference list. `choose`.
6. Score rows, then average by group. `score`.
7. Count topics: tag, unnest, count. `tag`.
8. Flag bad rows at load into a quarantine table. `decide`.
9. Second opinion on labels: list where a person and the model disagree. `decide`.
10. Top N per group with a window function. `rank`.
11. Relations into an edge table. `relate`.
12. Mask personal data in a free-text column. `recognize`.
13. Map a source column to a target column. `choose`.
14. Pull the cancellation reason into its own column. `find`.
15. Find contracts that renew on their own. `decide`.
16. Match tickets to help articles. `decide`.
17. Code open-ended survey answers into themes. `tag`.
18. Fill missing labels and keep the human ones. `choose`.
19. Pass on only edits that change the meaning. `decide`.
20. Drop junk form submissions. `filter`.

Agent harnesses:
1. Permission: should this command run? Ask a person when not sure.
2. Tool choice: load only the top few tools.
3. Context: does this chunk or guide matter now?
4. Model choice: send easy steps to a smaller model.
5. Confidence cascade: an unsure answer goes to a bigger model.
6. Parallel work: can these tasks run at once?
7. Data safety: could this touch secrets?
8. Off track: is the agent going in circles?
9. Done check: is the task done as asked?
10. Review: does this change do what was asked?
11. Evaluation: which prompt wins, in place of LLM-as-a-judge.
12. Refusal guard: one question per refusal case, each with its own cutoff.
13. Memory recall: does this old note matter now?
14. Escalate: is this hard case one for a person?
15. Write order: which agent writes first?

## Change

1. Add a section at `/how-tos/use-cases/` with an index page grouped by Shell, Data and Agent harnesses. Leave it out of the main menu. Link the index from the footer and the how-tos index. List every page in the sitemap and `llms.txt`.
2. Write one page per idea in plain sentences per `site/WRITING.md`: the problem in two or three sentences, the question, one short replayed example, its output or assert, what the code does with the answer, and how to tune the cutoff. Shell pages use Bash. Data pages show DuckDB SQL first and one data-frame example where it fits. Agent pages use Bash or Python. Keep each example simple, with no `--batch`. Give each page a unique title and description, and link its function page.
3. Every example is a doc test. It replays in `npm run smoke-bindings` or `npm run smoke-sql` and passes `npm run test-docs`. Use made-up but realistic inputs. Use no real person's data. Overlapping ideas get distinct inputs and angles.
4. Record new answers only through ThinkThen `sdlc/scripts/live`, under a $0.50 cap for the ticket, within Ian's standing budget ruling of 2026-09-28. Reuse recordings where the question and text match.
5. Add the section to `check-words`, `check-links` and `check-widths` coverage as the other pages are.

The work may land in slices: shell, data, agents.

## Ticket review decisions

The fresh ticket review of 2026-10-01 returned ten findings. The lead decided each one, and these override the text above:

1. Bash examples replay in `npm run smoke` (`scripts/smoke.mjs`). SQL and library samples replay in `smoke-bindings` and `smoke-sql`.
2. Slice order: shell first, as Bash pages on today's how-to template. The data slice first adds runner and proof support for `how-tos/` sample paths (`binding-proofs.mjs` `sampleSurface` and `samplePage`, `REPLAY`, `check-binding-proofs`) and SQL and library tabs on the how-to template. The agent slice reuses that.
3. The menu, sitemap and `llms.txt` already behave as wanted. Confirm them. Add the footer link in `SiteFooter.astro`.
4. The checks walk `dist/`. Confirm coverage, with no change.
5. The cap is `--max-tokens 2000000` for the ticket. At Jev's input price of $0.042 per million tokens (`sdlc/records/qf-readme-first-run.md` in ThinkThen), that is about 8 cents of input, inside the $0.50 ceiling.
6. Where an idea matches an existing how-to (data 4, 8, 16 and 17; shell 1, 10, 12 and 15; agent 1), do not write a duplicate. Improve the existing page's title for search if needed, and list it on the index under its idea.
7. Shell 19 feeds a saved `gh` JSON file. Shell 16 stubs the email step with `echo`. Agent 5, the cascade, folds into agent 4, Model choice. Agent 11, Evaluation, is a short page that links the audit how-to. Agent 15, Write order, is dropped. Data 1 uses `WHERE thinkthen_decide`, and data 10 uses `ORDER BY thinkthen_probability`. Data 11 and 14 keep a small rules file or unit list in the sample folder.
8. Every name is invented. Shell 13's fake key uses an obviously fake format that secret scanners and push protection ignore.
9. The build starts after 0.2.1 ships.
10. Agent pages name these functions: Permission `decide` with a band; Tool choice `rank`; Context `decide`; Model choice `decide` with a band; Parallel work `decide`; Data safety `decide`; Off track `decide`; Done check `decide`; Review `decide`; Evaluation `audit`; Refusal guard several `decide` questions; Memory recall `rank`; Escalate `decide` with a band.

## Retained behavior

The main menu stays the same. Every other page keeps its content. `npm run build`, `npm run check`, both smoke runs and `npm run test-docs` pass with every key and `THINKTHEN_` variable unset and `CARGO_TARGET_DIR` unset.

## Proof

The built sitemap listing every page, shot-scraper screenshots of the index and three pages at 375 and 1280 px, the live spend from `live --status`, and a fresh reviewer's ACCEPT per slice.

## Deferred

Translations of each example into the other bindings.
