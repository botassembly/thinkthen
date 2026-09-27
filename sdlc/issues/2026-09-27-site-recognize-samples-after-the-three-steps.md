# The site's recognize samples and recordings after the three steps

Status: Open. Filed 2026-09-27 by the queue owner for the marketing lead, from ticket 0147. Owner: the marketing lead, who owns `site/` (`sdlc/planning/ownership.md`).

## What changed

Ticket 0147 builds `recognize` in three steps, as ADR 0056 decides. Every recognize request body changes, so every recognize recording misses on replay. Each name now prints `text`, `start`, `end`, `length`, `kind` and `strength`. The field `name` is gone. With no kinds, every name has the kind `ENTITY`, and the three default kinds are gone. `strength` is now P(kind) times P(span). `--max-text-bytes` refuses a text over 600,000 bytes by default.

## What the site holds that no longer holds

- `site/examples/beatles/bench/examples/recognize/recording` holds a recording of the old requests. It needs a new live run.
- `site/examples/beatles/recognize/1-low.sh` and `2-high.sh` read `.name` with `jq`. Their `.out` files show the old shape and the old strengths.
- `site/examples/functions/recognize/` holds `1-details.sh` and `1-details.out`, and one sample for each surface. They read `name` and show the old `--details` members, `answer.tokens` in place of `answer.pieces`, `answer.names` and `answer.pairs`.
- `site/src/data/catalog.mjs`, the `recognize` entry. Its `requests` line and its `how` steps describe the old detection and kind questions, the kind vote and the old strength. The recognize page, `specification/recognize.md`, states the three steps plainly.
- Any recognize entry under `site/recordings`.
- `site/examples/functions/recognize/sqlite.sql` calls `thinkthen_relations`, which SQLite does not register, and `duckdb.sql` calls DuckDB's `thinkthen_relations` scalar in `FROM`. `closed/2026-09-26-three-items-from-the-named-answers-site-landing.md` item 3 gives the right call for each database.

## What to do

Record the Beatles recognize example again through `sdlc/scripts/live`, regenerate the samples against the landed surfaces, and rewrite the catalog's `recognize` steps from `specification/recognize.md`.
