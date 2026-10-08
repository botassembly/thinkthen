# 0472: Match C batch metadata to the public constants

Status: implementation complete; final checks and landing remain with the queue owner.

The C carrier now emits Records=1 and Max=2 as the public header declares. Complete native results now carry the saved versus running batch warning when a saved question has an authored threshold. A private authored-threshold marker distinguishes a saved threshold from the default rule. C public getter checks cover both kind/count pairs, both warning sides, and warning absence without an authored threshold. Go checks the decoded meaning through its installed native binding. The header, ABI layout, and binding conversion tables are unchanged.

The focused C getter test passed after the correction. The Go package check and full tests/lint are pending at this record's first write. A fresh read-only whole-change review found that the first warning implementation would report a false mismatch for a batch-only file. The corrected marker and the warning-absence case address that finding.

Rust source grew by 28 nonblank lines, including the marker and one shared complete-result helper, after checking the existing complete call sites for duplication. The C Rust gate grew by 12 lines for the public getter harness. The C fixture grew by 50 lines to check the actual borrowed metadata. The Go test grew by 28 lines. The existing size warnings belong to unchanged files; no changed file reaches the warning threshold. The measured ratchets are 163305 Rust, 15105 C Rust, 2696 C, and 4932 Go.

## What the build taught us

Layout and enum declarations do not prove the values returned by a public getter. The saved threshold's presence also matters: a resolved default threshold cannot establish that a file was tuned. One public getter test needs both kind values, warning direction, and warning absence. Release qualification and other binding checks remain outside this ticket.
