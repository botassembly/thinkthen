# ADR 0046: Wave-7 waivers

Date: 2026-09-23. Status: accepted by the wave-7 gate lane. Each waiver
below is a decision Ian can overturn. Each names its cost.

## R1-31: kind names are spelled in each binding

The error kinds (`usage`, `deadline`, `defect`, and the rest) appear as
strings in each binding's own source: the TypeScript addon and its
typings, the C door, the R shim, and others. The contract owns the kinds.
Each host needs them in its own idiom, as a class name, a constant, or a
condition class. The conformance cases and each surface's tests pin the
spelled strings, so a drift fails a test. A generated table would add a
build step to nine hosts for strings that change rarely.

Overturn: ask for one generated kind table. The cost is a generator and
a build step in every binding.

## R1-29 remainder: floating images in one-time setup

The gate uses no floating image. The PostgreSQL image is pinned by
digest. Three images still float by tag, and none runs in the gate:

- `ruby:3.4-trixie`, the base of the Ruby builder image
  (`libraries/ruby/Dockerfile`), used once when the image is built.
- `ubuntu:24.04`, the SQLite package rehearsal's container half
  (`databases/sqlite/package.sh`), run by hand.
- `duckdb/duckdb:1.5.5`, the DuckDB package rehearsal's container half
  (`databases/duckdb/package.sh`), run by hand.

Their digests were not resolvable offline at pinning time. Both package
scripts say so, and `scripts/gate-hermeticity.md` lists them.

Overturn: ask for digest pins. The cost is one networked `docker pull`
per image to resolve each digest.

## R4-20 and R5-33: the history is not rewritten

Some commits that moved the surfaces ceiling lack the review the rule
asks for. Rewriting published history would break every clone and
worktree of the branch. The commits stay as they are, and
`sdlc/surfaces-ratchet-reviews.json` names each one by SHA, so the
ratchet accepts no new commit of the same kind.

- Empty bodies: `e11bb57`, `4238ea5`.
- Raised with no review line: `b7344fa` and `4252283`. `db3d09f` and
  `47ca0c8` cite reviews for them after the fact.
- Review cited before the commit was written (R5-33): `78315ba` cites
  `2b63c87` and the fourth review (`aac114a`). `d9cc3db` cites the fourth
  review's probes. `252f530` cites the review of the must-equal rule.
- The other historic moves in the list wrote their review in prose
  before the structured `Second-agent review:` line existed, or had no
  review line.

Overturn: ask for a history rewrite. The cost is a force-push of the
surfaces branches and a rebuild of every open worktree.
