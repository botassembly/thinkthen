# The R surface

Landed 2026-09-21. The package `thinkthen`: the eight verbs and the two
newer functions (`recognize`, `relate`) with a `tt_` prefix, a column in and
a column out, `NA` as "not sure", and the six error kinds as R conditions
carrying the retry signal. One engine under the contract, built with extendr
from `contract/` and `standin/`, installed into the folder-local `rlib/`.
Run `./check.sh` for the null suite, the conformance slice, the recognize
and relate acceptance, the slide sample, and the wire suite when the stub is
up on 8215. Findings and quirks are in `NOTES.md`. The width (the engine's
`width` setting or `ENGINE_WIDTH`) is the number of requests in flight, and
each in-flight request holds its own connection: 1,000 records at width 32
measured 33 pooled connections.

`tt_recognize(body, kinds)` returns a list column of data frames, one row per
name under `tidyr::unnest()`; a relation's ends are `source` and `target`,
and a name's computed number is its `strength`. `tt_relate(records,
relations, either)` returns a data frame of edges ready for a graph library.
Both run as written in the acceptance page, offline from the recordings.

The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`,
runs as drawn; `slide.R` holds the drawn block untouched and
`slide_check.R` proves it. Against the null backend the whole pipeline
runs, including the `choose` and `score` columns. Against the one-probability
stub the `decide` column and the crossing prove on the wire; the `choose`
and `score` columns cannot, because the stub answers one noul a request and
the wire shapes for those verbs need per-option distributions. That is a
finding against the stub's coverage, recorded in `NOTES.md`, not a change
to the slide.

## Building and installing

```sh
cd thinkthen
R CMD INSTALL -l ../rlib .    # configure runs cargo; rlib/ is folder-local
```

Nothing is published: no CRAN, no R-universe, no gem, no npm. The package
version follows the repository's one version number.
