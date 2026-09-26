# Settings table cells the site still types

Status: open. Filed 2026-09-26 by the marketing lead after site commit 88580fbb.

The site now reads every setting value from `specification/settings.md` through one lookup. A missing setting fails the site build. Four values stay typed by hand, because the table does not give them in a form a page can use:

1. **Address default.** The Default cell says "The built-in address". The site types `https://api.typesafe.ai/v1` in `site/src/pages/reference.astro` (the `--url` row and `THINKTHEN_BASE_URL`) and in `site/src/pages/install/backends.astro`. Item 2 of `2026-09-26-settings-table-gaps-a-site-reader-hits.md` covers the same gap.
2. **Address allowed values.** "An `http` or `https` base the backend rule accepts" does not tell a public reader what is accepted. Please state the rule or link the spec section that states it.
3. **Threshold band.** The table does not say whether a band's low end may be 0. The site derives `0 ≤ LOW < HIGH ≤ 1` from the cut's bounds. Please state the band's bounds in the Allowed values cell.
4. **Framing.** "Or JSON lines when a pointer is set" sits inside the per-function clause of the Default cell. Please give it its own clause, so the site can read it.

When each cell changes, the site follows with no site edit, and the four typed spots can go.
