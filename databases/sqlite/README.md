# The sqlite surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the extension. The
sample: thinkthen_warm fills the answer cache in one 32-wide pass, then WHERE thinkthen_decide reads it row by row.

The SQL functions carry the `thinkthen_` prefix, `NULL` is "not sure",
and the question file names itself as `'@refund.json'`. All three
databases ship `thinkthen_warm`.
