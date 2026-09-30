# The ordered output test races the next request under load

Status: closed on 2026-09-30 by ticket 0340. The test reads no standard output until request 6 arrives, then checks that record 1's row already waits in the pipe. Under load, 3 of 2,400 runs failed before and 0 of 2,400 after.

Kind: debt

Pay when: the next change to the command's output window, or before 0.1.

Keeping it risks a false red that teaches builders to rerun reds.

## What happened

`crates/thinkthen/tests/backend/scheduling.rs:238` `ordered_output_bounds_every_dispatched_row` failed once at load 22 with "request 6 started before record 1 reached standard output". It passed in the next full run and in eight runs alone. The test reads the first event after record 1's release, and under load request 6 can be observed before the output line that freed its slot.

## What should happen

The test proves the window without depending on which of two nearly simultaneous events it observes first, for example by checking that request 6 never starts while record 1's row is unwritten.
