# JVM demo entrypoints

The bounded JVM slice starts from `c85666963`. It removes test programs from the shipped Kotlin and Scala sources while retaining every facade method and the first-class function wrappers. Later API removal follows each language's installed migration.

`libraries/jvm/kotlin/KotlinCaller.kt` shrinks from 59 to 27 lines. `libraries/jvm/scala/ScalaCaller.scala` shrinks from 62 to 23 lines. These files remove 71 shipped source lines. Their test programs move to `libraries/jvm/tests/KotlinConsumer.kt` and `ScalaConsumer.scala` without copying the product facade definitions.

`libraries/jvm/tests/installed.py` copies those test consumers into the isolated installed project. `consumer-run.py` compiles and runs their distinct entrypoints against the product JARs. Their assertions retain cancellation, fired-token refusal, recovery and JSON-call coverage.

`libraries/jvm/tests/package_check.py` rejects the former Kotlin and Scala demo classes independently of the compiled-directory comparison. The original built JARs failed this assertion with `demo entrypoint escaped product JAR`. `sdlc/scripts/release-managed-pair.py` and its existing self-test remove those classes and Scala metadata from their declared archive inventories. The actual rebuilt JARs match those inventories.

The rebuilt package passes its member, metadata, private-byte and native ABI checks and planted ABI mutations. The existing bounded installed selectors pass for Java, Kotlin and Scala with one exact counted request per consumer. Both moved test consumers pass against installed JARs using the existing sandbox, backend and process-group helpers, including held cancellation and recovery with ten exact arrivals. The J1 checks pass through all three public bindings with 101 schema cases and 29 runtime cases each.

The full JVM check stops in the unchanged Java `Matrix.requests[11]` fixture at `Matrix.java:77`. It supplies an encoded object as a text record to an annotate question with `on: /body`. The native engine rejects it: ``question `check` reads `on`, and this record's evidence is text with no members``. A focused call using the unmodified fixture reproduces that refusal. The baseline and current Matrix source share Git object `0110a72ae40d4d6a56ae08070f3caa302a29eb74`. This slice changes no native admission behavior or Java request handling and does not repair that fixture.

The full native parity portion of `public_types.py` was stopped by the coordinator after the affected installed and J1 checks passed. Its Java rows passed through the observed image cases; the full native parity matrix did not complete. Final installed parity belongs to integration after the language migrations.

The existing local `build.sh` retains stale compiled classes. This task removed only the demo class and Scala metadata outputs it created during its baseline build before rebuilding. The package check rejects retained demo outputs. Release builds require an empty `THINKTHEN_JVM_OUT`, so they do not retain these classes.

The unchanged failing request is:

```json
{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?","on":"/body"}}},"records":["{\"body\":\"annotate-on\",\"hidden\":\"not-sent\"}"]}
```

Frozen C compatibility and legacy host-language APIs remain under the ticket's review amendment. This slice changes no public SDK implementation or function wrapper.
