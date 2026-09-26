# Settings table gaps a site reader hits

Status: open. Filed 2026-09-26 by the marketing lead.

The site's Settings page (`/install/settings/`) reads `specification/settings.md` at build time. Building it from table `a14d959e` showed these gaps. Each one leaves a user unable to answer a plain question from the page.

## Problems

1. **Key on the libraries.** Row 51 says "not on this surface" for the Python, TypeScript, Ruby and R libraries and for SQL. Only Rust names a setter. A Python user cannot learn where the key goes. The row should say whether each library and each database extension reads `THINKTHEN_API_KEY`.
2. **Address default.** Row 50 gives the default as "The built-in address" and never names it. A reader cannot see where requests go by default.
3. **Default cells mix three things.** A Default cell holds the value, the source links and notes such as "`either` defaults to false", "Fixed at 30 on the libraries and SQL", "SQL cannot name one" and "`cache_bytes` has no effect". The site has to split them by pattern. A separate source column and notes placed in the surface cells would remove the guessing.
4. **Citations a public reader cannot follow.** Cells cite `sdlc/issues/` paths, `databases/*/README.md`, "tickets 0109 decision 2", "Ian's ruling of 2026-09-21", "Batching design ruling 1", ticket names such as B12a, and a bare "ADR 0033" and "batching design's section 6" in Precedence. A public page needs a rule for which sources show.
5. **Settings on the way.** "The hard cap" has no option, so it is not a setting. The `window` designed default mixes in its allowed range. The `prefixes/suffixes/trim/defaults` item points at a design table the reader cannot see. The items have no fixed format, so the site finds "The designed default is" and "Tickets" by pattern.
6. **Deadline and cancel.** "A positive time" gives no unit, and the unit differs by surface.
7. **Audit bar.** One row packs five flags. Its allowed values do not line up one to one with the flags.

## Proposed fix

Give each Default cell only the value. Add a Source column. Move surface notes into the surface cells. Give "Settings on the way" the same columns as the main table. Name the built-in address and the key's reach on every surface.
