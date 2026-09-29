# JVM J1 recognize offset case returns a backend error

Status: Open. Observed during the focused Python-entry Quick Fix candidate `12058bb45`; cause and regression age are unconfirmed.

After the renamed direct JVM J1 runner passed Python startup and schema validation and compiled its Java/Kotlin/Scala consumers, Java case `41-offsets-past-an-accent-and-an-emoji` returned `{"error":"backend"}` instead of the expected recognize result. Kotlin and Scala runtime loops were not reached. The runner body was unchanged by the filename correction. Other selected hosts passed the shared case with the same native library hash. See the [build record](../records/2026-09-29-clean-python-package-entry-quick-fix-build.md).

First compare the actual JAR, TypeCase, native library and conformance-backend inputs to current main and reproduce this one case with its captured request. Separate stale artifact or fixture mismatch from product behavior; do not infer a Unicode product defect from the case name. Preserve the exact expected result and send counts.

Done when the mismatch is explained and fixed in its owning layer, the selected Java case passes under a minimal explicit environment, and the equivalent Kotlin/Scala routes are either checked or retain a named follow-up. Run only the relevant functional subset. A passing case does not qualify the complete package or release runner.
