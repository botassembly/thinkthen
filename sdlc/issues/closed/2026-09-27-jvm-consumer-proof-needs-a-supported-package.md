# A JVM consumer works through C, but no supported package exists

Status: closed 2026-09-30. Merged into `../2026-09-26-language-packages-need-a-release.md`, which keeps this language's open release items and limits.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 289 (renumbered from a collided 275 claim) now supplies Java, Kotlin, and Scala implementations, JAR-shaped package artifacts, documentation, tests and local archive rehearsal on Beelink. Ticket 0249 supplied reviewed integration. Release packaging and final-release verification remain before supported JVM use.

## Evidence

Experiment `experiments/289-thinkthen-jvm-c-interface/` covers two accepted stages on JDK 21.0.12.1 (preview FFM), Kotlin 2.4.20, and Scala 3.9.0, Linux x86_64 glibc, direct compiler invocations only. Ian installed the JDK; the Kotlin and Scala archives are recorded with hashes under `~/.local/opt`.

Stage one pinned main `a057c594` and answered the JVM-specific risk: platform threads keep thread-local C errors across a split call/read, but virtual threads do not. Retained runs record a carrier change and wrong error slots both with and without carrier identity changing. The door pins with a monitor on the calling thread from before the downcall through the error copy; a 32-virtual-thread mixed-failure check passes. Pinning consumes carrier capacity; a one-entry native helper returning status and copied error data is the unbuilt production alternative. Go needed the same class of fix in experiment 274.

Stage two pinned main `95f0458b` and delivered `stage2/package/` (door and facade sources, three deterministically built JARs, examples, license) plus rehearsal artifacts with seventeen verified hashes and byte-reproducible archives. Three bwrap namespace consumers — one per language, unrelated paths with spaces, fresh caches, no visible Cargo, rustc, repository or source export — each compiled against the packaged JARs, loaded the unpacked native archive, and ran the example, the full matrix, the one-MiB refusal plant, a separate concurrent-callers check, a planted classifier negative, and the real held-cancellation child: 68 independently counted arrivals per consumer. Wrong-artifact refusals cover stale sources, tampered archives with updated hashes, both wrong header versions at parser and compiler, decompressed private bytes, and source-tree fallback. Timeout descendants stopped while an unrelated control survived. ASan checked a C caller and detected a deliberate misuse.

The parent independently verified all input blobs at both pins, all manifest hashes, archive members and JAR rebuild equality, and reran both gates with identical results. Two fresh read-only reviews accepted each stage; the stage-two reviews returned ACCEPT with no findings.

## Sealed native blocker evidence

This section records the pre-fix failure. Ticket 0166 repaired it; the post-fix verification below records the passing strict proof.

At the sealed stage-two pin, a scalar C call could return success after its token fired during an accepted held request. All three languages reproduced it through their own cancellable facades, as do Zig, direct ctypes, and Go in experiments 273 and 274. [The closed cancellation issue](2026-09-26-cancelled-c-scalar-call-can-return-success.md) records repair. The sealed stage-two gate recorded `FINDING` and exited 1; no wrapper workaround was available at that pin.

## Handoff and remaining work

The historical experiment copy instruction is complete and superseded by `libraries/jvm/` and [ticket 0249](../../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, Maven distribution and other-host proof.

## Limits

Java 21 FFM is a preview; behavior on final-FFM JDKs is untested. Concurrent engine close is unsupported; callers join first. The one-MiB JSON bound is refusal, not large-result support. Kotlin and Scala do not repeat every Java matrix assertion; several JSON checks are shape or membership-only. No runtime ABI identity handshake, Rust allocator instrumentation, other platforms or toolchain versions, performance, or live-model quality is claimed. Synthetic loopback replies do not measure model accuracy.

## Post-fix verification (2026-09-27)

Ticket 0166 landed the engine fix. A parent-verified rerun at pin `22f36d0006fd34e7390a71d15c0458e493f3e844` (header `7fdca29a...`) rebuilt the native library and all three JARs and passed the complete copied gate with exit 0: strict held scalar returns `THINKTHEN_ECANCELLED` with untouched seeded native outputs, fresh-token recovery passes, held bulk returns 5, held deadline returns 3, and all 35 outer plus 33 child receipts match. Evidence: `post-fix/POST-FIX-REPORT.md`, worker gate `post-fix/logs/gate-20260927T130613Z`, parent rerun `post-fix/logs/gate-20260927T132352Z`. Integrators carry the strict-pass expectations into the product ticket and rerun on its final release pin.

Remaining release work is JAR and native-archive distribution, final-release rebuild, and the preview-FFM/carrier/close/one-MiB limits recorded in the handoff.

## Dependencies and installation (Beelink, recorded 2026-09-27)

Host: Ubuntu 24.04.3 LTS, kernel 6.17.0-35-generic, glibc 2.39, x86_64. JDK: `openjdk-21-jdk` 21.0.12.1 via apt (Ian installed). Kotlin 2.4.20 and Scala 3.9.0 from upstream archives unpacked at `~/.local/opt` with `~/.local/bin` links; archive SHA-256 hashes recorded in the experiment's `inputs/toolchain.json`. FFM is preview in Java 21: every compile and run carries `--enable-preview` and `--enable-native-access`. Native rebuild needs Rust 1.95.0 with an offline Cargo registry copy, clang 18.1.3, bwrap, Python 3.12. The package ships as JARs plus a matching native C shared archive; the FFM loader cannot use a static C archive.

## CI and release direction (Ian, 2026-09-27)

Each port must state its dependencies and how they were installed, and the product must also build and release on GitHub Actions `ubuntu-24.04` runners (JDK 21 is preinstalled; pin Kotlin and Scala versions in the workflow), publish native archives through GitHub Releases, and distribute JARs directly or through the JVM repository the queue owner chooses. The queue owner owns the workflow, release, and publishing tickets; nothing is published from the experiments.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope; packaged Kotlin/Scala examples fixed in repin only). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Landed product source, ticket 0249

The reviewed first batch imports `repin/package/` into `libraries/jvm/`, builds the local Java/Kotlin/Scala JAR bundle and records Maven metadata at `io.github.botassembly:thinkthen-jvm`. Java 21 preview FFM consumers pass 60 exact arrivals apiece through isolated installations. The carrier diagnostic lives in test code outside the product JAR. Settings, named errors, borrowed facts, member-order tolerant result decoding and the J1 public-binding corpus pass on Linux x86_64. The product build receipt is `sdlc/records/0249-csharp-jvm-build.md`. Ticket 0249 source integration landed. Maven publication, native release packaging and other hosts remain open.
