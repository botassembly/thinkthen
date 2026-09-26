# Settings table gaps a site reader hits

Status: open. Filed 2026-09-26 by the marketing lead. Item 8 added 2026-09-26 by the queue owner from local experiment 273, reports 06 (I-5) and 08 (4), for backlog ticket H4.

The site's Settings page (`/install/settings/`) reads `specification/settings.md` at build time. Building it from table `a14d959e` showed these gaps. Each one leaves a user unable to answer a plain question from the page.

## Problems

1. **Key on the libraries.** Row 51 says "not on this surface" for the Python, TypeScript, Ruby and R libraries and for SQL. Only Rust names a setter. A Python user cannot learn where the key goes. The row should say whether each library and each database extension reads `THINKTHEN_API_KEY`.
2. **Address default.** Row 50 gives the default as "The built-in address" and never names it. A reader cannot see where requests go by default.
3. **Default cells mix three things.** A Default cell holds the value, the source links and notes such as "`either` defaults to false", "Fixed at 30 on the libraries and SQL", "SQL cannot name one" and "`cache_bytes` has no effect". The site has to split them by pattern. A separate source column and notes placed in the surface cells would remove the guessing.
4. **Citations a public reader cannot follow.** Cells cite `sdlc/issues/` paths, `databases/*/README.md`, "tickets 0109 decision 2", "Ian's ruling of 2026-09-21", "Batching design ruling 1", ticket names such as B12a, and a bare "ADR 0033" and "batching design's section 6" in Precedence. A public page needs a rule for which sources show.
5. **Settings on the way.** "The hard cap" has no option, so it is not a setting. The `window` designed default mixes in its allowed range. The `prefixes/suffixes/trim/defaults` item points at a design table the reader cannot see. The items have no fixed format, so the site finds "The designed default is" and "Tickets" by pattern.
6. **Deadline and cancel.** "A positive time" gives no unit, and the unit differs by surface.
7. **Audit bar.** One row packs five flags. Its allowed values do not line up one to one with the flags.
8. **The configuration file's `cache` is wrong on the page.** Row 63, "Answer cache", gives the allowed values as "A folder, or off" and lists the configuration file's `cache` in the same row. Line 19 says the file's `cache` sits in the folder order. The code takes only true or false (`crates/thinkthen/src/config.rs:36`, `cache: Option<bool>`). A file with `{"cache": "/some/folder"}` stops every command at exit 5, `status` included, with `the configuration file is not valid closed JSON`. The message names no field. H4 corrects the row and line 19 to say the file turns the default cache on or off. The closed-JSON refusal should name the field, which is a code change H4 can carry or hand to a Quick Fix. Verified on main by reading `config.rs:36` and `settings.md`. The exit 5 runs come from the reports.

## Proposed fix

Give each Default cell only the value. Add a Source column. Move surface notes into the surface cells. Give "Settings on the way" the same columns as the main table. Name the built-in address and the key's reach on every surface.
