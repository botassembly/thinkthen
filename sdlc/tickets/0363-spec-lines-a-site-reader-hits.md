# 0363: Specification lines a site reader hits

Status: landed. Lane claude-2. Branch `ticket/0363-spec-lines-site-reader`. Plan: batch C2 of `sdlc/planning/issue-priorities-2026-09-30.md`. Closes `sdlc/issues/2026-09-30-settings-table-row-and-recording-page-a-site-reader-hits.md`.

## Outcome

The site renders `specification/settings.md` word for word, and every line of it reads as text for a user. The page cites no ticket number in a cell's meaning, says no "later", "now", "continues" or "remain unproved", and links no issue as a future answer. Its jargon and missing full stops are gone, and it names things as the site's name table does. `recording.md` says "Plan previews" in place of "Dry runs". `recording.md` and `result.md` say that `cache_answers` counts answers from the answer cache only, never from a replay or record folder. `settings.md` and `recording.md` agree with ticket 0360 on where the usage totals live. No code changes.

## Evidence

- Starts from: main `e6e57e3c6`, after 0360 landed (`dda7a13cb`).
  - The issue lists five items, checked again on this commit. Item 4's 23 findings all remain, at the lines below.
  - `settings.md:67`, the "Portable call settings (`thinkthen.settings/1`)" row: code marks in the name cell, "tickets 0284–0298" in its meaning, and "later" or a ticket number in seven surface cells. Read on main: the SQLite, PostgreSQL and DuckDB calls take the object as their call settings JSON argument (`databases/sqlite/src/question.rs`, `databases/postgresql/src/forms.rs`, `databases/duckdb/bridge/src/ffi/portable*`). Python and R check their question and call keywords by its rules (`libraries/python/src/asked.rs:97`, `libraries/r/thinkthen/src/rust/src/ffi/settings.rs`). The C door's `thinkthen_plan_json` takes it as `settings` (`libraries/c/include/thinkthen.h:227`). TypeScript and Ruby do not use it. Rust exposes `Settings::parse`, `check` and `conflicts`.
  - `settings.md:88`, the Deadline row's Rust cell ends "remain for source callers until ticket 0291". 0291 has landed, and `deadline_seconds` and `deadline_millis` still exist (`public/options.rs:261,302`).
  - `recording.md:30` says "Dry runs"; the flag is `--plan` and the setting is "Plan preview".
  - `recording.md:76` says `--facts` counts `cache_answers` as questions answered from the store. The code sets the count only under `--cache`, `THINKTHEN_CACHE` or the default cache (`cli/asking/folders.rs:40-52`), because the same count feeds the usage totals. `recording.md:36` already says so for the usage totals. `result.md:155` names the count without its rule. Coordinator default 8 keeps the code.
  - `settings.md:31`, `settings.md:80`, `recording.md:46` and `backends.md:63` call two issues "the future answers for a limit across processes" and link them.
  - `settings.md:31` cites "ticket 0360" beside the usage path; `recording.md:34` holds the 0360 rule. The two agree on the folder.
  - The site's settings check `site/scripts/check-settings.mjs:43` fails when its `LEFT_OUT` list names a row the table lacks, and `check-words.mjs:87` fails on an `ALLOWED` entry no hit matches. Both are marketing's, and the site deploy runs only by hand.
- Keeps: every setting's name, default, allowed values, surface spelling and source, except the Portable call settings row's name and cells. The table's columns and row count. `sdlc/scripts/settings` passes. Every rule each sentence states, reworded only. No code, test or binding changes.
- Changes: `specification/settings.md`, `specification/recording.md`, `specification/result.md`, and one sentence of `specification/backends.md`.
  - The Portable call settings row reads for a user: name "Portable call settings", what it does, the closed key list and its refusals, and each surface's real spelling or `not on this surface`.
  - The Deadline row's Rust cell drops the ticket clause and says the two older setters still work.
  - Item 4's fixes: three full stops; the writable-file paragraph says a file holding `backends` is refused with exit 5; the status, changelog and issue-link sentences become plain present-tense rules or go; the jargon in the SQL paragraph, Run facts, Plan preview, Private TLS roots, Throttle, Process request total, Estimated input admission total, Deadline, Answer cache and Prune preview rows is rewritten; "platform cache folder" becomes "default answer cache folder"; the Answer cache row stops mixing the recording words; the requests-a-minute rule stays only in its row.
  - `recording.md:46` drops the future-answers sentence and points to the Requests a minute rule in `backends.md`. `backends.md:63` drops the same sentence and its issue links.
  - `recording.md:30` says "Plan previews". `recording.md:76` and `result.md:155` state the `cache_answers` rule.
  - The settings paragraph points to `recording.md` for the usage folder, without the ticket number.
- Proof: `sdlc/scripts/settings` and `sdlc/scripts/spec`'s page checks pass. A search of `settings.md` finds no "later integration", "until ticket", "tickets 0284", "now include", "retains", "continues", "remain unproved", "future answers", "envelope", "atomic", "morsel", "ABI", "receipt", "provider backoff" or "platform cache folder", and `recording.md`, `result.md` and `backends.md` lose "future answers" and "Dry runs". The site's `check-settings.mjs` runs read-only to list what the site must change, and that list goes to marketing with the landing.
- Defers: the site's checks go red on main from this landing until marketing's edit. `site/scripts/check-settings.mjs:43` fails because `site/src/lib/settings-table.mjs:150` names the old row name, and `site/scripts/check-words.mjs:42-43` fails on the "planned cached exchange" entry once the Answer cache row loses that phrase. The Plan preview row keeps "whole-input records" and "planned requests", so its entry still matches. Marketing's own message (`repos/sdlc/inbox/thinkthen/2026-09-30-settings-table-wording-on-the-site.md`) asks for exactly this order: change the table, reply, and the site drops its entries. The site deploys only by hand, so nothing published breaks. The landing answers that message the same hour, naming the new row name and both lines. The opening paragraph's `sdlc/scripts/settings` line is not in the site's findings and stays.

## What the build taught us

- The issue named 23 wording findings, and a full search found two more of the same kind: "future answers" also sat in `recording.md:46` and `backends.md:63`, and "Dry runs" also sat in `backends.md:28`. A finding about one page is worth a search of its siblings.
- The site's parser reads the new table: 62 rows, and its one stale entry is the old Portable call settings name, as Defers says. The site's own check needs a built site, so the builder ran the parser alone.
- A lane can hold another agent's edit. A `git commit -a` swept a stranger's test change into this branch; the branch was rebuilt from the ticket alone. Stage named files only.
