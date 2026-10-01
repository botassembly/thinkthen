# A blank THINKTHEN_MAX_REQUEST_BYTES exits 2

Status: closed 2026-09-30 by ticket 0364. Reported by the site team on 2026-09-30 and confirmed on main `d75a4bdf1`. Owner: the queue owner, as batch C3 in `../planning/issue-priorities-2026-09-30.md`, after C2 lands. Resolution: A blank value counts as unset, like every other variable, and `specification/settings.md` says so. The site's Configuration page can drop the exception.
Kind: bug

`THINKTHEN_MAX_REQUEST_BYTES= thinkthen decide "Is it?" --plan` exits 2 with "THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1". Every other variable treats a blank value as unset. `crates/thinkthen/src/cli/edge.rs:113` reads it with `env::var(...).ok()`, while the others go through `read()` (`edge.rs:248-250`), which drops blank values. `request_size` (`edge.rs:183-196`) then refuses the empty string. `specification/settings.md:83` says nothing about blank values, while the Batch row says an empty `THINKTHEN_BATCH` counts as unset.

Fix: read it through `read()`, and say in `specification/settings.md` that a blank value counts as unset. Pin the blank case in the edge test. Tell the site team when it lands, because the site's Configuration page names this variable as the one exception.
