# The disk cache is never on unless the user names a folder

Status: Open

Ian asked on 2026-09-21 whether the cache is always there. His worry: a side effect nobody asked for is strange to people. This page states the rule for all ten surfaces, so the ADR 0017 rewrite and step 4 of the engine plan have it. It authorizes nothing.

## What is true today

The command saves nothing by default. `--cache DIR` turns the cache on, and the user names the folder. `specification/recording.md` holds the rule.

The planning pages for the libraries and the databases are less clear. `sdlc/planning/databases/README.md` rule 5 says "Answers are kept on disk" and "Tomorrow's run of the same query costs nothing". A reader takes that as on by default. No page says where the folder would be.

## The rule

Two kinds of saving, with two defaults:

| Kind | Where it lives | Default | Why |
| --- | --- | --- | --- |
| Asking once inside one call | Memory, for the length of one bulk call or one query | Always on | Equal pairs of question and text in one batch are one judgment. It writes nothing and outlives nothing. Nobody can observe it except on the bill |
| Answers kept across runs | A folder on disk | Off until the user names the folder | See below |

Three reasons hold the disk cache off by default:

1. `CLAUDE.md` says the tool "never writes a file the user did not name". A cache folder the library picked is such a file.
2. A cache entry holds the text that was judged. A default folder would copy a user's messages or records to a place they never chose.
3. `jev-latest` is an alias. A saved answer outlives a model change, and a user who never asked for a cache would not know to clear one.

Every surface spells the setting its own way: `--cache DIR` in the command, a `cache` setting on the engine value in a library, and a session setting in a database. No surface picks a folder on its own, and no environment variable turns a cache on silently.

## What this changes in the database pages

The memory rule alone covers the measured DuckDB case, where one call in `WHERE` and again in `SELECT` cost 8 requests with no cache and 4 with one, if the memory lasts for the query. "Tomorrow's run costs nothing" holds only after the user sets the folder. Rule 5 and `thinkthen_warm` need that sentence. `thinkthen_warm` with no folder set fills the memory for the session, and its page says so.

## What Ian can overturn

All of it. The wider choice is a disk cache on by default in the databases alone, under a folder the extension documents. It saves a second bill for a user who never read the page, and it breaks the three points above.
