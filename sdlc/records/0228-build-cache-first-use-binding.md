# 0228 build: missing key leaves an unbound empty cache

Status: complete at reviewed source `10ae3238`. The build used an independent `ticket/0228-cache-first-use-binding` lane after main `47a68ed0` merged at `0e18af77`. Ian approved the narrow before-key exception in the work plan. Fresh Medium preparation review accepted `31fc345a`; technical design review accepted `be0a2336`. No provider, stress, churn, full test, spec or surfaces run belongs to this candidate.

## Change and retained behavior

The recorder observes whether a write-capable folder is absent or present, empty and unmarked without creating it or setting its checked flag. It checks the private default's mode first. A present marker, including a malformed one, and a digest-shaped legacy entry still take the original gate first. A dangling symlink is a storage failure. On an unbound empty first use, request admission checks cancellation and reads the key once. A missing key triggers a second read-only observation. If the folder remains unbound and empty, the original key error returns without folder artifacts. If another writer appeared, the ordinary gate and entry lookup can return its matching answer without the key. A successful early key proceeds through the existing gate before sending. The gate retains identity authority and the marker-before-send concurrency rule. Strict replay and existing cache hits retain keyless behavior. The marker, entry digest and public errors are unchanged.

ADR 0099 is accepted, and ADR 0035, settled ticket 0065 and the recording specification state the timing exception. Register 10 remains open: cancellation after preparation, an admitted request that sends nothing, status and unbind guidance remain separate.

## Inherited compilation prerequisite

The merged backend test target initially failed because `tests/backend/public_json.rs` invoked `to_json`, `iter` and `len` directly on the newer `Call<T>` result. The four uses now borrow `Call::value()` and retain the same byte comparisons and edge count. An unused `Sealed` import in `public/bulk/details.rs` stopped strict lint; it was removed. These two exact paths were granted to this lane by the coordinator after the failure. The correction adds no cache behavior. Its focused public JSON comparison passed.

## Proof and limits

- `cargo test --locked --test backend cache_identity::`: 11 passed. The new table covers absent default, absent named and present empty named folders, exact no-key exit and diagnostic, folder presence or unchanged names, then one accepted request and marker per second address. The dangling-path case pins storage precedence. Existing mismatch, concurrent writers, legacy and replay cases passed; the legacy and malformed cases now run without a key.
- `cargo test --locked --test backend public_json::`: 1 passed. Strict workspace Clippy with all targets and features passed after the final dangling-path addition.
- Format, ratchet, pages, tickets and diff checks passed. No full gate or paid call ran. The second-address loopback listener cannot observe the first nonloopback address. Its first-run no-send conclusion follows the exact missing-key failure path.

The Rust source/test ceiling rose from 89,801 to 89,949 nonblank lines. The extra lines hold the admission observation, one-use key path and compiled-boundary edge cases. I checked the existing `Recorder::gate`, `identity::has_entry`, `request::finish_or_cancel` and cache identity cases for reuse before adding code. The new recorder and identity helpers reuse those gates and checks; no second binding protocol or copied concurrency test was added. No test was deleted or consolidated; retained mismatch, replay and legacy cases continue to prove distinct behavior.
