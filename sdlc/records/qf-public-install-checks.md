# Quick Fix qf-public-install-checks: record the 0.1.1 install checks and name what the install pages left out

Status: built on `ticket/qf-public-install-checks` from main `5cead8703`. The first fresh review returned two findings, both answered.

## Why

Ticket 0128 phase 4 step 8 asks for one install from each channel after the release publishes. The checks ran on 2026-10-03 against 0.1.1. The release record's "Public install checks" section waited for their results.

The checks followed each install page as a stranger would. Five pages left a step out:

- The SQLite, DuckDB and PostgreSQL pages show a `.load`, a `LOAD` or a `CREATE EXTENSION` line. None named the release archive that holds the extension. The SQLite archive holds `libthinkthen0.so`, and the page loads `./thinkthen`, so a reader must rename the file first. The PostgreSQL archive holds `lib/` and `extension/`, and the server reads them only from its own folders.
- The Python page named no Python floor. The wheels need Python 3.10. macOS's own Python 3.9 installs the 0.0.1 placeholder with no message.
- The Ruby page named no Ruby floor. The gems need Ruby 3.4. Ruby 3.3 installs the 0.0.1 placeholder with no message.

## Change

- `sdlc/records/0128-release-0-1.md`: the "Public install checks" section holds the results table and five findings. Checklist step 8 reads done, and "What stays open" names ticket 0394 and the R-universe build.
- `site/src/data/catalog.mjs`: the SQLite, DuckDB and PostgreSQL pages list their release archive first, with the step that places it. The Python page's `pip` line notes Python 3.10 or later. The Ruby page's line notes Ruby 3.4.
- `site/scripts/check-binding-proofs.mjs`: the archive check reads the database archives too. It accepts an archive `release-pack` packs by its whole name. A wrapper name must end at a space or a semicolon, so a prefix such as `co` no longer passes for `cobol`. Its pattern now takes the digits in `postgresql16`.

The R page's line is ticket 0392's change, and this fix leaves it alone. `release/0.1` needs no copy. Pages deploys the site from main, and the record lives on main.

## Checks

- `node scripts/check-binding-proofs.mjs` passes with the same 11 stale-page warnings as main `5cead8703`. With the DuckDB row renamed to `postgresql17`, `co`, `fl`, `g` or `objective` in turn, it fails each time with `release-pack makes no archive named` and that name.
- `npm run build` in `site/` passes, with `THINKTHEN_BIN` naming a binary built from main. The built SQLite, DuckDB, PostgreSQL, Python and Ruby pages show the new rows. `node scripts/check-widths.mjs` passes on all 138 pages.
- The private-name guard over this diff's added lines finds nothing, and no added line holds a home path.

## Lessons

- A registry check proves the package. It does not prove the page. Three install pages passed every site check and still lacked the step that puts the file in place.
- A name placeholder stays a trap after the release. Each registry falls back to it on any host the real files do not cover, and says nothing.

## Deferred gap

- The site redeploy. Ticket 0392 dispatches Pages after it lands. This fix rides the same deploy if it lands first, or the next one.
