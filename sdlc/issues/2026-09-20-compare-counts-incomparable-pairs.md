# `compare.jq` counts pairs whose evidence or label changed

Status: Open

Found 2026-09-20 in the full-project review at `2c32524` and confirmed after ticket 0026.

`compare.jq` correctly lists a shared id under `mismatched_input` or `mismatched_label`, but it still includes that pair in `same` or `flips`. A changed trusted label means the two runs measured different targets. Changed evidence means they judged different cases. Counting either pair as agreement or movement gives a result the transform's own header calls worthless.

The output also gives no count of pairs that were actually compared, so a reader cannot reconcile `same` and `flips` after mismatches are excluded. Ticket 0027 must separate shared ids from comparable pairs and keep mismatched pairs out of answer comparisons.
