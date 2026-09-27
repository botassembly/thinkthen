# A JVM consumer works through C, but no supported package exists

Status: the local JVM package experiment is complete, with a verified native cancellation blocker owned separately. The queue owner decides the integration ticket and release. Ian authorized the local engineering and testing and the toolchain downloads, not publication. He can overturn the FFM-over-JNI choice, the join-first close contract, and the one-MiB refusal bound.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 275 now supplies Java, Kotlin, and Scala implementations, JAR-shaped package artifacts, documentation, tests and local archive rehearsal on Beelink. The repository still needs reviewed integration, the native cancellation repair, and final-release verification before offering supported JVM use.

## Evidence

Experiment `experiments/275-thinkthen-jvm-c-interface/` covers two accepted stages on JDK 21.0.12.1 (preview FFM), Kotlin 2.4.20, and Scala 3.9.0, Linux x86_64 glibc, direct compiler invocations only. Ian installed the JDK; the Kotlin and Scala archives are recorded with hashes under `~/.local/opt`.

Stage one pinned main `a057c594` and answered the JVM-specific risk: platform threads keep thread-local C errors across a split call/read, but virtual threads do not. Retained runs record a carrier change and wrong error slots both with and without carrier identity changing. The door pins with a monitor on the calling thread from before the downcall through the error copy; a 32-virtual-thread mixed-failure check passes. Pinning consumes carrier capacity; a one-entry native helper returning status and copied error data is the unbuilt production alternative. Go needed the same class of fix in experiment 274.

Stage two pinned main `95f0458b` and delivered `stage2/package/` (door and facade sources, three deterministically built JARs, examples, license) plus rehearsal artifacts with seventeen verified hashes and byte-reproducible archives. Three bwrap namespace consumers — one per language, unrelated paths with spaces, fresh caches, no visible Cargo, rustc, repository or source export — each compiled against the packaged JARs, loaded the unpacked native archive, and ran the example, the full matrix, the one-MiB refusal plant, a separate concurrent-callers check, a planted classifier negative, and the real held-cancellation child: 68 independently counted arrivals per consumer. Wrong-artifact refusals cover stale sources, tampered archives with updated hashes, both wrong header versions at parser and compiler, decompressed private bytes, and source-tree fallback. Timeout descendants stopped while an unrelated control survived. ASan checked a C caller and detected a deliberate misuse.

The parent independently verified all input blobs at both pins, all manifest hashes, archive members and JAR rebuild equality, and reran both gates with identical results. Two fresh read-only reviews accepted each stage; the stage-two reviews returned ACCEPT with no findings.

## Native blocker

A scalar C call can return success after its token fires during an accepted held request. All three languages reproduce it through their own cancellable facades, as do Zig, direct ctypes, and Go in experiments 273 and 274. [The cancellation issue](2026-09-26-cancelled-c-scalar-call-can-return-success.md) owns repair. The gate records `FINDING` and exits 1; no wrapper workaround exists.

## Handoff and remaining work

Local, unpushed deliverables live in the experiment folder: `stage2/package/`, `stage2/artifacts/` with its manifest, `stage2/fixtures/` and `stage2/gate.sh`, and `stage2/HANDOFF.md` with exact destinations. The integration ticket copies `package/` into `libraries/jvm/`, adapts fixtures into product offline tests, decides JAR distribution and the native archive route, and adds the surface registry entry. After the native fix, update both the strict classifier and the ordinary Java/Kotlin/Scala held-call expectations, rebuild from the final landed release commit, and earn an all-pass gate. Do not publish the rehearsal archives or JARs.

## Limits

Java 21 FFM is a preview; behavior on final-FFM JDKs is untested. Concurrent engine close is unsupported; callers join first. The one-MiB JSON bound is refusal, not large-result support. Kotlin and Scala do not repeat every Java matrix assertion; several JSON checks are shape or membership-only. No runtime ABI identity handshake, Rust allocator instrumentation, other platforms or toolchain versions, performance, or live-model quality is claimed. Synthetic loopback replies do not measure model accuracy.
