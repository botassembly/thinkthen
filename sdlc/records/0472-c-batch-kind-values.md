# 0472: Match C batch metadata to the public constants

Status: implementation complete; final checks and landing remain with the queue owner.

The C carrier now emits Records=1 and Max=2 as the public header declares. Complete native results now carry the saved versus running batch warning when a saved question has an authored threshold. A private authored-threshold marker distinguishes a saved threshold from the default rule. C public getter checks cover both kind/count pairs, both warning sides, and warning absence without an authored threshold. Go checks the decoded meaning through its installed native binding. The header, ABI layout, and binding conversion tables are unchanged.

The focused C getter test passed after both review corrections. The full C package passed 73 tests. The full Go package check and a focused Go test against the final C library passed. C Clippy and bounded root lint passed. The first full test exposed a question-equality regression: the provenance marker must not change semantic question equality. That comparison was removed and the exact prior failing test passed. The full test rerun remains pending at this record's fifth write. A fresh read-only whole-change review found that the first warning implementation would report a false mismatch for a batch-only file. A second fresh review found that saved dynamic choose questions lost the authored-threshold marker during record conversion. The marker, its propagation, and the warning-absence cases address both findings.

Rust source grew by 20 nonblank lines, including the marker and one shared complete-result helper. The lint gate caught one extra line in the existing set-rank function, so both set-rank paths now share their existing member-question conversion. The C Rust gate grew by 12 lines for the public getter harness. The C fixture grew by 86 lines to check actual borrowed metadata, including dynamic choose. The Go test grew by 28 lines. The existing size warnings belong to unchanged files; no changed file reaches the warning threshold. The measured ratchets are 163297 Rust, 15105 C Rust, 2732 C, and 4932 Go.

## What the build taught us

Layout and enum declarations do not prove the values returned by a public getter. The saved threshold's presence also matters: a resolved default threshold cannot establish that a file was tuned. One public getter test needs both kind values, warning direction, and warning absence. Release qualification and other binding checks remain outside this ticket.
