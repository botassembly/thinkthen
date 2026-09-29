---
flow: build
priority: 272
opens: sdlc/issues/2026-09-27-csharp-consumer-proof-needs-a-supported-package.md sdlc/issues/2026-09-27-jvm-consumer-proof-needs-a-supported-package.md
---

# 0272: Prepare C# and JVM managed archives for the Linux x86 release workflow

Status: prospective design at main `5a1a6e15`, after 0271 and pending fresh independent review. No SDK download, compilation or runner qualification. [Preparation](../records/0271-0272-language-runner-preparation.md) maps the source/C handoff. Ian's SQL/DataFrame hold applies.

## One source pin and one native archive

[0262](../records/0262-csharp-jvm-release-build.md) already proves a local `c csharp jvm` build, exact package inventory and installed C#/Java/Kotlin/Scala calls. The current manylinux container has no recorded .NET, JDK, Kotlin or Scala setup. The source-only wrapper transport of 0271 cannot produce the compiled nupkg/JAR members. `release-container` currently deletes its scratch Git tar after the container exits. First persist one checked tar in a job-owned handoff, verify its tar ID against `resolve.outputs.sha`, and pass those exact bytes to the container. Use a bounded managed build/assembly step on the x86 Ubuntu build runner **after** its manylinux C archive is available. Build managed members from a fresh scratch extraction of those same tar bytes; compare the **entire** extraction with the tar before compiling. Reject a checkout override or unmatched source. Verify the existing container C archive/sidecar and take its digest as the only native identity. Compile the nupkg and three JARs with exact selected SDKs, then assemble the current fixed wrapper inventories and manifests from checked archived source plus fresh compiler outputs. Refuse an existing/linked C or output, a second C build, `--reuse`, untrusted compiled input, mismatched SHA and fallback to checkout product outputs. Preserve C's manylinux glibc floor; the outer Ubuntu step supplies no C library. The existing `release-go-cpp-pair csharp-jvm` remains the bundle-consistency validator, not a source signature.

Keep `libraries/jvm/build.sh` release output owned by a fresh empty scratch directory via `THINKTHEN_JVM_OUT`; it already refuses nonempty or linked output. `package_check.py` must compare all three JAR member lists with that fresh output. The nupkg is built by `dotnet pack libraries/csharp/ThinkThen.csproj -c Release` from archived `src/ThinkThen.cs`, README and LICENSE, with an isolated empty local restore source. It contains `lib/net8.0/ThinkThen.dll` and no native member. The JVM wrapper contains door/Kotlin/Scala JARs, POM, README, LICENSE and manifest, all as in 0262. Verify compiled inner identities and member hashes, then outer sidecars and exact selected family names before smoke and draft.

The runner setup must resolve .NET SDK 8.0.131; JDK 21.0.12.1 `javac`, `jar`, `java` with preview FFM/native access; Kotlin 2.4.20 compiler and matching `kotlin-stdlib.jar`; Scala 3.9.0 compiler and matching `scala.jar`; Python, `/usr/bin/bwrap`, `nm`, `tar`, `flock` and the checkout-owned conformance backend prerequisites. Use existing `THINKTHEN_DOTNET` and `THINKTHEN_{JDK,KOTLIN,SCALA}_HOME` selectors, print actual paths/versions and refuse mismatch before compilation or installed consumers. The 0262 local receipts and the mutable Ubuntu image inventory do not authenticate a runner install. Before any download-based setup, record official immutable URLs and checked digests for every absent SDK archive; no `latest`, unverified bootstrap or tolerated exit 77. Lack of those inputs stops runner execution, not static preparation.

## Gate, proof and scope

Extend the **landed** selected-family gate to require the exact x86 C# and JVM basenames/sidecars, their `csharp-jvm` pair preflight and resolved SHA manifests before `smoke-bundle` and before draft `collect`. Inspect all selected prefixed entries the copier sees, including extra suffixes, wrong targets and links; reject these families on the other targets. Keep the four-folder guard and every earlier selected-language gate. `release-smoke` already conditionally selects both installed `check.sh` files. Do not run its SQL/DataFrame aggregate during the hold.

Static proof under the hold: a fixture supplies a checked source tar and independently checked C archive, observes that managed assembly selects them and fresh outputs, then changes one archived product member, plants `Stale.class`, substitutes a valid differently hashed C archive, omits a family and adds an extra selected-family entry. Each fails before package output, consumer or collection as applicable. Use an independent observer, not a mock that enforces the checked rule. Keep any data-generating lock check in a separate scratch extraction from exact-member assertions. Register focused cases in the routine workflow checker; run pages, tickets and diff checks. Do not rerun the 0262 source matrices or add public callers.

Later permitted execution must record actual managed SDK source/version and runner paths, fresh nupkg/JAR hashes, container C/header/shared/static hashes and glibc floor, pair result, and existing installed consumer observations: two C# exact bodies/connections and one Java, Kotlin and Scala body/connection each, with the 0260 five-text mixed-answer oracle retained. A separately authorized Actions rehearsal and final-release-pin, NuGet/Maven or direct distribution and other-host criteria remain open in the original issues.

Prospective implementation claim after accepted design and cleared shared files: `.github/workflows/release.yml`, `sdlc/scripts/{release-pack,release-container,release-workflow,workflows}` plus one narrow managed assembly helper only if the reviewed design needs it, focused workflow fixtures and this ticket/build record. Existing `libraries/{csharp,jvm}` product, tests and C source stay read-only unless a concrete deficiency earns an exact claim. Coordinate with 0271; fresh independent High review checks the new source/native boundary, selected assets and first-step release controls. No SQL, DataFrame, publication or registry edit.

## Evidence

- Starts from: [0262 local build](../records/0262-csharp-jvm-release-build.md), [0260 managed host proof](../records/0260-managed-host-proof.md), current packer/build scripts and the two original consumer issues.
- Keeps: One manylinux C, clean source archive, fresh managed output, fixed pair validator and installed package callers.
- Changes: Proposes a bounded managed assembly handoff and selected-family workflow gate.
- Proof: Registered static source/C/output and gate refusals under the hold; later selected installed calls on the actual runner.
- Defers: SDK procurement and actual runner, container, Actions, final pin, distribution and other-host qualification.
