# 0667: Thin Zig calls onto run-call and packet answers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Named calls go through the run-call entry point. `CallResult` reads value and probability from the packet's answer fields, through the one result representation 0638 chose.
- The drain loop in `blocking.zig` and its private closed, reader and producer error text are deleted.
- The projection section of the Zig template (sdlc/generators/results/templates/zig.py) is deleted. Failure facts are read through the chosen result representation without per-field copying.
- Errors are a Zig error set plus a retained failure record, as the contract table describes.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

On the M5 through pm job, an outside Zig project depending on the unpacked package runs the examples. Its packet dump matches the 0642 oracle.

## Evidence

- libraries/zig/src/blocking.zig drains packets and copies failure facts field by field with `inline for` (:22-60).
- Each function's value and probability come from the generated `NamedCallResult` projection in results_generated.zig (5,813 lines). That is another copy of the per-function rules.
- Zig commits keep restoring generation and sessions after core changes: 1ddcaf267 "Restore Zig generation and check every target" and 557bd6e30 "Restore Zig native owned sessions and typed settings".
