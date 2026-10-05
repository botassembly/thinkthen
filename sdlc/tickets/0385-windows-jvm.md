# 0385: Windows stage 1: the JVM binding loads the Windows DLL

Status: ready. Deferred to 0.3 under Ian's 2026-10-05 scope ruling; the ticket stays open.

Milestone: 0.3

## Outcome

- On Windows, with `-Dthinkthen.library` naming ticket 0381's DLL, the binding loads it and answers from a loopback backend. The jars still neither fetch nor bundle the library, as `libraries/jvm/README.md` says.
- The JVM tests that need `bwrap` or `pthread_self` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/jvm/README.md` names Windows. Linux and macOS loading is unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build the JVM binding. `Door.java` loads the library from `-Dthinkthen.library`, so its code is name-agnostic. `check.sh` builds a `:` classpath and expects a `.so`. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 50 to 200 lines and 1 slice, with low Linux and macOS risk. No experiment preceded this ticket.
- Keeps: `Door.java`'s loading rule, which needs an absolute path. The Linux and macOS jars' contents.
- Changes: `check.sh` builds a classpath with the platform's separator and finds the platform's library name. The README shows the Windows `-Dthinkthen.library` form. The Unix-only tests get skips with reasons.
- Proof: a load and smoke case on `windows-2025` that counts loopback requests. The Linux and macOS JVM checks and the installed-file check stay green.
- Defers: the report names no unknown of note. Kotlin and Scala ride on the same jar, and their own checks on Windows wait for stage 2 unless the builder finds them free.
