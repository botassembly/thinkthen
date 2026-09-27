# A Go consumer works through C, but no supported package exists

Status: the local Go package experiment is complete, with a verified native cancellation blocker owned separately. The queue owner decides the integration ticket and release. Ian authorized the local engineering and testing, not publication. He can overturn the thread-pinning choice and the explicit native-archive route.

## Gap

The ideal state makes the C interface the route to additional languages. Experiment 274 now supplies the Go implementation, package metadata, examples, documentation, tests and local archive rehearsal on Beelink. The repository still needs reviewed integration, the native cancellation repair, and final-release verification before offering supported Go use.

## Evidence

Experiment `experiments/274-thinkthen-go-c-interface/` covers two accepted stages on Go 1.22.2, Linux x86_64 glibc, cgo, standard library only.

Stage one pinned main `48fd034f` and answered the Go-specific risk: goroutines move between OS threads across cgo calls, and thread-local C error metadata does not follow. A failing call reported code 1 on its calling thread and a different code after migration. The wrapper pins with `runtime.LockOSThread` around the call and error copy. A one-shot C helper is the untested alternative. This lesson applies to the deferred JVM experiment.

Stage two pinned main `19ca8302` and delivered `stage2/package/` (seven module files), rehearsal artifacts with thirteen verified hashes and byte-reproducible native archive, and an offline gate. Four bwrap namespace consumers — two shared, two static-C, fresh caches, unrelated paths with spaces, no visible Cargo, rustc, repository or source export — each ran the example, a race-enabled matrix covering all ten JSON verbs plus typed paths, the reconciled grammar (`find` with `none` answering JSON `null`, annotate `on` selecting JSON text), failing-middle bulk, transport close and retry statuses, malformed backend JSON, maximum deadline, forced bulk completion ordering, and fresh-token recovery: 45 independently counted arrivals per consumer. Wrong-artifact refusals cover stale sources, tampered archives, changed digests and both wrong header versions at parser and cgo compile. Planted timeout descendants stopped while an unrelated control survived. ASan checked a C caller and detected a deliberate misuse.

The parent independently verified all input blobs at both pins, all manifest hashes, archive members and exports, and reran the complete gate with identical results. Two fresh read-only reviews accepted the bounded engineering with no findings.

## Native blocker

A scalar C call can return success after its token fires during an accepted held request. All four stage-two consumers reproduce it through Go, as do Zig and direct ctypes in experiment 273. [The cancellation issue](2026-09-26-cancelled-c-scalar-call-can-return-success.md) owns repair. The Go wrapper reports exactly what C returns; the gate records `FINDING` and exits 1; no Go workaround exists.

## Handoff and remaining work

Local, unpushed deliverables live in the experiment folder: `stage2/package/`, `stage2/artifacts/` with its manifest, `stage2/fixtures/` and `stage2/gate.sh`, and `stage2/HANDOFF.md` with exact destinations. The integration ticket copies `package/` into `libraries/go/`, relocates fixture controls into product offline tests, adapts release packaging and documentation, and adds the surface registry entry. After the native fix, replace the gate's exact known-failure expectation with the strict cancellation contract and rerun everything on the final landed release commit. Do not publish the rehearsal archives.

## Limits

Static C is not a fully static executable. Go race checks do not instrument the native Rust allocator; ASan covers the C caller only. No runtime binary ABI identity query exists. Forced wrapper allocation exhaustion was source-reviewed, not injected. Other Go versions, musl, ARM, macOS, Windows, performance, live-model quality and exhaustive diagnostic secrecy remain unproved. Synthetic loopback replies do not measure model accuracy.
