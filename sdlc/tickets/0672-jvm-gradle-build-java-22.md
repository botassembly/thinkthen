# 0672: Build the JVM package with Gradle and keep Java 22 as the floor

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0688, 0728

## Outcome

- Changes: `libraries/jvm/build-session.py`, `libraries/jvm/build.sh`, `libraries/jvm/pom.xml`, `libraries/jvm/tests/toolchains.py`, `libraries/jvm/tests/package_check.py`, `libraries/jvm/check.sh`, `libraries/jvm/README.md`, `libraries/jvm/settings.gradle.kts`, `libraries/jvm/build.gradle.kts`, `sdlc/tools/jvm.json`, `sdlc/scripts/package-inventory.py`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-managed-pair.py`, `sdlc/decisions/`.
- Gradle builds and packs the artifacts from `libraries/jvm/settings.gradle.kts` and `libraries/jvm/build.gradle.kts`. `build-session.py`, `build.sh`'s compiler calls and the environment-variable toolchain lookup in `tests/toolchains.py` are deleted. `sdlc/tools.json` pins Gradle and a Java 22 runtime beside the existing JDK.
- Java 22 stays the floor. A decision record in `sdlc/decisions/` states the reason, the final foreign-function API, and the README says so.
- Java, Kotlin and Scala ship as separate artifacts with declared standard-library dependencies, and the Scala artifact carries a Scala-version suffix.
- The release path (`sdlc/scripts/release-workflow`, `sdlc/scripts/package-inventory.py`, `sdlc/scripts/release-managed-pair.py`) packs and checks the Gradle output.
- The packet dumper and the event-driven session tests are 0722's.

### Proof

- On the M5 through `pm job run --machine m5 0672 SHA --memory 8 -- python3 libraries/jvm/tests/package_check.py`, Gradle publishes the artifacts into a scratch Maven repository. A clean Gradle consumer and a clean Maven consumer each resolve them and run a named call on Java 22 and on the pinned newest JDK, with no library path override. On current main no Gradle or Maven build exists, so the consumers cannot resolve anything.
- The installed JVM check (`sh libraries/jvm/check.sh`) needs bubblewrap and is declared for `linux-x86_64` only, so it runs at the 0619 container checkpoint on Yellow (x86_64).

## Evidence

- Order (coordinator, 2026-10-19): this packaging ticket lands before the call ticket 0673, so 0673 can prove its named calls on the M5 through the Gradle and Maven consumers in `tests/package_check.py`.
- **No standard build tool.** `libraries/jvm/build-session.py` calls `javac`, `kotlinc` and `scalac` directly, found through `THINKTHEN_JDK_HOME`, `THINKTHEN_KOTLIN_HOME` and `THINKTHEN_SCALA_HOME`, and `libraries/jvm/tests/toolchains.py` maps `THINKTHEN_SESSION_JDK_HOME` onto `THINKTHEN_JDK_HOME`. `sdlc/scripts/release-workflow` builds the release JARs through `build.sh` and checks them with `tests/package_check.py --session`.
- **Java version.** `pom.xml` sets `thinkthen.session.jdk` to 22, and `toolchains.py` reads it as `JDK_FLOOR`. Java's foreign-function API became final in Java 22; on Java 21 it is a preview feature that needs `--enable-preview`, which no library can require of its callers. `sdlc/tools.json` pins only Temurin 25. The floor is a minimum host version and stays under `sdlc/decisions/2026-10-19-no-version-numbers-before-1-0.md`.
- **Missing dependencies.** The pom declares the native classifier JARs and only optional coroutines. It declares no Kotlin or Scala standard library, and the Scala classes carry no Scala-version suffix.
- **Collision partner.** 0722 also edits `libraries/jvm/check.sh` and `libraries/jvm/tests/`; the coordinator runs the two one after the other.
