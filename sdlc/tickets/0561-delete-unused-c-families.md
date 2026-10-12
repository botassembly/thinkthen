# 0561: Delete the C export families no library calls

Status: OPEN.

Milestone: 0.2

Depends on: 0549, 0550, 0551, 0552, 0553, 0554, 0555, 0556, 0558

## Outcome

The C library exports only what the 0.2 session interface and the language libraries use, plus anything a 0.2 document names as public C API. Hand-written source in `libraries/c/src` falls from about 12,500 nonblank lines toward about 2,700. No 0.2 behavior changes for any language library.

## Evidence

- Starts from: the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md) and a read-only review of the 0.2 combined branch.
  - The library exports 111 symbols, against 22 in 0.1.2.
  - The libraries that use the C ABI are Dart, Ada, COBOL, Objective-C, Swift, JVM, PHP, Go, C#, C++ and Zig. Together they call about 27 of those symbols: the engine, error and session exports, `thinkthen_question_parse`, `thinkthen_question_free` and `thinkthen_request_plan_json`. Python, R, Ruby and TypeScript call the core through their own Rust crates.
  - These families have no caller in any library, and none of them shipped in 0.1.2:
    - the "complete" result ABI: `src/complete.rs`, `src/complete/` and `src/ffi/complete/ffi.rs`, about 3,030 lines;
    - the carrier layouts in `src/ffi/carriers/`, about 1,860 lines;
    - the "current" typed inputs in `src/current/` and `src/ffi/current/`, about 2,950 lines, apart from what question parse and free need;
    - the lazy batch in `src/ffi/batch/ffi.rs`, about 400 lines.
  - The frozen 0.1 paths (`src/call/`, `call.rs`, `typed.rs`, `ffi/typed_facts`, `ffi/texts`, `plan.rs` and the 0.1 exports in `ffi.rs`) total about 1,700 lines. They did ship in 0.1.2.
- Keeps: everything the 0.2 libraries call, their installed checks, the session interface, and the generated header and views, regenerated afterward.
- Changes: these deletions.
  - Delete the complete, carrier, current and batch families, apart from what question parse and free use. Delete their C tests and door tests too, about 4,000 lines.
  - Delete the frozen 0.1 C exports under the 0.2 rule that old public names are deleted ([drive to done](../decisions/2026-10-10-drive-0-2-to-done.md)). Add their 0.2 replacements to `libraries/UPGRADING-0.2.md`. If a 0.2 document or library still needs one of them, keep it and say why in Progress.
  - Merge `plan.rs` into the request plan export.
  - Fold `session/errors.rs` into `failures.rs` where the two repeat each other.
  - Regenerate the header, the Swift bridge, the PHP, Ada and COBOL declarations and the Zig checks.
  - After the deletions, delete each public `Engine` wrapper that is left with no caller outside tests and did not ship in 0.1.2. Check `try_annotate_with`, `try_rank_with`, `try_find_with` and `relate_complete_with` first, and move their tests to `Request` under the 0558 amendments. Idea 0559 holds whatever remains.
  - Lower each changed ratchet to its measured value.
- Proof: these checks.
  - Every language library's routine installed checks pass.
  - The C gate passes: formatting, all-targets Clippy, and its unit and door tests.
  - A symbol listing of the built library shows only the kept exports, and every kept export has a caller or a documented reason.
  - The routine `test` and `lint` pass.
- Defers: moving remaining C-side logic into the core beyond these deletions.
