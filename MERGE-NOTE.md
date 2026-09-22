# Merge note: the `surfaces` branch

For the build team, from the library team, 2026-09-21. The branch is
`surfaces` in `/home/ian/workspace/worktrees/thinkthen-surfaces`; the
rulings it obeys are in `sdlc/issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`.
Each section below is one merge-ticket decision or edit. The branch never
touched main; everything that needs both sides is here.

## 1. The conformance file: one name, one union, the defect case kept

Main owns `conformance/cases.json` (27 cases, consumed by
`crates/thinkthen-core/tests/conformance.rs`). The branch owns
`conformance/conformance.json` (72 cases; validator at
`conformance/tools/validate_conformance.py`; divergence record at
`conformance/DIVERGENCES.md`). Per ruling 2: the build team picks the one
name and owns the union; main's `25-defect-fault` stays, marked engine-only,
and no public door gets a fault hook; each surface's binding tests already
prove the `defect` kind maps to the host's own error (the branch carries
those per-surface unit tests). The branch's 72 cases and the divergence
list are handed over as-is.

## 2. Stale planning text on main

- `sdlc/planning/databases/README.md:49` and `postgres.md:75`: "the 1 GiB
  cap and the prune ship with the location" — the ruling is 100 MB.
- `sdlc/planning/adr/0017-libraries-over-one-bound-core.md:132` attributes
  the superseded rule-5 wording ("the disk cache waits for a named folder")
  to `databases/README.md`, which no longer says that.
- `sdlc/planning/relate-design.md:78` still shows `(name, from_id, to_id,
  probability)`; `:86` still lists the settled `probability`/`confidence`
  question as open. The one-rule page and the ruled `source`/`target`
  replace both.
- The recognize-and-relate update issue's closing says "four findings, all
  filed" and lists an item with no file; the rulings issue's closing says
  "three sample findings filed". Correct both sentences to the record's
  actual locations (three mktg files; one in-branch fix; one lane note).

## 3. The rulings issue was edited on both sides

`sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md`
has the branch's Phase A/B record on one side and main's Polars ruling and
"The surfaces experiment closes" on the other. A merge taking either side
whole deletes the other's record; hand-merge keeping both blocks.

## 4. The gate ladder, after the merge

The branch adds its rung now: `sdlc/scripts/surfaces` runs the offline
checks (generated name lists, public names, the conformance validator)
always, and `scripts/check_surfaces.sh` when the toolchains are present.
Bringing `contract/`, `standin/`, `libraries/`, and `databases/` under
main's fmt, clippy, deny, and the source ratchet is the merge ticket's,
because `sdlc/ratchet.json` counts `crates/**/*.rs` only and the ceiling
moves. The public-name check (`scripts/check_public_names.py`, wired into
`check_surfaces.sh` and the rung) is what enforces ruling 4 after the
merge; it fails on any public name that is not ruled or documented.

## 5. Items other teams own

- The recognize team: `experiments/225-recognize-harvest-package/README.md`
  says "20 checks"; `rules/tests.py` has 25 `check(` sites.
- PostgreSQL's packaged `.so` is not glibc-pinned; the pin or the stated
  floor is the build team's call.
- macOS artifacts still owed: TypeScript, Ruby, SQLite, DuckDB,
  PostgreSQL (C and Python cross-built real artifacts). Each blocker and
  the Mac's ready command is in `NOTES-packaging.md`. The R user-installed
  SIGINT handler case stays unchecked.
- The deck: three open findings in `repos/mktg/sdlc/issues/` (`located_in`
  has no recording; the DuckDB `relate` subquery cannot bind; the Ruby
  score comment pinned a distribution). The `thinkthen_relations` finding
  from ruling 9 is filed in this repository
  (`sdlc/issues/2026-09-21-the-thinkthen-relations-call-cannot-run-as-drawn.md`)
  with each engine's working shape.

## 6. Things the ADR and the real engine must state

- **The interrupt shape** (ruling 3): an interrupt or a spent deadline
  surfaces within one poll tick, however fast the backend answers; the
  poll runs from the answer arms too, gated by one tick. The regression
  test is `libraries/python/tests/test_cancel_fast.py` plus the Rust
  test; it fails in 2.87 s with the busy-arm ticks dropped and passes in
  0.24 s with them. Each binding's channel into that poll is named:
  SQLite's progress handler, PostgreSQL's interrupt check,
  `AbortSignal`, R's tick, and DuckDB's chained SIGINT-at-LOAD handler
  (proven inside a Python process, 0.11 s stop with the wire frozen).
  ADR 0017 copies this shape only.
- **The usage counter** (ruling 5): counts what left the machine. A sent
  retry counts again; a cache hit, a size refusal, and a usage error count
  nothing; tokens come from the vendor's replies only.
- **The fork preconditions** (ruling 10): the settings lock is read
  before the fork, and the child holds the inherited pool's file
  descriptors for its lifetime. Both are stated in the stand-in's docs.
  Optional real-engine improvement, not a constraint: close the inherited
  descriptors in the rebuild and the second precondition disappears.
- **`strength`** is the settled name on a recognized name (ours,
  computed, parts under details); a relation's number is `probability`;
  the vendor's `confidence` appears under details only. The generator was
  fixed and rerun (ruling 7).

## 7. The two contracts the branch extended

The question file's relation ends are `source` and `target` only
(ruling 1): the parser refuses `from`/`to` with the ruled spelling named,
PostgreSQL's dual-accept is gone, and the conformance questions carry the
ruled keys. `relate` refuses more than 255 records with a usage error at
every door. The C door's `thinkthen_call` doc line and the two functions'
return-code docs were corrected to the codes the door actually returns.
