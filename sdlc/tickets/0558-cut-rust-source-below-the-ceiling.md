# 0558: Cut Rust source well below the ceiling

Status: OPEN.

Milestone: 0.2

Depends on: 0550, 0551, 0552, 0553, 0554, 0556

Reviews: revision c4b747f05cf35f05d5ca2a376cb003a96820ac07, accept

## Outcome

The nonblank Rust line count that `sdlc/scripts/ratchet.mjs` measures falls to 187,000 or fewer, and `sdlc/ratchet.json` falls with it. The cut comes from deleting dead code, unpublished duplicate methods and repeated test setup. Public behavior and contract coverage stay the same.

## Evidence

- Starts from:
  - Combined lanes 0550 to 0556 measure 190,417 on `da353c648` against a ceiling of 189,500. 0549 removes 37 more.
  - The 2026-10-11 adversarial review found that the ceiling was raised eight times without paying any raise back.
  - A read-only survey of `da353c648` on 2026-10-11 found the items under Changes, and a ticket review checked them. The line counts below are approximate.
- Keeps:
  - Every public, documented Rust method that shipped in 0.1.2. Twelve of the wrapper methods below shipped: `choose_many`, `choose_many_with`, `decide_many`, `decide_many_with`, `details_many`, `details_many_with`, `filter`, `filter_with`, `score_many`, `score_many_with`, `tag_many` and `tag_many_with`. `#[doc(hidden)]` host items sit outside the reviewed public inventory under 0359, so this ticket may delete them when nothing calls them.
  - Every binding, CLI, MCP and database case.
  - `rustfmt.toml` as it is. Changing format settings lowers the count without removing logic, so it does not count toward this outcome.
  - Every distinct regression, under the AGENTS.md rule. A test leaves only when another test checks the same behavior.
  - Unit tests in `src` that no outside test covers, including `http/tests.rs`, `mcp/tests/`, the schema golden tests and the fork, process and Windows tests.
- Changes:
  - Delete dead host code in `crates/thinkthen-host/src/source/`: `execute`, `execute_prepared`, `dispatch` and everything only they reach, including all of `relate_output.rs`. That is about 510 lines. Keep `parse`, `Selection`, `Selection::read`, `usage`, `defect` and `request_source`, which `libraries/c` uses at `plan.rs:92` and elsewhere. Delete the out-of-date `allow(dead_code)` reasons at `mod.rs:33,76,91,408`.
  - Delete the hidden `Engine::annotate_each_with` and its helpers, `public/bulk/annotation.rs:280-432`, about 148 lines. Nothing calls it since DuckDB moved to a `Request` session. Keep `rendered`, `Annotating`, `record` and `admit_declarations`.
  - Cut the 58 public `Engine` wrapper methods that did not ship in 0.1.2. They are the `*_complete_with`, `*_records_complete_with`, `try_*` and image `*_input*` families in `public/complete/`, `public/images/calls.rs`, `public/images/many.rs`, `public/bulk.rs` and `public/bulk/details.rs`.
    - Delete each method that only tests call, and move its tests to `Request`.
    - Make each method that crate code calls `pub(crate)`, or inline it at its single caller. The internal callers include `public/engine.rs:265,296,323,392`, `public/frame/typed_batches.rs:27-183`, `public/bulk/complete.rs:72,145`, `public/rank_set.rs:94`, `public/recognize.rs:597` and `public/frame/inputs.rs:34`.
    - Move the C library's calls to `try_filter_with` and `try_details_many_with` at `libraries/c/src/current/execute.rs:221,235` onto `Request`, the way 0552 moved its other entry points.
    - Move `libraries/rust/examples/recognize_observe.rs` to `Request`. Update `sdlc/scripts/inventory` and the specs that name the removed methods.
  - In `crates/thinkthen/tests/backend`, replace repeated setup with shared helpers in `harness/mod.rs`, about 2,500 lines in all:
    - one `exits(&output, n)` helper for the 157 copies of the six-line exit-code check, with an optional message for the copies that carry their own, such as `address.rs:420`;
    - one annotate runner for the 48 repeated `spawn(&["annotate", …])` calls and the three local `run()` copies;
    - one copy each of `folder()`, `printed`, `said`, `text` and `code`.
  - Delete the tests in `decide_edge.rs` and `choose_and_score_edge.rs` that repeat rows in `refusals.rs` and `secrecy.rs`. First add the four rows the tables lack: `--raw --quiet`, `--threshold 90`, the reversed band `0.9:0.1` and empty evidence. Keep the plan output, model flag, removed environment variable, estimated-input, plan shape and range tests in those files.
  - In the other integration tests, add helpers for the repeated `Engine::builder()` chains, `RecordInput` literals and session result reads, about 950 lines. A builder helper takes the key as an argument, because some tests check that their own key never leaks.
  - Keep every unit test in `src`. This includes the store privacy test in `src/engine/store/tests.rs`, since no library-only test asserts the store modes. Keep `src/core/question_file/tests.rs`, the usage tests in `src/engine/usage/tests.rs` and `src/core/records/tests.rs`. Their outside covers in `tests/backend` build only with the `cli` feature (`tests/backend/main.rs:8`), and the code they check builds without it. Idea 0545 handles them after 0.2.
  - If these changes leave the count above 187,000, take the remaining cut from the test-helper families the survey rated medium confidence: stderr string compares, closed output pipes, profile runs, retry tables, cache races and hand-built launchers in `tests/backend`.
- Proof:
  - The ratchet passes at 187,000 or fewer, and `sdlc/ratchet.json` equals the measured total.
  - Rust public cases, CLI cases, installed C checks, Rust Clippy in both library configurations and policy pass.
  - `cargo check --workspace --all-targets` passes.
  - A search finds none of the deleted public wrapper names in public signatures outside `sdlc/`.
  - The refusal and secrecy tables hold every row that the deleted edge tests checked.
- Defers: removing the twelve wrapper methods that shipped in 0.1.2, and DuckDB's `details_many_recoverable_with` path, which needs DuckDB on a `Request` session first. Idea 0545 keeps the remaining test cuts after 0.2.

## Progress

- 2026-10-11 started
