# DuckDB uses owned native Request calls

DuckDB's named SQL functions now enter shared Request admission and owned native sessions. The extension retains SQL value conversion, authorized calling-thread file readers, host cancellation and typed result projection. The legacy shared dispatcher was removed in the shared-host migration; its 277 deleted lines are counted there, not again here.

Final installed checks found a real file-recognition defect: every streamed aggregate replaced the preceding one, so the empty final aggregate erased all recognized rows. The projection now appends recognition rows while retaining ordinals, completed prefixes and failure facts. The existing files-recognize check failed before the fix and passes afterward. Fresh review accepts `9a882af553fc03b5136b23eb33d0a3be5cf23f59`, including the DuckDB Rust ceiling increase from 5395 to 5403 for eight nonblank lines. Supported SQL names remain valid, and README guidance identifies native JSON results.

The current rebuilt and installed extension passes 68 routine, complete, file and focused selectors, 13 host checks, owning run facts and typed SQL tables. All ten complete and file functions work through that artifact. NULLs, binary images, descriptions, options, context, original positions, strict replay, failure handling, small file bounds and held cancellation pass. Strict Clippy, formatting, policy, child-process, source and ratchet checks pass. Root routine and lint evidence is reused for unchanged behavior; release-only and platform cases run at the candidate.

The installed extension in `target/0495-close/installed/thinkthen.duckdb_extension` has SHA-256 `6eb0440002bd2a9d59063716798f2d259bda2640e360350d91922f08295536fa`. Build and installed evidence is in `target/0495-close/` in the DuckDB lane. No paid or load calls ran.

## What the build taught us

An empty terminal packet settles a stream; it must not replace completed output. Test the installed file function as well as the complete function, because their projections differ. Keeping shared admission does not remove the need to check host result accumulation.

Closure integration passes all 1996 routine Rust cases, one doctest, 21 external consumer cases and the remaining routine script and lint checks. Unchanged earlier passing checks are reused.
