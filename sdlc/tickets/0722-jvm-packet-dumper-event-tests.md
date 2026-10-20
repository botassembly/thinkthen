# 0722: Replace the JVM fixture consumers with a packet dumper and event-driven tests

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0673, 0688

## Outcome

- Changes: `libraries/jvm/tests/`, `libraries/jvm/check.sh`.
- The JVM conformance runner becomes a packet dumper following the 0642 contract. `tests/SessionConsumer.java`, `tests/public_types.py` and the other fixture consumers are deleted.
- Timing tests are replaced by event-driven ones: latches, a held provider that signals, and the native call's own completion. The `/proc/self/task` thread sampling is deleted.

### Proof

- `git grep -nE 'Thread\.sleep|/proc/self/task' libraries/jvm/tests` finds nothing; on current main it finds the sleeps in `SessionConsumer.java`, `SessionScala.scala` and `UsageStatus.{java,kt,scala}` and the sampling in `SessionConsumer.java`.
- The JVM check needs bubblewrap and is declared for `linux-x86_64` only, so at the 0619 container checkpoint on Yellow (x86_64), `sh libraries/jvm/check.sh` passes, the Java, Kotlin and Scala dumps match the 0642 oracle, and the session tests pass three runs in a row.

## Evidence

- `libraries/jvm/tests/SessionConsumer.java` sleeps between steps and samples `/proc/self/task` over 600 ms to count worker threads.
- `libraries/jvm/tests/UsageStatus.java`, `UsageStatus.kt` and `UsageStatus.scala` poll for a `released` file every 5 ms.
- `libraries/jvm/tests/public_types.py` drives the JVM fixture consumer for the shared parity cases.
- Split from 0672, which owns the Gradle build. Both edit `libraries/jvm/check.sh`, so they run one after the other.
