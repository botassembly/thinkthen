# The duckdb surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the extension. The
sample: WHERE thinkthen_decide('Does the customer ask for a refund?', body); and choose with ['billing','shipping','account'], with the question named as '@refund.json'.

The SQL functions carry the `thinkthen_` prefix, `NULL` is "not sure",
and the question file names itself as `'@refund.json'`. All three
databases ship `thinkthen_warm`.
