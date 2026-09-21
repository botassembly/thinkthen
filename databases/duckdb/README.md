# The duckdb surface

The nine SQL functions over the one engine, per ADR 0017 and the database
pages: `thinkthen_decide`, `thinkthen_probability`, `thinkthen_choose`,
`thinkthen_score`, `thinkthen_tag`, `thinkthen_annotate`,
`thinkthen_details`, `thinkthen_usage`, and `thinkthen_warm`. `filter`,
`rank`, and `find` get no functions; `WHERE`, `ORDER BY`, and `LIMIT` are
those verbs. `NULL` is "not sure", and a failure is an error that never
reads as `NULL`.

The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`,
runs as drawn against the stand-in: `tools/run_slide.sh` extracts the
slide's code block verbatim and runs it in the stock CLI. One finding for
the slide's owner sits in NOTES: the sample's `choose` column reads NULL
against the offline stand-in (its options carry no keyword the offline
rules can lift) and the loopback stub cannot answer a choice question at
all, because it answers one probability per request. The shape is proven;
the values await a backend that distinguishes options.

## The ruled shape this surface takes

- A question argument is plain text under the grammar's default cut, a
  file named `'@refund.json'`, or the file grammar's own JSON. One door,
  one grammar; the surface adds no parser.
- A verb whose members ride in a `LIST` argument — `choose`, `score`,
  `tag` — takes its question as plain text and its members from the list.
  A file or JSON question beside a list is a usage error, because the
  members would come twice.
- `thinkthen_score` returns the specification's position from 0 to K−1;
  the nearest level's name rides in `thinkthen_details.level`.
- `thinkthen_details` is one struct for every verb. For a decide it
  carries the probability, the answer's word, the model, the digest, and
  the sends that produced the judgment. For the other verbs the
  yes-probability does not exist, so probability, answer, and sends read
  NULL and the question's own model and digest carry the audit; a score
  question also names its nearest level.
- `thinkthen_annotate` returns a JSON object, one field per question in
  the set's name order, because a scalar's return type is declared at
  registration and a set's questions are not known there. DuckDB's own
  JSON functions read the object.
- `thinkthen_warm(question, text)` is the aggregate: it collects the
  distinct texts a scan sees and judges them once through the engine's
  batch door.
- Every call carries the process-wide cancel token, and the extension
  takes SIGINT at LOAD and chains to the CLI's own handler, so a Ctrl-C
  stops between requests. The Python-process proof is on the planning
  page; nothing new was claimed here.

## Build and run

The version pin is the trap 207 found: the Rust path builds against the
unstable C API, so the extension and the CLI must be the same version.
`TARGET_DUCKDB_VERSION=v1.5.5` in the Makefile, and the stock v1.5.5 CLI
in `duckdb-bin/` (fetched user-level; the machine CLI is older and cannot
load what this path builds).

```
make configure && make release
./duckdb-bin/duckdb -unsigned -c "LOAD '$PWD/build/release/thinkthen.duckdb_extension'; \
  SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today');"
./check.sh
```

Nothing published. No container needed for the checks; the wire suite
wants the loopback stub on port 8217.
