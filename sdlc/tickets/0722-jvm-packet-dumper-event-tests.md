# 0722: Replace the JVM fixture consumers with a packet dumper and event-driven tests

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0673, 0688, 0740

## Outcome

- Changes: `libraries/jvm/tests/`, `libraries/jvm/check.sh`, `sdlc/scripts/children`.
- The JVM conformance runner becomes an installed consumer. The installed consumer runs each shared case through the language's run call and compares the answer, probability, presence and owned error fields it converted against `conformance/fixtures/packets.json`, as 0640 does for C. It does not print Rust's Call JSON back. `tests/SessionConsumer.java`, `tests/public_types.py` and the other fixture consumers are deleted.
- Timing tests are replaced by event-driven ones: latches, a held provider that signals, and the native call's own completion. The `/proc/self/task` thread sampling is deleted.

### Proof

- `git grep -nE 'Thread\.sleep|/proc/self/task' libraries/jvm/tests` finds nothing; on current main it finds the sleeps in `SessionConsumer.java`, `SessionScala.scala` and `UsageStatus.{java,kt,scala}` and the sampling in `SessionConsumer.java`.
- The JVM check needs bubblewrap, so it runs in the Linux image. Linux x86_64: `pm job run --machine gator 0722 SHA --memory 7 -- sdlc/scripts/linux-check surface libraries/jvm`. Linux arm64 joins only if 0672 declares bubblewrap and the JVM tools for linux-arm64 in `sdlc/tools/jvm.json`. In that run, `sh libraries/jvm/check.sh` passes, the Java, Kotlin and Scala consumers' comparisons pass, and the session tests pass three runs in a row.

## Evidence

- `libraries/jvm/tests/SessionConsumer.java` sleeps between steps and samples `/proc/self/task` over 600 ms to count worker threads.
- `libraries/jvm/tests/UsageStatus.java`, `UsageStatus.kt` and `UsageStatus.scala` poll for a `released` file every 5 ms.
- `libraries/jvm/tests/public_types.py` drives the JVM fixture consumer for the shared parity cases.
- Split from 0672, which owns the Gradle build. Both edit `libraries/jvm/check.sh`, so they run one after the other.
