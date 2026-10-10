# 0504: Make Java, Kotlin and Scala thin and first-class

Status: OPEN.

Milestone: 0.2

Depends on: 0516
Depends on: 0517

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 9cedc050c69fa3f1301dc142245526f2540f6295, accept

## Outcome

Java, Kotlin and Scala callers declare one Maven dependency, call the ten functions with native values, read generated typed results and handle typed failures. Each language cancels and cleans up in its own idiom. The jars run on current JDKs without preview features and carry their native libraries. Hand-copied layouts, readers and old public names are gone.

## Evidence

- Starts from: The [2026-10-09 decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). `libraries/jvm/build.sh` compiles with `--enable-preview --release 21`, so the jars break on JDK 22 and later. Native calls lock on the current thread and block virtual threads. Layouts are hand-copied from the C header.
- Keeps: All ten functions, files and images, failure facts, cache and replay behavior, installed package checks, and the Kotlin and Scala demo removal already landed under 0515.
- Changes: Follow the 0516 pilot pattern on 0503's session with 0513's generated results. Meet the caller acceptance and the Java, Kotlin and Scala sections of `../../libraries/BINDING-AUTHOR.md`. This ticket owns:
  - stable foreign-function APIs with the JDK floor declared by 0517 before coding;
  - one shared transport and ownership layer for the three languages, with each keeping its own public types and scheduling;
  - the JVM target templates and generated outputs;
  - native input conversion, typed errors, cancellation and cleanup;
  - the Maven native-jar packaging slice under 0517's design;
  - the JVM README, with a short old-to-new call mapping;
  - removal of old public names after installed parity.
  One public API is one coherent family of named typed calls. Claim `libraries/jvm/**` narrowed per slice, naming files before each slice.
- Proof: Supported-platform rulings stay explicit in the package design and the installed checks. Shared conformance runs through installed Java, Kotlin and Scala consumers, covering absent, null, failures, unknown output fields, located files, images and cancellation. One installed held-provider case per async idiom shows unrelated work progressing, and cancel and cleanup returning before the provider is released. Pending final facts stay pending. A required host case that does not run is a failure. Record handwritten code removed and added, counting templates, in the landing record.
- Defers: Windows qualification to 0385. Final distribution assembly to 0530. Dart and Flutter to 0522, Swift to 0523, Go to 0524, C++ to 0525, PHP to 0526, Rust and Rust Polars to 0527, Objective-C to 0518.

## Progress

- 2026-10-10 started
- 2026-10-10 landed 08babf732; next: JVM local Maven artifacts and inventory-derived native dependencies are landed; focused Java, Kotlin and Scala consumers pass. Finish generated typed inputs, native language results and stable default build routing; final qualification remains held.
