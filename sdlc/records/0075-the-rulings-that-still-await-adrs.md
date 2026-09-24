# 0075: The rulings that still await ADRs

The third review asked for the list of rulings living only in records and
notes, finalized for the ADR owner. ADR 0037 (the C door) and 0038 (DuckDB
relate) already consumed their rulings. This is what remains, where each
lives today, and what its ADR must decide. None of these is blocked on the
other; each is small.

Update, 2026-09-23 (surfaces-review-7 R2-29): ADR 0041 took item 1 and ADR 0043 took item 2. Ticket 0099 ported both to main. ADR 0042 records the R interrupt window from record 0074. Items 3 and 4 went to branch ADRs 0044 and 0045, which stay at tag `surfaces-wave7-final`.

1. **One deadline spelling everywhere.** `-1` is the only no-deadline
   spelling on every surface; every other negative refuses; zero is spent;
   computed budgets clamp to zero. Lives in
   `sdlc/records/surfaces-notes/NOTES-rulings-wave.md`. The ADR decides the
   public rule and whether the contract enforces it for every future
   surface (it already owns the checked conversion).
2. **PostgreSQL's deadline stands in for cancel.** A blocking socket
   cannot hear `pg_cancel_backend`; the deadline is the tool. Lives in
   record 0074 (renumbered from the wave-two 0069 collision). The ADR
   decides whether the single-row cancel story stays "deadline only"
   publicly or gains an engine-side interrupt later.
3. **The per-thread error lifetime on the C door.** The message lives
   until that thread's next call on that engine; the granted header edit
   that wrote it is recorded in 0071. The ADR decides whether per-thread
   slots are the permanent C ABI shape or a documented bridge to a
   per-engine slot when a need shows.
4. **The conformance driver permission.** The phase-three lane may extend
   the DuckDB driver's two branches because its owning lane had landed.
   Lives in `databases/duckdb/NOTES.md`. The ADR (or a standing rule)
   decides the general form: a lane may edit another lane's runner only
   after that lane's work lands, with the reason in the commit.
5. **The private-reference scope.** sdlc/ is part of the public checkout;
   the private name and home paths are cleaned there with evidence kept
   and the name genericized to "the deck repository"; the checker exempts
   sdlc/ by design and the decision lives at its enforcement point
   (`scripts/check_no_private_refs.py` docstring). The ADR decides whether
   that scope is permanent or sdlc/ splits into a public and private tree
   before the repository opens.
6. **The per-surface license widenings.** `Apache-2.0 WITH LLVM-exception`
   (pyo3/polars trees), `Zlib` (polars, pgrx), `BSL-1.0` (polars), each
   allowed only in the workspaces whose trees carry them, generated from
   the root rules; the two unmaintained-crate advisories (paste in R,
   serde_cbor in PostgreSQL) are ignored only where they appear, with the
   replacement plans on record. Lives in `sdlc/scripts/lint-workspaces`
   comments and the unmaintained-crates record. The ADR decides whether
   these widenings become root rules when the tree merges.
7. **Annotate preserves input columns.** R and Ruby's annotate no longer
   overwrite caller columns; the ruled shape is: answer columns are added,
   input columns are returned unchanged, collisions refuse. Lives in the
   R and Ruby NOTES. The ADR confirms the shape as the public contract for
   every surface.
