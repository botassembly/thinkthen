# 0249 language runbook reconciliation

Fresh independent Medium review accepted `edac4e3a` after checking the runbook against accepted preparation, integrated package records and the current C header. The correction names current source inputs, package IDs, Flutter layout, 30 header exports, focused product gates and the separate release requirements. It changes no product source or completion count.

The first review found one retained error: the runbook claimed every experiment had `inputs/toolchain.json`, although Go and C++ preparation recorded its absence. The correction names actual manifests, reports and executed tool versions, and permits checksum claims only where an artifact exists. The same reviewer accepted the one-sentence correction. Ticket and page checks and `git diff --check` passed; no product test was repeated.

## What the build taught us

A preparation note can correctly identify a stale handoff while leaving the original handoff unchanged. Update that shared entry point after acceptance so the next worker does not repeat the investigation. Preserve historical proof and distinguish experiment inputs from integrated product source.
