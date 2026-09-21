# The postgresql surface

The extension is landed. Nine SQL functions over the one engine — `decide`,
`probability`, `choose`, `score`, `tag`, `annotate`, `details`, `usage`, and
the `warm` aggregate — plus the array overload of `thinkthen_decide`, the
second bulk form `postgres.md` names. Every function is `PARALLEL
RESTRICTED`; `NULL` is "not sure"; a question file names itself with the
command's spelling, `'@refund.json'`, resolved against the backend's working
directory; the six error kinds map to six SQLSTATEs with the retry signal in
the message.

The acceptance sample from
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md` runs
as drawn: the `IS NULL` query returns the `maybe` row a person should read,
and `thinkthen_annotate('form.json', body)` orders by urgency 1.7 / 1.05 /
0.99.

`check.sh` packages the extension, proves it in a disposable `postgres:16`
container on the host network (picking a free port itself, because sibling
sessions race fixed ones), runs the slide sample, the offline conformance
slice through `runner.py`, and the wire suite when the loopback stub is up
on 8219, and removes the container afterwards. The running log is
`NOTES.md`.

The known divergences: the stand-in never credits its in-process memory in
`cache_answers` (its own record says so; a real-engine requirement), and
`details` on a score question carries no nearest level until the contract
gains the field.
