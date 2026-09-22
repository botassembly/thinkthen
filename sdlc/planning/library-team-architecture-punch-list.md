# Library team: architecture punch list

Handoff from the architect's survey of `surfaces` at `9ef50a6` and main's engine code at `76b3511`. Main's later settlement ledger at `5c11eb9` was also read. These are integration requirements and cleanup recommendations, not authorization to publish or change settled public contracts. Ian can overturn the recommendations.

Keep working in your own worktree. Leave main and ticket 0065 to their owners. Do not build another version of the coming engine inside the stand-in. The architect owns the production Rust API, shared semantics, and integration sequence.

Progress checked on resumption, 2026-09-22: `surfaces` at `7fdb1fa` commits the language lane's report in `sdlc/records/2026-09-21-punch-list-report.md`. Ruby's extra probability calls and JSON re-parsing are fixed; ownership tests and parser-invalid fixtures are committed. Six scalar loops await production bulk entries. The original punch list below remains the acceptance checklist, not a request to repeat those fixes. Database and packaging work remains with its active owners, and none of these stand-in checks proves the real-engine swap.

## The boundary

One Rust engine owns parsing, validation, question construction, answer interpretation, scheduling, retries, cache, counters, recognition, and relations. A binding converts arguments, manages host lifetimes and interrupts, calls Rust, and presents the result. Native adapter code may construct host objects and columns; it must not become another scheduler or implement another answer rule.

Rust creates a complete immutable judgment snapshot. A binding derives the existing bare answer or requested details from that same call. Keep the public bare-value forms. The standalone `details(question, text)` remains a separate judgment call with normal cache/network behavior. Reading fields already returned by a call must not ask the engine again.

## Cleanup to prepare on your branch

1. **Remove extra calls for data already obtained.** Ruby's `decide_many_with_probabilities` makes the bulk call and then a details call per record (`libraries/ruby/lib/thinkthen.rb:119-126`). Carry the existing probabilities through the native binding instead. Acceptance: a counted backend sees no extra sends merely to expose those probabilities.

2. **Inventory scalar loops that belong in the engine.** Some column forms call scalar choose, score, or tag repeatedly. Identify them for conversion to the shared Rust bulk entry points. A loop that converts returned values into a host column is fine. A loop that schedules judgments belongs in the engine. Do not invent a second bulk implementation while waiting.

3. **Preserve canonical results and ownership.** Use typed accessors instead of scraping serialized answer JSON. Test nested mutation, result lifetime after the call, and buffer ownership. Host tables or objects may be detached copies; changing one must not change the Rust result. Document index units and perform each host conversion once. Include accents, emoji, repeated names, and relation endpoints.

4. **Remove the hidden counter reset.** SQLite still accepts `thinkthen_usage('reset')` and clears process counters plus its shim cache (`databases/sqlite/src/lib.rs:358-366`). That is the removed reset capability under another spelling. Keep counters cumulative. Callers can subtract snapshots. Keep the temporary shim cache explicitly marked for removal at the engine swap.

5. **Make database authority explicit.** Name each extension's allowed question-file access, backend selection, credential source, query execution, connection lifetime, and cancellation channel. Keep SQLite non-deterministic. Prove DuckDB's accepted LOAD-time signal handler coexists with its host. A configured PostgreSQL credential must reach the engine safely or be refused; it must not silently do nothing. Never put credentials in fixtures or reports.

6. **Finish the packaging evidence.** Supply clean installed-artifact checks, supported database/ABI floors, a self-contained R package, remaining macOS evidence, and one tested examples file per surface. State unavailable checks plainly. Node child-process startup does not prove inheritance of a warm native Rust pool.

## Changes that wait for the engine contract

- Replace the contract's second question-set parser with the production parser. It currently sorts names and lacks production validation (`contract/src/lib.rs:1574-1604`). Prepare invalid cases for versions, duplicate/unknown keys, empty sets, names, `on` groups, collisions, and file order. Do not maintain two grammars for release.
- Replace stand-in per-unit `find` and per-question `annotate` with production relative find and grouped annotation. Keep synthetic fixtures labeled; do not rewrite old responses under new request digests.
- Preserve score's bare number and the implemented highest-probability meaning of its detailed level. Do not reinterpret `nearest` as rounding the numeric score. Preserve rank's index/probability pairs. Find also owes the full candidate distribution under the accepted Rust contract. CLI record output remains separate.
- C needs an accepted options/ownership design covering cancellation, deadlines, partial completion, null/length inputs, allocated results, and concurrent callers before its ABI freezes. Report the missing capabilities; do not widen the header speculatively.
- Do not independently implement recognition policy changes. The architect will settle configurable boundary and overlap behavior in Rust and reconcile the conflicting relation request designs.

## What to return

One short list: fixed on branch; waiting for a named engine capability; still blocked. For each fixed item, name the commit and focused test. Preserve the existing shared cases and named divergences. Real-engine integration must add request-count and failure-path proof; a green stand-in suite alone is not release acceptance.

Nothing is published, no registry name is claimed, and no paid call is authorized by this handoff.
