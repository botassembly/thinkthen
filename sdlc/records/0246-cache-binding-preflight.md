# 0246 cache binding preflight

Notes-only preparation from `7e8fe1e8`; no product file, saved cache, provider call, or broad test changed. Register 10's original success criteria are in experiment 284/10. The accepted 0228/ADR 0099 exception and the settled 0065/ADR 0035 before-send marker are the current contract.

## Observed boundary

The current compiled public API, using a fresh default cache, a valid key, `http://127.0.0.1:9/v1`, and `CallOptions::send_budget(&budget, Some(0))`, returned `Kind::Usage` and `Some(SendBudgetDenial::BeforeFirstSend)`. The previously absent default cache then existed with `.locks` and `.thinkthen-backend.json`. The ignored source/binary are `target/0246-probe.rs` and `target/0246-probe`; isolated output is `/tmp/thinkthen-0246.4RsTVp`. This is an exact-path and public-error observation, not a measured listener count. The final HTTP reservation refuses before `send`, so the proposed outside-in test must count listener traffic itself.

## Source trace and race boundary

`engine/request.rs::ask_prepared` read-only probes an unbound empty folder, checks stop, and reads the key once. A valid key then enters `Recorder::prepare_checked_refresh`, whose `gate` creates the folder, opens a shared folder lock, and publishes/checks the marker before entry lookup. `engine/http.rs::post_marked_with_retry` later asks `Cancel::reserve_send`; an explicit zero limit denies at that point. `SendBudget::reserve` counts outstanding reservations and refunds uncommitted ones. It would be wrong to classify an arbitrary positive total as finally spent during the early probe. `Some(0)` is a stable configuration fact; no reservation is needed to recognize it. The early denial needs ADR 0099's second read-only folder observation to preserve a writer that bound and installed a hit meanwhile; the original gate still rechecks under lock.

`Cancel::stop()` already sees a pre-fired token, host check, or passed deadline before an empty folder is made; `Recorder::gate` checks it again before `make_folder`. A new early zero-limit denial must check stop again after key lookup to preserve a cancellation that arrived there. A cancellation or expiry after the marker is published can leave it behind without an actual attempt. The HTTP layer also rejects a key containing a line break after preparation; this is a separate known local refusal, not evidence that the zero-limit slice completes all no-send cases. FolderGate retains an open directory inode, and digest lock files retain stable names. Deleting/recreating a folder or marker to hide an unsent result can split the namespace among concurrent callers. A crash between synced marker and socket cannot be rolled back. The version-one reader rejects extra fields, and the hash cannot recover a printable bound URL. No new marker state, sidecar, or cleanup is justified by this slice.

## Smallest review and build table

| Boundary | Expected proof |
| --- | --- |
| Absent default and present-empty unmarked folder, valid key, zero send limit | Existing `BeforeFirstSend` usage fact; listener zero; folder absent or name/bytes unchanged; second address can bind and send once. |
| Existing bound hit or strict replay, zero send limit and no key | Exact saved result, zero listener sends, no new key requirement or marker mutation. |
| Existing mismatch, malformed marker, or unmarked legacy entry | Existing refusal wins before budget denial; no send or changed folder. |
| Writer binds between first and second probe | Same-address hit still replays; different-address winner refuses before this call can send. Use a controlled key-closure barrier in `engine/request/tests.rs`; the CLI key reader has no precise pause point. Count actual listener bytes, not socket arrival as a proxy for admission. |
| After second empty probe, writer binds | This call may return budget denial; later call can see the new entry. This is the accepted finite observation limit, not a lost already-observed hit. |
| Pre-fired cancel/expired deadline; post-admission cancellation | Retain early no-touch cases. Document bound marker as permitted in the late case; do not assert disk/socket atomicity. |
| Positive process total and retry | Keep final `reserve_send`, refund, and actual-attempt accounting; no new early denial or reservation. |

Retain `tests/backend/cache_identity.rs`'s missing-key/second-address, concurrent writer, malformed/legacy, symlink and keyless replay cases, `tests/backend/default_cache.rs::rejected_input_creates_no_default_cache`, and `engine/facade_tests.rs::local_refusals_send_nothing_and_store_nothing`. Run only affected cases, strict lint/format, measured ratchet, pages, tickets and diff. No stress, full suite, or paid call.

## Compatibility and disposition

The CLI already names the requested URL and a cure; `public/error.rs` has a generic mismatch without a cure. A fixed library cure is safe within the existing fixed-text policy but still cannot name the bound URL. `cli/status.rs` reports the requested URL and cache counts, not marker binding; a new comparison field would be a public output change. The original experiment's two-address and universal no-touch success criteria therefore remain unmet. They require an explicit acceptance/reconciliation, not an implied closure from a zero-limit fix. A raw URL marker/sidecar and a publish-at-install move would change privacy or the cross-address race and are not proposed.

Prospective exact product files are `crates/thinkthen/src/engine/{mod,request}.rs`, `crates/thinkthen/src/engine/request/tests.rs`, `crates/thinkthen/tests/backend/cache_identity.rs` or its coherent child, `specification/recording.md`, derived `sdlc/ratchet.json`, and the build record. `crates/thinkthen/src/public/error.rs` is optional and should be a separately reviewed fixed-cure line if the coordinator includes it. No shared marker/core/lock file is needed. Source headroom measured before design: `request.rs` 265, `recorder.rs` 472, `recorder/identity.rs` 246, `cache_identity.rs` 433 nonblank lines. The recorder path should remain unchanged.

## What the preparation taught us

The usable distinction is whether refusal is certain **before** first write-capable admission. `Some(0)` is certain; a positive budget observed during concurrent reservations is not. Existing cancellation/deadline checkpoints already cover calls stopped before admission. A failed call after admission can leave a valid marker by design, even with no listener bytes. Treating every zero-send case as rollback work would undermine durable identity and older-reader compatibility.
