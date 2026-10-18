# 0672: Build the JVM package with Gradle and keep Java 22 as the floor

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0673, 0688

## Outcome

- Gradle or Maven builds and packs the artifacts. The script-driven compiler calls and environment-variable toolchain lookup are deleted.
- Java 22 stays the floor. A recorded decision states the reason, the final foreign-function API, and the README says so.
- Java, Kotlin and Scala ship as separate artifacts with declared standard-library dependencies, and the Scala artifact carries a Scala-version suffix.
- The JVM conformance runner becomes a packet dumper following the 0642 contract, and `tests/SessionConsumer.java` and the other fixture consumers are deleted.
- Timing tests are replaced by event-driven ones: latches, a held provider that signals, and the native wait. The `/proc` thread sampling is deleted.

### Proof

- On the M5 through pm job, a clean Gradle consumer and a clean Maven consumer each resolve the locally published artifacts.
- Each consumer runs a named call on Java 22 and on the newest Java, with no library path override.
- The session tests pass repeatedly with no sleeps in the test sources.

## Evidence

- **No standard build tool.** `libraries/jvm/build-session.py:65-69` calls javac, kotlinc and scalac directly, found through environment variables, and `toolchains.py:34` maps one variable onto another.
- **Java version.** `pom.xml:9` sets the release to Java 22. Java's foreign-function API became final in Java 22; on Java 21 it is a preview feature that needs `--enable-preview`, which no library can require of its callers.
- **Missing dependencies.** The pom declares only optional coroutines (`pom.xml:34-37`). It declares no Kotlin or Scala standard library, and the Scala classes carry no Scala-version suffix. This is uncertain if a classifier pom adds them.
- **Silent skip.** `check.sh:10` exits 77 ("not run") when a tool is missing; 0618 makes that a failure.
- **Timing-dependent tests.**
  - `tests/SessionConsumer.java:31,52,80-83` sleeps.
  - The same file samples `/proc/self/task` over 600 ms to count worker threads.
  - `UsageStatus.{java,kt,scala}:19` poll a file every 5 ms.
