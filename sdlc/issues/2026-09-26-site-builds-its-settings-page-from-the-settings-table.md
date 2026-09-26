# The site builds its Settings page from the settings table

Status: open. Filed 2026-09-26 by ticket 0140 for the website owner, who owns `site/`.

Ian ruled on 2026-09-26 that every setting is explained in one place. `specification/settings.md` now holds that table. The batching design's C1 section says the website generates its Settings page from the table at build time.

The table sits under the heading `## Settings`. Its 17 columns are fixed, in order: Setting, What it does, Default, Allowed values, Command flag, Environment variable, Configuration file, Question-file key, Rust, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL, SQLite. A pipe inside a cell is escaped as `\|`. `sdlc/scripts/settings` fails the `spec` rung when a column moves, so a site parser can rely on the order.

The ask: the site's build reads the table and renders a Settings page. The prose sections above and below the table explain precedence and list the settings on the way, and the page should carry them too.
