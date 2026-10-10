# 0385: Load the bundled Windows library from the JVM packages

Status: COMPLETE.

Milestone: 0.2

Depends on: 0504
Depends on: 0530

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

On Windows a Java, Kotlin or Scala caller declares the ordinary Maven dependency and makes a real call. The package selects and loads its bundled Windows native library with no `-Dthinkthen.library` setting and no preview flags. Linux and macOS loading is unchanged. The JVM README names Windows as supported.

## Evidence

- Starts from: Windows stage 0 (0373) did not build the JVM binding. `Door.java` loads the library from an absolute `-Dthinkthen.library` path. `check.sh` builds a `:` classpath and expects a `.so`. The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) requires every package to carry its native library, which replaces the old jars-without-natives contract and mandatory path. The [0521 assessment](../records/0521-surface-contract-assessment.md) replaced the 0.3 deferral.
- Keeps: Linux and macOS loading and jar contents apart from the added native assets.
- Changes: Consume 0504's stable JVM implementation and the Windows native artifacts assembled by 0530 under 0517's package design. Checks build classpaths with the platform separator and find the platform library name. Unix-only test machinery (`bwrap`, `pthread_self`) may use Windows equivalents. Required cancellation and cleanup cases cannot pass by skipping. Claim `libraries/jvm/**` test and check files named per slice and `.github/workflows/windows.yml`.
- Proof: On `windows-2025`, installed Java, Kotlin and Scala public consumers make one call each and count loopback requests. The Linux and macOS JVM checks and the installed-file check stay green. Windows execution is owed to the first authorized candidate, and the ticket stays open until native qualification passes.
- Defers: Candidate dispatch and publication wait for Ian's permission.
