# 0248 independent design review request

Status: **pending fresh High review**. This author prepared the request and has not reviewed or accepted their own design. Read pushed `ticket/0248-skip-unchanged-retry-sidecar` at the exact SHA supplied with the handoff, read [ticket 0248](../tickets/0248-skip-unchanged-retry-sidecar.md), [preflight](0248-preflight.md), accepted [0235](../tickets/0235-usage-compatibility-and-transient-backend-failures.md) and current `engine/usage/storage.rs`, `usage/tests.rs` on base `6fd7b170`. Return ACCEPT or concrete findings before root grants a runtime file claim.

Review the one proposed amendment: after validating every recognized month/sidecar, checked current and aggregate arithmetic, all contaminated-month migrations and the atomic base write, omit only the **ordinary** retry-sidecar replacement when `next.retries == old.retries`. Verify the state table for a clean missing sidecar, a clean positive sidecar, contaminated current and prior months, a retry-only delta, changed retries, an old writer under the stable lock, overflow before month/sidecar mutation, interruption at every durability checkpoint, unsafe or corrupt sidecar, and preserved file identity. Directory/lock creation may precede overflow refusal. A first-call no-sidecar saving requires no retries and no migration sidecar work.

Check that the proposed outside-in proof really distinguishes an omitted write from identical bytes after atomic replacement: current code must fail a Unix `(dev, ino)` stability assertion for an existing sidecar and a no-sidecar assertion for a clean first no-retry write. Also check that old-reader 100/7 → 101/7 → 102/7, changed-retry sidecar faults, all-month migration, overflow, strict parser/permissions, fixed warnings and previously durable base totals remain covered. `usage/tests.rs` is 480/500 nonblank lines and contains setup assumptions about zero-retry sidecars; the ticket names the necessary test migration and a private child for new cases.

This review decides only the design. It does not establish a current latency improvement or close the original usage-cost issue. No source, public page, provider, real usage folder, ledger, artifact or broad gate was changed in preparation.

## Result

Pending independent reviewer, exact reviewed SHA, verdict and corrections. Preserve findings here rather than replacing this request with an unverified acceptance claim.
