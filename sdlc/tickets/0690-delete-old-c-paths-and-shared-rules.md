# 0690: Delete the old C session, error, input and projection paths

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0648, 0651, 0655, 0658, 0662, 0665, 0667, 0668, 0671, 0673, 0676, 0678, 0679, 0682, 0683, 0686, 0693

## Outcome

- The low-level C session functions (`thinkthen_session_new*`, `push`, `try_push`, `wait`, `next_packet`, `try_read`, `finish` and their typed twins) are deleted from the header and the C library.
- The per-thread error slots and the `thinkthen_error_*(NULL)` and `thinkthen_session_error_*` accessors are deleted.
- The input path 0635 rejected is deleted: its functions, generated carriers and header structs.
- The result representation 0638 rejected is deleted, with its generated outputs in every language.
- The 8192-byte COBOL cap and `THINKTHEN_COBOL_OVERFLOW` are deleted from the shared interface.
- `sdlc/generators/results/templates/host_answers.py` (at its new location after 0637), the shared rule table and every template section left that derives answers are deleted.
- The 0637 answer-rule check's list of exceptions is deleted, so the check refuses answer rules in every template and library file with no exception.

### Proof

- On the M5 through pm job, `generate.py --check`, `lint` and `test` pass, and `sdlc/scripts/check-c-exports.py` passes with the header listing none of the deleted symbols.
- Every library gate passes, natively on the M5 and in the pinned arm64 container.
- The 0691 checkpoint run passes on the revision that lands this ticket.

## Evidence

Phase 2 adds the new path (the run call, one input path, one result representation, answers in the packet) and moves the C library and the Rust library onto it, but it cannot delete the old paths while every other language still uses them on main. Each phase 3 ticket deletes its own language's projection, session loop, error text and fixture consumer. The shared old paths are temporary scaffolding with this ticket as their named deletion owner, not compatibility code. Root cause 3 of the 0.2.1 ruling, no single native path, ends only when they are gone.
