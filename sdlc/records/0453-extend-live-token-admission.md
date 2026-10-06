# 0453: Extend authorized live token admission

The live helper can add token admission with --add-tokens N under the same lock as charges. It appends a durable extension row and preserves all prior charges. Amounts and cumulative totals use checked signed-64-bit bounds. The control command reads no key and starts no job. Token admission does not change monetary authority.

Fresh High read-only review: ACCEPT. Existing live fixtures, syntax and old-helper refusal checks passed; full tests and lint run on the landing commit. No actual ledger extension or paid job occurred during this change.

## What the build taught us

Encoded image admission can greatly exceed vendor billing tokens. Separate token allowance from the approved dollar cap. Older helpers safely refuse extension rows; operators must use the updated helper. Inspect status before retrying an ambiguous append or sync failure.

After landing, the coordinator used the helper to add 1,231,988,070 tokens for approved 0036/0037 preparation. Charges remained 475,976,740; no job launched. Shared mailroom answer d37418b preserves the separate $15/$10 monetary caps and closes all five admission asks.
