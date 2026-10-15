# 0565: Remove compatibility code before 0.2

Status: OPEN.

Milestone: 0.2

Depends on: 0561

Reviews: revision e6182ac, accept
Reviews: revision 6b2a737, accept
Reviews: revision bec704f, accept

## Outcome

ThinkThen 0.2 carries no code, tests or documents whose only job is backward compatibility. Each public surface has one current way to do each thing, and old input gets a clear refusal. This follows the [no-compatibility ruling](../decisions/2026-10-12-no-compatibility-before-1-0.md).

## Evidence

- Starts from: these places a search on the combined branch found.
  - The 12 Rust wrapper methods that shipped in 0.1.2 and that 0558 kept: `choose_many`, `choose_many_with`, `decide_many`, `decide_many_with`, `details_many`, `details_many_with`, `filter`, `filter_with`, `score_many`, `score_many_with`, `tag_many` and `tag_many_with`. Also the unshipped wrappers that idea 0559 lists, and whatever 0561 leaves.
  - Result compatibility projections: `public/results/compatibility.rs`, `cli/recognize/compatibility_schema.rs`, the legacy Request projections that `tests/request_legacy_projection.rs` checks, and the message-text remap of old decide errors in `public/request/admission.rs`.
  - Store and cache migration: `engine/store/migration.rs` and `engine/store/versioned.rs`.
  - The handwritten complete readers and raw JSON compatibility methods in Python, Ruby, R and JavaScript, which an `AGENTS.md` rule protects until generated readers replace them.
  - `libraries/UPGRADING-0.2.md` and the per-language README sections that compare against 0.1.2.
  - Anything else a search for "legacy", "compat", "frozen" or "0.1" finds in `crates/`, `libraries/` and `databases/`.
- Keeps: every current 0.2 behavior and its outside-in tests, and a clear refusal sentence for old configuration, old cache files and old request shapes.
- Changes: these deletions.
  - Delete each item above, along with tests that check only compatibility. Route every caller through `Request` and the generated readers.
  - Replace store migration with a refusal that names the file and says to delete it. Do the same for old configuration.
  - Delete the upgrade guide and the 0.1.2 comparison text. Remove the `AGENTS.md` rule about handwritten complete readers.
  - Fold idea 0559 into this ticket and close the idea.
  - Lower each changed ratchet to its measured value.
- Proof: these checks.
  - Searching `crates/`, `libraries/` and `databases/` for "legacy", "compat" and "frozen" finds only names this ticket justifies in Progress.
  - Outside-in tests show the refusals for an old configuration file, an old cache file and an old request shape.
  - Routine `test`, `lint` and every language library's routine installed checks pass.
- Defers: nothing.

PM amendment, 2026-10-12. Reading an old cache to refuse it may leave the SQLite `-wal` and `-shm` sidecar files that a read-only open creates. The refusal must leave the database file's bytes and modification time unchanged. Test only that, and do not delete or prevent the sidecars.



PM amendment, 2026-10-13. Remaining work. Close 0565 now. The 0576 Rust lane removes the `compatibility: DecisionResult` field in `core/result/complete/atomic.rs` and `tests/backend/native_results/compatibility.rs` while it merges the result families. 0578 owns the 0.1.2 README text in the R and MCP READMEs. The 0564 lane adds the outside-in test that refuses an old configuration file. At the candidate, `git grep -n -w -i -E 'legacy|compat|compatibility|frozen' crates libraries databases` finds only identifiers that the 0543 record justifies in one line each; English prose is exempt.
