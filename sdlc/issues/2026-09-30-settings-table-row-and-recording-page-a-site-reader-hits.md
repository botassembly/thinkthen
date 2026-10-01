# Specification lines the site shows readers in their draft form

Status: open. Filed 2026-09-30 by the marketing lead. Owner: the queue owner, as batch C2 in `../planning/issue-priorities-2026-09-30.md`, after ticket 0360 lands.
Kind: bug

The site's Settings page renders `specification/settings.md` at build time. Three lines there and in `recording.md` read as working notes to a site reader.

1. **The "Portable call settings" row** (`specification/settings.md:67`). Its name cell holds code marks, `(`thinkthen.settings/1`)`, which the page prints raw. Its description cites "tickets 0284–0298" and its cells say "later". A reader cannot act on a ticket number or on "later". Until this row reads as user text, the site leaves it off the Settings page, and the site's settings check names it as the one skipped row.
2. **"Dry runs"** (`specification/recording.md:30`). The flag is `--plan` since 674a82041, and the Settings row is "Plan preview". The sentence should say "Plan previews".
3. **The "Deadline and cancel" row's Rust cell** (`specification/settings.md`). It ends "remain for source callers until ticket 0291". Ticket 0291 has landed. The site drops a cell clause that cites a ticket. Rewrite the clause for a reader. Merged on 2026-09-30 from `closed/2026-09-30-site-fixtures-converted-and-plan-examples-moved.md`.

4. **The settings table's wording** (`specification/settings.md`). Added 2026-09-30 from the site team's plain-writing read in site ticket 0040. On main `d75a4bdf1`, 23 of its 25 findings remain. Two are fixed: the doubled stop after "then `max`" and the ",." after the named-backends link. The rest:
   - Missing full stops after "both ways" (line 56), "named" (line 69) and "request" (line 74).
   - The writable-config warning paragraph (line 31) says the command still reads the file. It omits that a file holding `backends` is refused with exit 5 (`specification/backends.md:61`).
   - Project status and links into the repository's records: "Native Intel hardware and macOS 15 remain unproved" (line 15), and "are the future answers for a limit across processes" with `sdlc/issues/` links (lines 31 and 80).
   - Changelog wording: "Library `usage` results now include `retries`", "SQL usage output retains its existing shape", "PostgreSQL continues to use native `statement_timeout`".
   - Jargon: "answered details envelope or a typed, safe failed envelope", "An atomic attempted-send count is shared within the process", "charged `ceil(bytes × 908 / 1000)`", "`jev-latest` refreshes every planned cached exchange", "Bare explicit prune still commits.", "old typed names remain bare ABI forms ...", "prompt stops carry a completion receipt", "a fresh deadline for each evaluated morsel", "provider backoff" (the site says backend), "Replace Mozilla roots", "measured token band".
   - Names that differ from the site's name table: "platform cache folder" for the answer cache, and "`--cache DIR` also replays and records into DIR". The requests-a-minute rule appears twice, at its row and in the opening notes.
5. **`--facts` and replay** (`specification/recording.md:74`, `specification/result.md:155`). Added 2026-09-30 from site ticket 0039. A run answered wholly from a `--replay` folder prints `"cache_answers":0`, while `meta.cached` is true. The code counts only answers from the answer cache on purpose (`cli/asking/folders.rs:40-52`, `engine/call_facts.rs:68`), because the same count feeds the usage totals. Coordinator default, which Ian can overturn: keep the code, and say in both files that `cache_answers` counts answers from the answer cache only, never from a replay or record folder.

Asked: rewrite the row for a reader with no ticket numbers or "later", change the one word, and rewrite the Rust cell's clause without the ticket number. Edit the settings table for a site reader, and reword the `cache_answers` sentences. Tell the marketing lead when it lands, so the site drops its skip and any `ALLOWED` word entry that no longer matches.
