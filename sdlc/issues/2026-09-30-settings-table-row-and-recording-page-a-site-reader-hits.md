# Two specification lines the site shows readers in their draft form

Status: open. Filed 2026-09-30 by the marketing lead. Owner: the queue owner.
Kind: bug

The site's Settings page renders `specification/settings.md` at build time. Two lines there and in `recording.md` read as working notes to a site reader.

1. **The "Portable call settings" row** (`specification/settings.md:67`). Its name cell holds code marks, `(`thinkthen.settings/1`)`, which the page prints raw. Its description cites "tickets 0284–0298" and its cells say "later". A reader cannot act on a ticket number or on "later". Until this row reads as user text, the site leaves it off the Settings page, and the site's settings check names it as the one skipped row.
2. **"Dry runs"** (`specification/recording.md:30`). The flag is `--plan` since 674a82041, and the Settings row is "Plan preview". The sentence should say "Plan previews".

Asked: rewrite the row for a reader with no ticket numbers or "later", and change the one word. Tell the marketing lead when it lands, so the site drops its skip.
