---
flow: build
priority: 277
opens: sdlc/issues/2026-09-26-every-surface-should-give-back-run-facts.md
---

# 0277: Return owned facts from Go and PHP typed calls

Status: COMPLETE.

Opened as: 2026-10-11. Fresh High code review accepted `0d714cfee`; the coordinator integrated the reviewed implementation and focused proof. Broader issue and release qualification remain open.

## Outcome and retained behavior

Make each existing direct typed Go/PHP judgment method return its former value with its **own final call facts** from the same C `*_with_facts_opts` operation. Migrate `Decide`, `DecideMany`, `Recognize`, `Relate` and PHP counterparts together, including zero-count bulk. The Go return becomes `Result[T]` with typed `Facts`; PHP returns `['value'=>old value,'facts'=>associative facts]`. The successful JSON `Call`/`call` route already carries `value`/`facts` and keeps its string return. Existing Go `Error.Facts` and PHP `ThinkThenFailure::factsJson` keep started-failure facts; pre-start errors carry none. Answer order, probabilities, JSON value bytes, argument grammar, deadlines, cancellation, engine-close behavior, C ABI and every old C symbol remain as today. No bare Go/PHP compatibility alias is added.

The C facts keys are required `records`, `requests_sent`, `cache_answers`, `seconds`, with optional `input_tokens`, `output_tokens`, `model`. Go optional fields use nil pointers; PHP omits absent keys. Neither wrapper invents zero tokens, cost, request IDs, timing, digests or per-row details. Returned facts are owned and survive later same-engine calls and close. Go supports concurrent calls on one engine; PHP's current single-threaded FFI limitation remains. The full every-surface-facts issue stays open for its other carriers and richer-detail criteria. SQL/DataFrame work remains held.

## Implementation route and exact file inventory

1. Go: in `libraries/go/thinkthen.go`, add `Result[T]`/`Facts` and migrate the four typed methods to `thinkthen_decide_with_facts_opts`, `thinkthen_decide_many_with_facts_opts`, `thinkthen_recognize_with_facts_opts`, `thinkthen_relate_with_facts_opts`. Preserve C allocation and `enter`/`leave`/same-thread failure handling. Validate presence and types of all required facts keys, copy the independent counted facts string, then free it, and free structured result strings separately. The zero-count bulk path must still call C with facts slots. Keep shared ownership logic local; if the 393-nonblank-line source grows substantially, extract pure result/facts decoding into one small Go file rather than copying four ownership implementations. `Call` and JSON grammar remain unchanged.
2. PHP: in `libraries/php/src/ThinkThen.php`, add the four facts-returning `_opts` FFI declarations; migrate direct methods to an associative value/facts envelope. Allocate output pointer and length slots, copy the C-owned bytes with returned lengths, decode facts while preserving omitted keys, and free each native string once in `finally`. Keep the existing `fail()` borrowed-slot copy before another native operation; never return partially published success. The direct methods remain the only four typed verbs; `call` continues to serve the seven JSON-only verbs. `decideMany` is a bulk form of decide.
3. Migrate all current direct callers, not only decide. Go: `libraries/go/thinkthen_test.go`, `recovery_test.go`, `portable_batch_test.go`, `examples/decide/main.go`, `README.md`; inspect the source and installed selectors in `check.sh`, `fixtures/installed_release.py` and `fixtures/portable_batch.py` for compiled external callers. `fixtures/type_case.go` uses `Call` and needs no shape edit. PHP: `fixtures/matrix.php`, `portable_consumer.php`, `installed_consumer.php`, `examples/direct.php`, `README.md`; `fixtures/type_case.php` uses `call` and stays on its JSON envelope. Inspect `check.sh`, `fixtures/installed.py` and release selectors for copy/packaging registration. Edit fixture scripts only where the changed caller or derived member inventory requires it; 0276 owns their separate child-environment cleanup. Update measured Go/PHP and shared derived ratchets, and package source/member manifests only when changed bytes require them.

## Smallest outside-in proof

- Reuse the source matrix's exact mixed `first/second/third` bulk responses to pin ordered values and one batch facts object, with request count observed by its loopback controller. A single scalar, recognize and relate each pin value plus facts. Inspect the wrapper routes for one native call each; loopback counts corroborate no extra send. Use the current C header and library containing all four owned-facts `_opts` symbols; keep C's existing `tests/c/typed_facts.c` ownership/alias proof as underlying evidence. No generic facts cache.
- Add one missing-usage response variant to the existing synthetic reply fixture (or reuse its C controlled reply), then assert Go optional token pointers nil and PHP token keys absent while required counts remain. Do not change the provider protocol or construct a second paid request. Empty `DecideMany`/`decideMany` must return an empty value with `records=0`, zero sends and an owned facts object.
- Reuse each host's started backend-failure and later-call cases. Assert pre-start refusal has no facts; started failure has copied final facts; a later call, another same-engine failure and close leave earlier facts unchanged. Count loopback arrivals for failures and success; Go's existing overlapping same-engine case must retain distinct per-call facts. PHP proves sequential lifetime, not unsupported PHP concurrency. Preserve the existing native deadline/cancellation tests and C zero-output-on-failure rule.
- Run focused Go tests/`gofmt` and PHP syntax/fixture checks, exact changed-file child guard, source ratchets and current-header/export check. The installed package receipt later uses a **new matching C archive** and the migrated unrelated Go/PHP consumers under their existing selectors; accepted 0261/0264 installed receipts prove their old source only. Actual Linux runner, other-target binaries, registry publication and release workflows stay in their release issues. Do not repeat 0260's broad body corpus or old installed matrix for this design.

## Routing and deferred gaps

Root claimed the implementation after High design/API review and approval of ADR 0106; the [build record](../records/0277-go-php-call-facts-build.md) carries source and installed-source receipts for fresh High code review. No Ian question was needed for the delegated pre-0.1 return migration. Candidate follow-ons: C++/Dart typed wrappers, then C#/JVM typed wrappers, each with their own return-shape and owned-output proof against the same C API. Other wrapper families, SQL/DataFrame hosts, full detail, timing and cost stay in the open issue or existing holds.

## Evidence

- Starts from: Ian's [every-call ruling](../issues/closed/2026-09-26-every-surface-should-give-back-run-facts.md), ADR 0101's **C-only** compatibility exception, current C header/typed-facts proof and the [0260 wrapper host proof](../records/0260-wrapper-host-proof.md).
- Keeps: The four typed values and order, JSON `Call`/`call` envelopes, started-failure facts, C ABI and old C exports, existing cancellation/deadline and host concurrency limits.
- Changes: Migrates pre-0.1 Go/PHP typed success returns that own value and final facts from one C facts-returning operation, with all direct examples and consumers migrated.
- Proof: Focused existing-fixture scalar/four-method, ordered bulk, empty bulk, missing-usage omission, failure/next-call lifetime and same-engine Go concurrency receipts, plus matched source and copied installed-source checks; release archives stay separate.
- Defers: Fresh High code review and landing; full per-row detail, cost, vendor timing/IDs, other wrappers, release archive/actual runner qualification and held SQL/DataFrame work.

## What the build taught us

The old release C library in this lane predates the facts exports, so current C was built locally under the lane lock for source proof. The existing typed relate case has two input entities but one logical question in `facts.records`; tests pin the native count rather than substituting input length. A required Go numeric JSON key needs an explicit presence and non-null check because Go decoding otherwise accepts zero. The PHP fixture retains omitted optional keys, and both wrappers copy and free successful facts independently of their values. The initial Go goroutines did not prove native overlap, and the PHP copied started-failure facts check covered JSON `call` only. The corrected Go witness waits for both held backend arrivals before release, then checks two successful snapshots and typed failure facts after a later call and close; the focused ledger has four exact arrivals. PHP now pins the existing typed malformed-backend facts after recovery and close without changing its 40-request matrix. Source and installed receipts remain distinct; the build record names each actual check and remaining qualification.
