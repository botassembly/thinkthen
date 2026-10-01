# 0385: Windows stage 1: the JVM binding loads the Windows DLL

Status: ready. It waits for the `release/0.1` cut, for ticket 0380 slice A, which adds the fifth release target, and for ticket 0381 slice A, which ships the DLL. No slice lands on main before the cut (ADR 0116 item 2). Plan: `sdlc/planning/windows.md`, stage 1. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. The coordinator assigns a lane after the cut.

Milestone: 0.2

## Outcome

- The jar carries the DLL in its native resources, and on Windows it loads it and answers from a loopback backend.
- The JVM tests that need `bwrap` or `pthread_self` skip on Windows with a reason. The rest pass on `windows-2025`.
- `libraries/jvm/README.md` names Windows. Linux and macOS loading is unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0) did not build the JVM binding. `Door.java` loads the library from `-Dthinkthen.library`, so its code is name-agnostic. `check.sh` builds a `:` classpath and expects a `.so`. The stage 1 report in `sdlc/planning/windows.md` sizes this ticket at 50 to 200 lines and 1 slice, with low Linux and macOS risk. No experiment preceded this ticket.
- Keeps: `Door.java`'s loading rule. The Linux and macOS jars' contents.
- Changes: `check.sh` builds a classpath with the platform's separator and finds the platform's library name. The jar's native resource layout gains the DLL. The Unix-only tests get skips with reasons. `release.yml` puts the DLL into the jar.
- Proof: a load and smoke case on `windows-2025` that counts loopback requests. The Linux and macOS JVM checks and the installed-file check stay green.
- Defers: the report names no unknown of note. Kotlin and Scala ride on the same jar, and their own checks on Windows wait for stage 2 unless the builder finds them free.
