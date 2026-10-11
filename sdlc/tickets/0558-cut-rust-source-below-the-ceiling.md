# 0558: Cut Rust source well below the ceiling

Status: OPEN.

Milestone: 0.2

Depends on: 0550, 0551, 0552, 0553, 0554, 0556

## Outcome

The nonblank Rust line count that `sdlc/scripts/ratchet.mjs` measures falls to 186,000 or fewer, and `sdlc/ratchet.json` falls with it. The cut comes from deleting dead code, unpublished duplicate methods and repeated test setup. Public behavior and contract coverage stay the same.

## Evidence

- Starts from:
  - Combined lanes 0550 to 0556 measure 190,417 on `da353c648` against a ceiling of 189,500. 0549 removes 37 more.
  - The 2026-10-11 adversarial review found that the ceiling was raised eight times without paying any raise back.
  - A read-only survey of `da353c648` on 2026-10-11 found the items under Changes. The line counts below are from that survey and are approximate.
- Keeps:
  - Every public Rust method that shipped in 0.1.2. Twelve of the wrapper methods below shipped: `choose_many`, `choose_many_with`, `decide_many`, `decide_many_with`, `details_many`, `details_many_with`, `filter`, `filter_with`, `score_many`, `score_many_with`, `tag_many` and `tag_many_with`.
  - Every binding, CLI, MCP and database case.
  - `rustfmt.toml` as it is. Changing format settings lowers the count without removing logic, so it does not count toward this outcome.
  - Unit tests in `src` that no outside test covers, including `http/tests.rs`, `mcp/tests/`, the schema golden tests and the fork, process and Windows tests.
- Changes:
  - Delete dead host code in `crates/thinkthen-host/src/source/`: `execute`, `execute_prepared`, `dispatch` and everything only they reach, including all of `relate_output.rs`. That is about 560 lines. Keep `parse`, `Selection`, `usage`, `defect` and `request_source`, which `libraries/c` uses. Delete the out-of-date `allow(dead_code)` comments.
  - Delete `Engine::annotate_each_with` and its helpers, `public/bulk/annotation.rs:280-432`, about 150 lines. Keep `rendered`, `Annotating`, `record` and `admit_declarations`.
  - Delete the public `Engine` wrapper methods that did not ship in 0.1.2 and that nothing outside the crate's own tests calls. These are the `*_complete_with`, `*_records_complete_with`, `try_*` and image `*_input*` families in `public/complete/`, `public/images/calls.rs`, `public/images/many.rs`, `public/bulk.rs` and `public/bulk/details.rs`. That is 58 of 70 methods, about 1,200 lines. Move their tests to `Request`. Move `libraries/rust/examples/recognize_observe.rs` to `Request`. Update `sdlc/scripts/inventory` and the specs that name them.
  - In `crates/thinkthen/tests/backend`, replace repeated setup with shared helpers in `harness/mod.rs`, about 2,500 lines in all:
    - one `exits(&output, n)` helper for the 157 copies of the six-line exit-code check;
    - one annotate runner for the 48 repeated `spawn(&["annotate", …])` calls and the three local `run()` copies;
    - one copy each of `folder()`, `printed`, `said`, `text` and `code`.
  - Move the edge cases in `decide_edge.rs` and `choose_and_score_edge.rs` that repeat rows in `refusals.rs` and `secrecy.rs` into those tables, then delete the copies. First add the four rows the tables lack: `--raw --quiet`, `--threshold 90`, the reversed band `0.9:0.1` and empty evidence.
  - In the other integration tests, add helpers for the repeated `Engine::builder()` chains, `RecordInput` literals and session result reads, about 950 lines. A builder helper takes the key as an argument, because some tests check that their own key never leaks.
  - Delete the unit tests in `src` that an outside test already covers, about 400 lines. These are `src/core/question_file/tests.rs`, the unsafe-sentence and held-lock tests in `src/engine/usage/tests.rs`, `src/core/records/tests.rs`, and the store-privacy test in `src/engine/store/tests.rs`. Each deletion names its covering test in the commit message. Keep a unit test when its covering test runs only with the `cli` feature or only on Unix and the unit test is the only check of that behavior in a library-only or Windows build.
- Proof:
  - The ratchet passes at 186,000 or fewer, and `sdlc/ratchet.json` equals the measured total.
  - Rust public cases, CLI cases, Rust Clippy in both library configurations and policy pass.
  - `cargo check --workspace --all-targets` passes.
  - A search finds none of the deleted wrapper names outside `sdlc/`.
  - The refusal and secrecy tables hold every edge case the deleted edge files checked.
- Defers: removing the twelve wrapper methods that shipped in 0.1.2, and DuckDB's `details_many_recoverable_with` path, which needs DuckDB on a `Request` session first. Idea 0545 keeps the remaining test cuts after 0.2.
