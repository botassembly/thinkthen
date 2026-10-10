# 0385: Bundle the Windows JVM native library

The stable JVM packages use the platform loader and declare all five native classifiers. Final registry assembly consumes the Windows C ZIP bin/thinkthen.dll and creates the win-x64 native classifier JAR. Accepted 0530 assembly review and existing registry checks cover exact source identity and refusal when the Windows archive is absent. Current installed stable-JDK Java, Kotlin and Scala migration checks pass without a manual library path or preview flags.

The 2026-10-10 ruling closes this packaging ticket on reviewed implementation and local evidence. Native Windows execution is not claimed. Final assembled-package installation, one counted loopback call per Windows SDK and dependency/cancellation qualification remain in 0530 and the candidate. The coordinator is adding the missing hosted consumer routes before that run. Publishing remains on Ian's go.

## What the build taught us

Package construction and platform execution are separate obligations. Keep platform execution at the candidate and name missing workflow routes explicitly instead of holding completed SDK implementation open.
