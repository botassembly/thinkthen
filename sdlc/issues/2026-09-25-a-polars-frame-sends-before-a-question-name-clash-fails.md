# A Polars frame sends before a question name clash fails

Status: Open

Filed on 2026-09-25 from the pandas build (ticket 0122, which named this check in its Defers). No key was used, and no request left the machine.

## What happens

`tt.annotate(set, frame, on="body")` over a Polars frame that already has a column named as one of the set's questions sends every row first. Then Polars refuses the answer frame with its own `DuplicateError`: "could not create a new DataFrame: column with name 'late' has more than one occurrence". On the loopback backend, a two-row frame with a `late` column and a set with a `late` question counted 2 sends before the error. The error is not a `ThinkThenError`.

A pandas frame refuses the same clash before any send: "the frame already has a column named 'late'; rename it first" (ticket 0122, decision 4).

## What would fix it

Check each question name against the frame's column names before the first send, in `_annotate_frame` in `libraries/python/src/frame.rs` or in the package, and raise the pandas sentence. Ian can choose instead to overwrite the column, as `assign` would, for both libraries.

## Tests to update when fixed

- `libraries/python/tests/test_door.py`, `test_what_the_door_refuses_sends_nothing`: add the Polars clash row with its sentence and zero sends.
