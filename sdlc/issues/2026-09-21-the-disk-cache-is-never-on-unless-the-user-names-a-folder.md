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

## How the cache stores and finds an answer, and where it stops scaling

Ian asked on 2026-09-21 how the cache works, how fast it is, how large it grows, and when an entry expires. The answers, from `specification/recording.md` and two measurements:

- **Storage.** One JSON file per request, in one flat folder, named by the SHA-256 of the adapter, the address, and the request bytes. It uses no database and no index file. The file system is the key-value store.
- **Finding.** The tool computes the digest and opens that one path. It never lists the folder.
- **Speed.** 100 whole runs of `decide --replay` on the refund how-to took 0.155 s on 2026-09-21, about 1.5 ms each with the process start included. A live request takes some hundreds of milliseconds.
- **Size.** No limit. A short entry is about 500 bytes and takes one 4 KB block on disk. The accuracy round's folders measure it: 2,940 entries in 12 MB, and 770 entries with 77 options each in 6.2 MB.
- **Expiry.** None. The first answer for a digest stays until the user deletes the file. A changed question, text, model name, or address is a new digest, and the old entry is never read again. `jev-latest` is one model name, and an answer saved under it outlives a change of the model behind it. A user who wants a clean break pins the model or clears the folder.

This design suits the command. An entry diffs in a pull request, a test folder is plain files, and deleting a file is the whole expiry rule.

It stops suiting a database somewhere. A million judged rows is a million files and about 4 GB in one folder. Nothing here has measured that. Before the database extensions promise "answers stay on disk" at that size, the experiment team measures a folder of a million entries: the time to find one, the time to fill, and the disk used. If it fails, the engine gets a second store behind the same setting, one file in place of a folder, and the recording folder stays as it is for tests. No code changes before that number exists.

## What Ian can overturn

All of it. The wider choice is a disk cache on by default in the databases alone, under a folder the extension documents. It saves a second bill for a user who never read the page, and it breaks the three points above.
