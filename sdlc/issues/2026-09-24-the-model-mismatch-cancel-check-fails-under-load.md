# The model-mismatch cancel check fails under load

Status: Open.

`annotate::scheduling::a_model_mismatch_cancels_groups_that_have_not_started` asserts at `tests/backend/annotate/scheduling.rs:356` that the listener saw 3 requests. The listener orders its replies with fixed delays of 10, 20, and 40 ms. On a loaded machine the command also sends group 3, the listener sees 4 requests, and the check fails. The command still exits 4 with the model-mismatch message.

Observed on 2026-09-24 while building ticket 0085, one-minute load 7 to 8 on 16 cores:

- Main at `d4ebe645`, alone: failed 8 of 8, although one earlier run passed.
- The 0085 branch: failed 15 of 15 alone, then 1 of 5 and 3 of 5 across trivial code changes. The full ladder `test` step failed this test once.

The check proves a real property. A fix should hold group 0 and group 1 with the harness `after_release` barrier or a held arm, and release group 1 only after group 0 has answered. It should not rely on the gaps between delays.
