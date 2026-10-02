# Three status phrases pass the site word check

Status: open for one item of part 4, owed by the builder. Parts 1, 2 and 3 are done. Part 3 landed with site ticket 0047 slice F (`e474b27c2`), which replays the PostgreSQL samples. Found on 2026-09-30 by running the site smoke against main `acebf3044`. The builder owns `site/` (`sdlc/planning/ownership.md`). Merged on 2026-09-30 with the site samples issue, now in `closed/`. Moved from 0.1 to later on 2026-10-01: the open item changes no product behavior and no page uses the phrases, so it does not block a release.

Kind: debt

Pay when: the builder next changes `site/scripts/check-words.mjs`.

Debt: 007

Severity: low

Milestone: later

Keeping it lets three status phrases back onto the site with no check failing.

## Done

- Parts 1 and 2 landed with site ticket 0031. The fifteen replay folders hold converted fixtures, and the five `--dry-run` examples use `--plan`.
- Part 3 landed with site ticket 0047 slices A to F. Slices A to E replay the function samples for Python, Polars, R, Ruby, TypeScript, C and Rust, and the SQLite and DuckDB samples (`1da03f0f2`, `ca9fe5f7f`, `68bee98ba`, `3fe726b54`, `d2cf57b26`). Slice F replays the PostgreSQL samples and closes the pending list (`e474b27c2`).
- Site tickets 0038 and 0043 replay every binding's install and backends samples, 77 in all, through `npm run smoke-bindings`.
- Part 4, except one item: `check-words` refuses preview, planned and beta, the pandas page exists, and `check-links.mjs` fails on a code tag after a table.

## 4. Three status phrases (builder)

`site/scripts/check-words.mjs` refuses preview, planned and beta. It does not refuse "ships first", "not run yet" or "comes with 0.1", which this part named. No page uses them today. Done when the builder adds the three phrases to `check-words`, or records why not. Close this issue then.
