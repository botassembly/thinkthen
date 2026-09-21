# The R surface

Landed 2026-09-21. The package `thinkthen`: the eight verbs with a `tt_`
prefix, a column in and a column out, `NA` as "not sure", and the six error
kinds as R conditions carrying the retry signal. One engine under the
contract, built with extendr from `contract/` and `standin/`, installed
into the folder-local `rlib/`. Run `./check.sh` for the null suite, the
conformance slice, the slide sample, and the wire suite when the stub is up
on 8215. Findings and quirks are in `NOTES.md`.

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
