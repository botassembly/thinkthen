# A Go consumer works through C, but no supported package exists

Status: the local Go package experiment is complete. Ticket 0166 repaired the native cancellation blocker, and a post-fix rerun at main `22f36d00` passed the strict contract. The queue owner decides the integration ticket and release. Ian authorized the local engineering and testing, not publication. He can overturn the thread-pinning choice and the explicit native-archive route.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 274 now supplies the Go implementation, package metadata, examples, documentation, tests and local archive rehearsal on Beelink. The repository still needs reviewed integration, release packaging and final-release verification before offering supported Go use.

## Evidence

Experiment `experiments/274-thinkthen-go-c-interface/` covers two accepted stages on Go 1.22.2, Linux x86_64 glibc, cgo, standard library only.

Stage one pinned main `48fd034f` and answered the Go-specific risk: goroutines move between OS threads across cgo calls, and thread-local C error metadata does not follow. A failing call reported code 1 on its calling thread and a different code after migration. The wrapper pins with `runtime.LockOSThread` around the call and error copy. A one-shot C helper is the untested alternative. This lesson applies to the deferred JVM experiment.

Stage two pinned main `19ca8302` and delivered `stage2/package/` (seven module files), rehearsal artifacts with thirteen verified hashes and byte-reproducible native archive, and an offline gate. Four bwrap namespace consumers — two shared, two static-C, fresh caches, unrelated paths with spaces, no visible Cargo, rustc, repository or source export — each ran the example, a race-enabled matrix covering all ten JSON verbs plus typed paths, the reconciled grammar (`find` with `none` answering JSON `null`, annotate `on` selecting JSON text), failing-middle bulk, transport close and retry statuses, malformed backend JSON, maximum deadline, forced bulk completion ordering, and fresh-token recovery: 45 independently counted arrivals per consumer. Wrong-artifact refusals cover stale sources, tampered archives, changed digests and both wrong header versions at parser and cgo compile. Planted timeout descendants stopped while an unrelated control survived. ASan checked a C caller and detected a deliberate misuse.

The parent independently verified all input blobs at both pins, all manifest hashes, archive members and exports, and reran the complete gate with identical results. Two fresh read-only reviews accepted the bounded engineering with no findings.

## Sealed native blocker evidence

This section records the pre-fix failure. Ticket 0166 repaired it; the post-fix verification below records the passing strict proof.

At the sealed stage-two pin, a scalar C call could return success after its token fired during an accepted held request. All four stage-two consumers reproduced it through Go, as do Zig and direct ctypes in experiment 273. [The closed cancellation issue](closed/2026-09-26-cancelled-c-scalar-call-can-return-success.md) records the repair. The stage-two Go wrapper reported exactly what C returned; the sealed stage-two gate recorded `FINDING` and exited 1; no Go workaround was available at that pin.

## Handoff and remaining work

Historical evidence remains local and unchanged under `stage2/`, including its package, artifacts, fixtures, gate and handoff. Product integration must use `post-fix/package/`, `post-fix/fixtures/` and `post-fix/gate.sh`, which carry the passing strict expectations. The integration ticket copies `post-fix/package/` into `libraries/go/`, relocates fixture controls into product offline tests, adapts release packaging and documentation, and adds the surface registry entry. The post-fix package also contains the context-watcher correction described below; the sealed stage-two package lacks it. The integration ticket records the final Go API behavior. The post-fix experiment copy already requires strict cancellation success and passed. Product integration must carry that contract and rerun everything on the final release pin. Do not publish the rehearsal archives.

## Limits

Static C is not a fully static executable. Go race checks do not instrument the native Rust allocator; ASan covers the C caller only. No runtime binary ABI identity query exists. Forced wrapper allocation exhaustion was source-reviewed, not injected. Other Go versions, musl, ARM, macOS, Windows, performance, live-model quality and exhaustive diagnostic secrecy remain unproved. Synthetic loopback replies do not measure model accuracy.

## Post-fix verification (2026-09-27)

Ticket 0166 landed the engine fix. A parent-verified rerun at pin `22f36d0006fd34e7390a71d15c0458e493f3e844` rebuilt the native library offline and passed the complete copied gate with exit 0 across two shared and two static-C isolated consumers (47 arrivals each), plus a direct-C null-token deadline probe returning code 3 with sentinels intact and a three-trial direct-C scalar cancellation proof returning code 5 with sentinels untouched. Evidence: `post-fix/POST-FIX-REPORT.md`, worker gate `post-fix/logs/gate-20260927T140304Z`, parent rerun `post-fix/logs/gate-20260927T141134Z`.

New Go binding finding the fix exposed: the sealed wrapper maps one context deadline to both `deadline_ms` and an asynchronously fired cancel token, so post-0166 a `WithTimeout` expiry reported cancellation code 5 instead of deadline code 3. The post-fix copy corrects the watcher to fire only on `context.Canceled`; expiry spends the native budget and returns code 3. The product queue owns the final Go API decision; the correction is documented in `post-fix/POST-FIX-REPORT.md`. The recognize schedule now sends two requests per name on main; consumers must re-derive counts per pin.

## Dependencies and installation (Beelink, recorded 2026-09-27)

Host: Ubuntu 24.04.3 LTS, kernel 6.17.0-35-generic, glibc 2.39, x86_64. Go 1.22.2 from the Ubuntu package at `/usr/bin/go` with `CGO_ENABLED=1`; cgo uses the system gcc. Native rebuild needs Rust 1.95.0 with an offline Cargo registry copy, clang 18.1.3 for ASan, bwrap for isolated consumers, Python 3.12 fixtures. The package ships as a Go module zip plus a matching native C archive consumed through a local file proxy; standard library only, no third-party modules.

## CI and release direction (Ian, 2026-09-27)

Each port must state its dependencies and how they were installed, and the product must also build and release on GitHub Actions `ubuntu-24.04` runners (pin Go 1.22; Rust toolchain for the native rebuild), publish native archives through GitHub Releases, and let consumers install directly or through the Go module path. The queue owner owns the workflow, release, and publishing tickets; nothing is published from the experiments.

## Post-J1 re-pin pass (2026-09-28, pin 6dbdf03f)

The accepted four-consumer gate passed unchanged in every behavioral assertion at the post-J1 pin (39 receipts, exact 47-arrival multisets per consumer, strict held scalar 5 / bulk 5 / deadline 3 with recovery, direct-C proofs). One mechanical adaptation: the ABI preflight now expects twenty exports for `thinkthen_engine_new_with`, which the Go wrapper still does not bind — that stays with the J8 ticket. One drift finding filed separately: recognition relation-pair request shape changed (see `2026-09-28-recognition-relation-requests-changed-shape-on-main.md`). Evidence: local experiment 274, `post-j1/POST-J1-REPORT.md`.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, multiset 47->39). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Product source candidate (2026-09-28)

Ticket 0249 copied the accepted repin package into `libraries/go/` and adapted a product-owned offline gate. At source `976bcd75`, the gate passed four copied-module consumers with 39 exact full request bodies each, plus one external module call per consumer, 55 schema cases and 29 executable public Go cases, current 30-export native ABI, settings construction, named error kinds and copied failure facts. See `sdlc/records/0249-go-cpp-{preflight,build,review}.md`. Fresh Medium review accepted source `d7723841`; Go registration passed focused policy and registry checks after Swift/Zig landed. Narrow registration review is pending. The original final-release pin, `ubuntu-24.04` CI/release run, native archive distribution, and module-tag/direct-install criteria remain open. No package was published.
