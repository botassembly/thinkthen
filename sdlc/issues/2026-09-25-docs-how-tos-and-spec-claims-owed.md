Status: Open for thirteen pages that can follow 0.1. Ticket 0350 wrote page 11 as how-to 48. Ticket 0316 rechecked every item on 2026-09-30 and cut the finished ones. The wrong spec claims, the 0.1 pages 5 to 10, the held-flag answers and the annotate help fix are done; git history holds their record.

Priority: ranked in `../planning/issue-priorities-2026-09-30.md`. Owner: a future docs ticket after 0.1; marketing for page 23.

# Docs and how-tos owed

A new page either joins the `demos/` list under ADR 0018 or becomes a site how-to. A page that shows a SQL or data frame call uses ADR 0105's settings form. The site keeps how-tos to 10 to 25 lines and no `--details`; a page that needs `--details` stays in `demos/`.

## Pages owed

12. **Grep a folder by meaning and print `file:line`.** Use `jq -Rc '{file: input_filename, n: input_line_number, text: .}'`, then `filter --jsonl --field /text`, then print `file:line: text`.
13. **Diagnose a failed agent trace.** `find --none` picks the failing step and `choose` names the kind of error.
14. **Ask a question about the answers.** `annotate` fills a form per ticket, `jq` groups the answers, and a second `decide` asks whether a group is one outage.
15. **Watch a live log.** `specification/records.md` "Order and requests" already states the streaming rule: an open batch goes after 50 ms with no new record, and output waits only behind earlier unprinted work. The `tail -f app.log | thinkthen filter ... --lines` page remains. Say that ThinkThen holds no state, no windows and no delivery promise.
16. **Map a week of incident reports.** `filter`, `recognize`, `relate` and `score` over about twenty made-up reports with a known answer, ending in SQL, green under `--replay`.
17. **An `examples/` folder with River Run as the first Python example.** Ian asked for it on 2026-09-23. Port the card game from workspace experiment 250, run it under replay with no key, and report its request counts. This needs its own ticket.
18. **One measured backend profile.** `profiles/README.md` still carries no claimed limit. Experiment 219 bracketed tokens, not bytes. Ship one profile with a measured byte ceiling and its record, and add one conformance case at a backend's own edge.
19. **Split one file into piles by a `choose` label.** Ian asked for it on 2026-09-20. The capability works: replaying how-to 21's recording, `choose ... --details` piped through `jq -r '[.value, (.input|tostring)] | @tsv'` and `awk -F'\t' '{print $2 > ($1".out.jsonl")}'` wrote one file per label. No how-to shows it. Moved here from the stumble register on 2026-09-30.
20. **Exit code 1 under `set -e`.** One page: the `if` form, the `||` form, what `--raw` prints, and a host that treats exit 2 as a block.
21. **Terminal recordings** scripted with VHS under `--replay`.
22. **A published skill file** that teaches an agent the verbs, the exit codes and `--plan`.
23. **Site search and a sitemap.** This one belongs to marketing.
24. **SQL and frame contract pages.** From withdrawn ticket 0290: a count and dialect table in each SQL README, DuckDB's macros (ticket 0286) in its README, SQLite's and PostgreSQL's one-call-per-row limit, and the Python frame pages' lazy limits. Each claim names the test that proves it.

Pages 20 to 23 moved here on 2026-09-30 from the release issue's "Moved to the docs issue" list, which ticket 0128 deferred to this issue.

## Handed to marketing

- "Buckets" beside band talk in the marketing repository's deck and objections pages. The fixed words are "band" and "middle range".
- The cost slide's 3.6 cents names no record.
