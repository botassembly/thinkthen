# 0564: Move toward outside-in tests only

Status: OPEN.

Milestone: 0.2

Depends on: 0560

Reviews: revision 80dc059, reject

Reviews: revision 7134b30, accept

Reviews: revision 387ccc1, accept

Reviews: revision 699ef50, accept

## Outcome

Unit tests in `src` stay only where no public surface can show the behavior, or where the stakes call for a direct test, and each one says why. Every behavior they covered that a caller can reach has an outside-in test through the CLI, the Rust public API, the MCP command or an installed library. Outside-in coverage, measured by 0560, rises from 88.7%, and no unique covered line is lost. Passing tests leave no temporary folders behind.

## Evidence

- Starts from: the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md), the updated outside-in skill (`repos/agents/skills/outside-in-tests.md`), and a coverage review of `6712593f6`. The review's scripts and results are on branch `ticket/0560-measure-coverage` under `sdlc/scripts/coverage-draft/`.
  - Outside-in tests alone cover 88.7% of 44,882 production lines, and the unit tests raise that to 92.3%. 93.5% of the lines the unit tests reach are already reached from outside.
  - `src` holds 526 test functions in 22,166 lines. 90 of its 133 unit modules each add fewer than five unique lines, 12,657 lines and 270 tests in all. Examples:
    - `cli/conformance_tests*`, 1,753 lines with no unique lines. The binding drivers replay the same `conformance/cases.json`.
    - `core/question_file/resolve/tests.rs`, 15 tests with no unique lines.
    - `engine/facade_tests.rs`, 469 lines with 2 unique lines.
    - `core/digest.rs`'s inline tests and `ownership_tests.rs`, neither with unique lines.
    - Duplicate framing tests in `core/records/tests.rs`.
  - Unit tests that are needed today:
    - `src/mcp/tests/`, because outside tests never reach `mcp/request.rs`, `tools.rs`, `admission.rs` or `executor.rs`;
    - the schema golden tests;
    - the timing and fault-injection cases in `engine/usage`, `engine/http` and `cli/interrupt`.
  - Gaps in outside-in coverage:
    - the MCP server, 837 lines;
    - the binding-only request API in `public/request`, 735 lines (`column.rs` and `question_parts.rs` are fully unreached);
    - result accessors that nothing calls, about 600 lines, including `with_wording_version`, which has no caller at all;
    - error and refusal paths, about 1,100 lines.
  - Two integration files use doc-hidden items that only bindings use: `tests/public_batches/native.rs` and `tests/request_legacy_projection.rs`.
  - About 4,000 lines of language-library tests call native or internal layers instead of the installed API, such as `test_native_results.py`, R `:::` calls and the Zig raw C session test. Nine places carry near-identical copies of a privacy scanner. An estimated 35 to 55% of hand-written binding test lines restate rules that the core or the CLI already tests.
  - Each passing run leaves about 80 `/tmp/thinkthen-complete-*` and `/tmp/thinkthen-native-store-*` folders. About 93,000 have built up on the Beelink.
- Keeps: every unique covered line, every distinct regression under `AGENTS.md`, the schema golden tests, `conformance/cases.json` as the one shared edge-case table that every binding replays, and the routine suite's speed limits. A unit test stays where the stakes call for a direct test: parsers, secrecy, cancellation and timing. Its name or a comment says why.
- Changes: these steps, in small batches, each checked against the 0560 report.
  - Before deleting a unit module, name the outside-in test that covers each of its claims. Then delete it. Start with the modules listed above.
  - Rewrite the MCP unit tests as `thinkthen mcp` tests that drive the command over stdin. Move `ownership_tests.rs` into `tests/`.
  - Cover `RequestColumn` and `RequestQuestion::inline` through the Rust public API, or through one binding's public tests.
  - Delete public accessors and methods that no surface uses, starting with `with_wording_version`. Give the others an outside-in test.
  - Put the refusal paths that are worth stating into one edge-case table. Leave rare I/O failures uncovered.
  - Replace the test-only hooks in `engine/usage.rs` with a real boundary, or keep them as documented exceptions.
  - In language libraries, move native-layer tests onto the installed public API. Share one privacy scanner. Delete hand-written repeats of core refusal rules that `cases.json` already replays.
  - Make each test that creates a temporary folder remove it.
  - Lower each changed ratchet to its measured value.
- Proof: these checks.
  - A 0560 run after the change shows outside-in coverage at or above 88.7%, and no line that unit tests alone reached before is now unreached, unless its code was deleted.
  - Each unit test that remains in `src` names why it stays.
  - Routine `test`, `lint` and every language library's routine installed checks pass.
  - One routine run leaves no new `/tmp/thinkthen-*` folders.
- Defers: per-test coverage between the CLI and Rust API suites. The 0560 script can add it later.



PM amendment, 2026-10-13. Remaining work. 0564 keeps only Rust, in `crates/thinkthen/src` and `crates/thinkthen/tests`, outside `src/core/result/complete` and `src/public/results`, which the 0576 Rust lane owns. Delete or move `src/cli/conformance_tests`, `engine/facade_tests.rs`, `core/question_file/resolve/tests.rs`, the repeated framing tests in `core/records/tests.rs`, the inline tests in `core/digest.rs` and what `src/mcp/tests` duplicates, naming in each commit message the outside test that covers each deleted claim. Make `tests/native_complete.rs` remove its `/tmp/thinkthen-complete-*` folders. Move `src/ownership_tests.rs` into `tests/`. Stop `tests/public_batches/native.rs` using doc-hidden items. Give each remaining unit test module a `//! Stays:` line, and add a lint check for it. Add one outside-in test that refuses an old configuration file, for 0565. The candidate coverage run is the only coverage proof, against the 0560 floor; the 88.7% figure is withdrawn. The library test items move to the 0576 language lanes, and one shared privacy scanner goes in `libraries/` with the first lane that lands. Proof: `sdlc/scripts/test` and `sdlc/scripts/lint` pass, and the count of `/tmp/thinkthen-*` folders is the same before and after one routine run.

## Progress

- 2026-10-13 started
- 2026-10-13 next: Merged the freshly accepted record-test cuts and parent-owned signal-folder cleanup into final at 39700e21b; preserve distinct checks and continue remaining test cleanup before the candidate.
