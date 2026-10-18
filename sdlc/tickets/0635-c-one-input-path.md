# 0635: Choose one input path for the C interface

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0612

## Outcome

- The choice between typed structs and JSON text as the one C input path is recorded in the repo with its reasons and the cost of each option.
- The C library's own examples and tests, and the run call from 0638, use only the chosen path.
- The other path's functions, generated carriers and header structs stay only until the languages that use them move in phase 3. 0690 deletes them; nothing new is added to them.
- The runtime version check and the COBOL cap are 0692's work.

### Proof

- The decision record names the chosen path, the rejected one and the reasons.
- On the M5 through pm job, the installed C consumer runs the session and plan cases through the chosen input path.

## Evidence

Most C entry points come twice, once taking JSON text and once taking typed structs: `thinkthen_session_new_with_surface` and `..._new_typed_with_surface`; `push` and `push_typed`; `try_push` and `try_push_typed`; `finish` and `finish_typed`; and `thinkthen_request_plan_json`, `..._typed_json`, `..._value`, `..._typed_value` and `..._named_value`. Every new input feature needs both paths. The typed path alone is a 5,161-line generated module (`libraries/c/src/ffi/native_request/carriers_generated.rs`). The header carries 422 structs for 46 functions.

The run call in 0638 should be built once, on the chosen input path, so this choice comes first.
