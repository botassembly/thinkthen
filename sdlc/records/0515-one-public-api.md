# Remove dead binding code

## JVM demo entrypoints

The bounded JVM slice starts from `c85666963`. It removes test programs from the shipped Kotlin and Scala sources while retaining every facade method and the first-class function wrappers. Later API removal follows each language's installed migration.

`libraries/jvm/kotlin/KotlinCaller.kt` shrinks from 59 to 27 lines. `libraries/jvm/scala/ScalaCaller.scala` shrinks from 62 to 23 lines. These files remove 71 shipped source lines. Their test programs move to `libraries/jvm/tests/KotlinConsumer.kt` and `ScalaConsumer.scala` without copying the product facade definitions.

`libraries/jvm/tests/installed.py` copies those test consumers into the isolated installed project. `consumer-run.py` compiles and runs their distinct entrypoints against the product JARs. Their assertions retain cancellation, fired-token refusal, recovery and JSON-call coverage. Removing an unused Kotlin import leaves one extra Kotlin source line and three extra Scala source lines across product and tests because each installed consumer now imports its own dependencies. The existing `ratchet.kt.json` and `ratchet.scala.json` record those measured totals. The Python package-exclusion and installed-consumer changes add one net line; `ratchet.py.json` records that measured total.

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

The managed package self-test and all 35 registry self-test cases pass. Policy checks and the child-environment check pass. The lint prefix before the surface registry passes at `02aa3e0cbc6e6aa53124dc19ddab993aad8eaf42`. The subsequent change to `ratchet.py.json` affects the surface registry, while the prefix's checked source and runner inputs remain unchanged. At `e58338a93e6db7541654ca787ec89a83552b2107`, the existing surface registry and remaining lint commands pass: offline dependency checks and their source plant, formatting, Clippy, documentation, and the public inventory with its four planted refusals. Later changes to this record do not alter those checked inputs.

## Unshipped PHP carriers

A fresh read-only reviewer accepted the complete PHP slice at 6a41b91c, including its README correction. Integration preserves the accepted source; PHP's measured ceiling, shell syntax and whitespace checks pass. No new full installed-package run is claimed.

The PHP slice starts from `3f409a253`. It removes the 21 private PHP files under `libraries/php/src/complete/` other than `models.php`: `annotation_specs.php`, `answers.php`, `atomic_results.php`, `bootstrap.php`, `call.php`, `decision.php`, `dictionaries.php`, `entities.php`, `errors.php`, `facts.php`, `members.php`, `plans.php`, `question_specs.php`, `questions.php`, `read.php`, `requests.php`, `set_results.php`, `sources.php`, `type_dispatch.php`, `types.php` and `values.php`. They contain 2,646 handwritten physical source lines, including 2,564 nonblank lines. The obsolete private `fixtures/complete_carriers.php` adds 81 removed lines. The existing PHP ceiling drops from 4,404 to 1,759 nonblank lines.

`autoload.php` loads only `models.php` from that directory. Its runtime included-file list confirms the other files are unreachable from the public entrypoint. `release-pack` copies only `models.php`; `release-go-cpp-pair`, `fixtures/installed.py` and `fixtures/portable_batch.py` independently declare the same shipped members. The removed files are reached only through the private carrier fixture and its source lint. This slice removes that fixture and its obsolete gate invocation. `models.php` stays unchanged because the live native result implementation uses its `FailureKind`. `fixtures/complete.json` stays because Dart and Flutter consume it. Public PHP implementations, package inventories and frozen C exports stay unchanged.

On source commit `8d3282a1d`, both copied installed packages pass their declared member, byte, privacy, header and native-library checks and their three counted public calls. The portable batch and named-backend checks pass legacy and complete calls with two exact requests per route, selected path and bearer, cancellation, usage refusal and secrecy. The shared J1 suite passes 101 schema cases and 29 public binding cases, including the plan, limits and annotation assertions. PHP ABI checks pass 128 actual FFI layouts, 15 constants and 93 prototypes; the frozen C export check passes. PHP syntax, shell syntax, both PHP source ratchets, the child-environment check and the existing surface registry pass. The public inventory checks 1,898 declared items and refuses its four plants. The unchanged policy inputs pass with existing size warnings; this slice grows no warned file.

The broader PHP matrix stops in the unchanged direct example at `examples/direct.php:14`. Its strict facts-key assertion excludes the native facts fields `largest_request_bytes`, `largest_request_estimated_input_tokens` and `token_estimate_method`. The call returns the expected true value and reports two counted arrivals before that fixture refusal. The example has identical Git blob `5a9efe4b429c42d4b6f0d918f26dbdcc4f5112ff` before and after this slice; its loaded public sources and native artifact also stay unchanged. This slice does not alter the result contract or weaken that assertion.

`libraries/php/README.md` removes two stale sentences that described the deleted private carrier readers and their restrictions. Its canonical parity guidance remains unchanged. This documentation correction passes whitespace and referenced local-path checks; it changes no implementation or fixture, so the source-commit evidence above remains applicable.

The other named candidates require language migration. C++ `door.hpp` includes `complete.hpp`, which CMake and the source archive ship. Swift's package target compiles `Complete.swift`, and both source packaging and installed consumers include it. Zig's public root imports `complete.zig`, its source archive ships it, and installed carrier tests use it. Their audit labels do not establish deadness, so this slice retains them. Dart requires its own import and installed-consumer audit. Frozen C export documentation remains a separate slice.
