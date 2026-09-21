# The adversarial review of the surfaces and the experiments

Status: Open. Written 2026-09-21 by the library team's final lane. Ian asked
for a thorough adversarial review of all the experiments and all the
library work; three independent lenses ran read-only against pinned
snapshots and the orchestrator settled the doubts it could by command. This
page is the synthesis; the three full lens reports are preserved verbatim
as records, so they outlive their temporary files:

- `sdlc/records/2026-09-21-adversarial-review-implementation.md` —
  the `surfaces` branch at `0801a32`: code, tests, benches, packaging.
- `sdlc/records/2026-09-21-adversarial-review-experiments.md` —
  experiment 211 and the job 2/3 records: claims against commands.
- `sdlc/records/2026-09-21-adversarial-review-records.md` —
  the paper: ADR, rulings, findings pages, cited hashes and paths.

## Verdict

The work is honest and its headline numbers trace to commands, and four
findings blocked a clean merge or an ADR copy. All four are settled under
the product rulings of 2026-09-21
(`sdlc/issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`),
and this lane's fixes carry the numbers below.

## The one confirmed defect, by command

**The interrupt promise.** The poll ran only on idle waits, so a fast
backend starved it: SIGINT one second into a three-million-record null
batch surfaced at 8.48 s, after the batch drained — reproduced by the
orchestrator, predicted independently by the experiments lens from a single
call site. The fix runs the poll from the answer arms too, gated by one
tick; the same probe now surfaces at about 0.8 s, and the regression test
fails in 2.87 s with the busy-arm ticks dropped and passes in 0.24 s with
them. The promise ADR 0017 copies: an interrupt or a spent deadline
surfaces within one poll tick, however fast the backend answers.

## The contradictions the rulings settled

1. **The question-file spelling.** The shipped parser required `from`/`to`
   while the ruling said `source`/`target` on every surface including the
   question file; eight doors refused the ruled spelling and PostgreSQL
   quietly accepted both. Ruling 1 refuses `from`/`to` outright; the parser
   now names the ruled spelling in its error, PostgreSQL's dual-accept is
   gone, and the conformance questions carry the ruled keys.
2. **The gate.** The whole surfaces check ran from no rung, so surface rot
   would pass every repository gate, and fmt, clippy, deny, and the ratchet
   never saw the new folders. Ruling 6 adds `sdlc/scripts/surfaces` on the
   branch: the offline checks always, `scripts/check_surfaces.sh` when the
   toolchains exist, the skip named when they do not; the lint-and-ratchet
   extension is a merge-ticket item because the ceiling moves.

The review's other findings — the stale generator key (one run from failing
the tree's own validator), the counter comments, the fork preconditions,
the overstated claims, stale records, R's exported internals, the missing
relations finding — are fixed or recorded under rulings 4 through 10.

## What the review confirmed strong

Every cited commit resolves; the conformance arithmetic is exact and the
validator proves replay rather than schema; the offsets are right in all
four indexing worlds including the emoji case per host; the 255 refusal
holds at every door; the atomic-slot fork reasoning is sound for locks, and
the standby's remaining preconditions are now stated rather than implied;
the engine holds no async anywhere; the history is linear and the probe
commits were kept as the measurement record; both lanes recorded their own
`git add -A` incidents unprompted.

## What the review corrected in its own premise

There were two sweep incidents, not one: TypeScript swept Python's files
and the C lane swept R's. Both aftermaths were checked file by file: no
duplicated or lost work anywhere.

## What Ian can overturn

All of it. The cheap one: nothing here — the rulings were his, and this
page only records what the wave did with them.
