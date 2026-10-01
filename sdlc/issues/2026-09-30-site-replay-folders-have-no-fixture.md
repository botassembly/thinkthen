# The PostgreSQL site samples run under no check, and three status phrases pass the word check

Status: open for part 3 and one item of part 4, both marketing's. Part 3 waits on site ticket 0047 slice F, which replays the PostgreSQL samples. Parts 1 and 2 are done. Found on 2026-09-30 by running the site smoke against main `acebf3044`. Marketing owns `site/` (`sdlc/planning/ownership.md`). Owner of the rest: marketing. Merged on 2026-09-30 with the site samples issue, now in `closed/`.

Kind: debt

Pay when: site 0047 slice F lands and `check-words` refuses the three phrases in part 4, before the site goes public with 0.1.

Debt: 007

Severity: medium

Milestone: 0.1

Keeping it lets the PostgreSQL tab drift from its surface with no check failing, and lets three status phrases back onto the site.

## Done

- Parts 1 and 2 landed with site ticket 0031. The fifteen replay folders hold converted fixtures, and the five `--dry-run` examples use `--plan`.
- Part 4, except one item: `check-words` refuses preview, planned and beta, the pandas page exists, and `check-links.mjs` fails on a code tag after a table.
- Site tickets 0038 and 0043 replay every binding's install and backends samples, 77 in all, through `npm run smoke-bindings`.
- Site ticket 0047 slices A to E replay the function samples for Python, Polars, R, Ruby, TypeScript, C and Rust, and the SQLite and DuckDB samples (`1da03f0f2`, `ca9fe5f7f`, `68bee98ba`, `3fe726b54`, `d2cf57b26`).

## 3. Run the library and database samples (marketing)

Part 3 waits on site ticket 0047 slice F. That slice replays the PostgreSQL samples and removes the pending list, on branch `ticket/site-0047-f`. Part 3 closes when slice F lands on main. Close this issue when parts 3 and 4 are both done.

## 4. Three status phrases (marketing)

`site/scripts/check-words.mjs` refuses preview, planned and beta. It does not refuse "ships first", "not run yet" or "comes with 0.1", which this part named. No page uses them today. Done when `check-words` refuses all three, or the site owner records why they were dropped.
