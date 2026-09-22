# The postgresql surface

The extension is landed. Twelve SQL functions over the one engine —
`decide`, `probability`, `choose`, `score`, `tag`, `annotate`, `details`,
`usage`, `warm`, and `recognize`, `relate`, and the beta
`thinkthen_relations` — plus the array overload of `thinkthen_decide`, the
second bulk form `postgres.md` names. Every function is `PARALLEL
RESTRICTED`; `NULL` is "not sure"; a question file names itself with the
command's spelling, `'@refund.json'`, resolved against the backend's working
directory; the six error kinds map to six SQLSTATEs with the retry signal in
the message. The width (the engine's `width` setting or `ENGINE_WIDTH`) is
the number of requests in flight, and each in-flight request holds its own
connection: 1,000 records at width 32 measured 33 pooled connections.

`recognize` returns the five ruled columns `(text, kind, start, end,
strength)`, used with `LATERAL`; `start` and `end` count characters, so
`substring(text from start + 1 for end - start)` is the name. `relate`
takes a query (the record id first, its text second) and returns
`(name, source, target, probability)` with the id values, so the edges join
back to the query's table; more than 255 records is a usage error.
`thinkthen_relations(body, '@names.json')` is the beta companion returning
the relations as rows.

The acceptance sample from
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md` runs
as drawn: the `IS NULL` query returns the `maybe` row a person should read,
and `thinkthen_annotate('form.json', body)` orders by urgency 1.7 / 1.05 /
0.99. The `recognize` and `relate` calls in
`recognize-surfaces.md` run as drawn too: the LATERAL call, the names-become-
rows pattern with the no-request join, and `thinkthen_relate('SELECT id, body
FROM alerts', ARRAY['caused_by'])`.

`check.sh` packages the extension, proves it in a disposable `postgres:16`
container on the host network (picking a free port itself, because sibling
sessions race fixed ones), runs the slide sample, the offline conformance
slice through `runner.py`, the recognize/relate acceptance, the 255-record
refusal, and the wire suite when the loopback stub is up on 8219, and
removes the container afterwards. The running log is `NOTES.md`.

The known divergences: the stand-in never credits its in-process memory in
`cache_answers` (its own record says so; a real-engine requirement). Case
71's per-subject arm shares its input with the pairs arm and the stand-in
serves the ruled pairs form: a conformance-data finding for the build team.
The contract's settled `nearest` field rides `thinkthen_details` here like
every other member — PostgreSQL serializes the contract's own `Details`
struct, so the field needed no wiring in this shim — and the conformance
slice proves it: case 13's `nearest_level` (`mid`) is compared against
`(details->>'nearest')` in `runner.py`.
