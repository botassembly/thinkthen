# The surfaces

How the six language libraries and the three database extensions live in
this repository, per ADR 0017 and the rulings of 2026-09-21. One repository,
one version number (`VERSION`, and 0.1.0 is the first release), one engine
under ten surfaces.

## The layout

| Path | Holds |
| --- | --- |
| `contract/` | The written interface both engines meet: the Rust trait, the public types, and `include/thinkthen.h`, the C door |
| `standin/` | The stand-in engine, implementing the contract today. The real engine replaces it with one changed dependency |
| `libraries/<language>/` | One folder a language: `python`, `typescript`, `ruby`, `r`, `rust`, `c` |
| `databases/<engine>/` | One folder a database: `duckdb`, `sqlite`, `postgresql` |
| `conformance/` | The one conformance file every surface reads, its validator, and the divergence record |
| `scripts/check_surfaces.sh` | The one script that builds and checks everything landed |

A surface binds `thinkthen-contract` and never the engine beneath it. The C
surface owns every exported symbol; the engine exports none.

## The acceptance test

The slide code in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md` is
the acceptance test. Each sample runs as drawn against the stand-in and
gives the answer in its comment. A sample that cannot work as drawn is a
finding: report it, and the slide changes. Nothing in a surface may rename
a verb, add one, or spell a bulk form the contract does not carry.

## How to add a surface

1. Make `libraries/<language>/` (or `databases/<engine>/`) with a README
   naming its slide sample.
2. Bind the contract: convert the host's arguments into a `Question` or a
   question-file string, call one engine function, convert the result, and
   map the six error kinds to the host's own errors. No rule, no retry, and
   no sending lives in the host.
3. Run the slide sample as drawn against the stand-in and the surface's
   slice of the conformance file, in `check.sh` inside the folder. The
   script in `scripts/` picks it up when it is executable.
4. The host's own empty value is "not sure": `None`, `null`, `nil`, `NA`,
   `NULL`. `Answer::Unsure` on the Rust surface, `THINKTHEN_UNSURE` at the
   C door.

## How to add a function

1. Add the verb's shape to `contract/` first: the trait method, the public
   types, the C door when the hot path needs it. The contract's doc rules
   are ADR 0017's.
2. Implement it on the stand-in, with the verb's rule (which thresholds it
   takes) enforced at the engine.
3. Add its cases to `conformance/conformance.json`, once, in the file's own
   grammar. Every surface inherits them by reading the one file.
4. Each surface grows the verb in its own spelling of the same name. If the
   repeated parts across surfaces feel heavy, generate them from one table
   of functions: name, inputs, output, and the help line — the
   maintainability test in the surfaces brief says when.

## What the check script runs

`scripts/check_surfaces.sh` builds and tests the contract and the stand-in,
runs the stand-in's wire tests when the loopback stub is up (and says so
when it is not), validates the conformance file offline, and calls each
landed surface's `check.sh`. Run it from anywhere; it finds the repository
root itself.
