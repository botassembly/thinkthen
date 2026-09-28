---
flow: build
priority: 250
opens: crates/thinkthen/src/engine/http.rs crates/thinkthen/src/engine/request.rs crates/thinkthen/src/public/error.rs crates/thinkthen/src/engine/request/tests.rs crates/thinkthen/tests/backend/cache_identity.rs crates/thinkthen/tests/public_env/cache_budget.rs specification/recording.md sdlc/ratchet.json sdlc/records sdlc/tickets
---

# 0250: Refuse a line-break key before first cache binding

Status: design candidate for fresh independent review. No runtime claim or issue closure yet. This is a bounded remainder of [register 10](/home/ian/workspace/experiments/284-issue-register/10-d5-default-cache-binds-address.md) after accepted [0228](0228-cache-first-use-binding.md) and [0246](0246-cache-binding-before-send.md). The [preflight](../records/0250-cache-remainder-preflight.md) records the current source trace. ADR 0035, ADR 0099, ticket 0065 and the version-one marker remain authoritative; no new ADR is needed for moving this existing deterministic refusal earlier.

## Outcome and exact limit

An unbound empty write-capable cache or recording folder receives one early key lookup, as it does today. If that key contains CR or LF and this call has no explicit zero send limit, the existing fixed Usage refusal happens before writable admission while the folder remains unbound and empty. An absent folder stays absent; an existing empty one keeps its names and bytes. A later request at another address can bind and send. Do not repeat, log or persist key bytes. Add a fixed safe recovery instruction to the public library's backend-mismatch message. It should say: `the recording folder belongs to another backend address; restore its backend settings or choose another folder`. Its error kind, retry signal and facts stay unchanged. The command's already accepted requested-URL and cure sentences stay unchanged.

This catches exactly the HTTP layer's existing CR/LF refusal, not every malformed header value or zero-send outcome. A successful key lookup alone does not establish an admissible HTTP key. With an explicit zero send limit, retain 0246's `BeforeFirstSend` precedence on an unbound empty folder even if the key also has a line break. Existing bound hits and strict replay do not require a usable key. A bound mismatch, malformed marker and unmarked legacy entry still refuse through the folder gate before inspecting key content.

The original register also requires every zero-send run to leave the default folder untouched and every mismatch to name both the bound and requested addresses plus a cure. This ticket does **not** meet those literal criteria. A durable marker may precede a cancelled, expired, crashed or ambiguous transport path that never reaches a listener. The marker holds only a hash; no reader can recover the bound address from it. Keep register 10 open and seek a separate reviewed disposition of those conflicts; do not infer closure from this narrow repair.

## Evidence

- Starts from: `engine/request.rs::first_use_key` already reads one key before an unbound empty folder's gate; `http.rs::post_marked_with_retry` rejects CR/LF only after that gate has published a marker. The retained `backoff` test proves the fixed refusal and zero sends under `--no-cache`.
- Keeps: ticket 0065's before-send marker, ADR 0099's read-only probe and writer race, 0246's explicit-zero precedence, keyless hits/replay, mismatch/legacy/storage precedence, one key lookup and final HTTP send reservation.
- Changes: reuse the CR/LF predicate before first writable admission when the folder remains unbound and empty; add one fixed public-library cure sentence without any address, key, path or marker content.
- Proof: compiled absent/present-empty listener and folder snapshots, later second-address bind, bound hit/replay, controlled racing writer, zero-limit/cancel precedence and exact safe public message; retain the existing `--no-cache` refusal.
- Defers: every post-admission no-send case, raw bound-address recovery, marker migration, status redesign and any claim that this ticket closes register 10.

`engine/request.rs::first_use_key` performs ADR 0099's read-only `unbound_empty` observation, cancellation check and one key lookup. It now recognizes 0246's immutable `Some(0)` before entering `Recorder::prepare_checked_refresh`. `Recorder::gate` then creates the folder, obtains the shared folder gate and calls `recorder/identity.rs::check`, which publishes and syncs the marker before entry lookup or a possible send. `engine/http.rs::post_marked_with_retry` currently rejects a key containing CR or LF at its start, after that marker was published. The existing `tests/backend/backoff.rs::a_line_break_in_the_key_fails_locally_without_a_send` pins the Usage sentence and zero transport for `--no-cache`, but says nothing about the default folder.

Ticket 0065 deliberately made `Recorder::gate` the only binding authority before any permitted send. It also made the version-one marker a closed, bounded hash of adapter and canonical URL. 0228 and 0246 preserved a read-only first-use observation followed by the authoritative gate. This ticket reuses that seam. `public/error.rs` currently gives a fixed mismatch sentence without a cure; `cli/failure/recording.rs` already supplies safe requested-address and cure wording. The public message addition uses no URL, path, key, marker byte or evidence.

## Design

Extract the existing CR/LF predicate and fixed `Error::Usage("the API key contains a line break")` into one private `Key` validation method in `engine/http.rs`. Keep its invocation at the start of `post_marked_with_retry`, so the final HTTP boundary still checks every live attempt. In `first_use_key`, after the one early key lookup and the existing explicit-zero branch, call that same method. If it refuses, re-observe `unbound_empty` read-only. If the folder remains unbound and empty, check `cancel.stop()` again and return the same Usage before `prepare_checked_refresh`. A storage error remains a storage error. If a cooperating writer has published a marker or a legacy entry, retain the key and enter the original gate/lookup path: mismatch, malformed or legacy refuses there; a matching cache hit can answer; a live miss reaches the unchanged HTTP check and refuses after admission. This is the same finite observation limit already accepted in ADR 0099 and 0246. Do not add a third probe, second key lookup, new state file, early send reservation or marker cleanup.

Change only the fixed public mismatch text to add the recovery instruction. The library caller can select a different cache or recording folder via its existing settings. Keep the command's URL rendering and both existing mismatch sentences untouched. Do not copy a bound URL into the marker, entry, sidecar, public error or status output.

## Smallest outside-in proof

| Boundary | Required observation |
| --- | --- |
| Absent default and present-empty unmarked named folder; key has CR or LF | Same fixed Usage/exit 2 and zero listener requests as the retained `--no-cache` case; no new folder, marker, lock or temporary entry; existing empty folder's names and bytes unchanged; a later valid-key request at another address binds and sends once. Include both CR and LF without printing either key. |
| Bound matching cache hit and strict replay with a line-break or missing key | Existing answer, no HTTP attempt or new key requirement; marker and entry unchanged. |
| Bound mismatch, malformed marker, legacy unmarked entry | Original gate-first Local refusal and unchanged folder before key validation. Public mismatch carries the fixed cure, remains nonretryable Local, and reveals no bound URL, key or path. Command text stays exact. |
| Writer binds between early key lookup and second read-only probe | A same-address winner may provide a keyless hit; a different-address winner refuses before this caller sends. Use the existing controlled key-closure seam in `engine/request/tests.rs`, not socket arrival as an admission marker. |
| Explicit zero limit plus line-break key; cancellation during key lookup | Retain 0246's zero-limit precedence when still unbound and empty; a stop after key lookup wins over the new Usage if the folder is still empty. Final `reserve_send` remains the only actual-send authority. |
| Controlled post-admission stop | A valid first-use marker may remain with zero actual sends. Pin this as a permitted counterexample to the universal no-touch claim, not as a cleanup target. |

Reuse `tests/backend/cache_identity.rs`'s folder snapshot/second-address and listener helpers, `tests/backend/backoff.rs`'s original CR/LF refusal, `tests/public_env/cache_budget.rs`'s public mismatch assertion and `engine/request/tests.rs`'s controlled writer. Run only those affected cases plus format, strict affected lint, measured counters, pages, tickets and diff. No full matrix, provider, stress or migration run.

## Separate register-10 disposition

The attainable compatibility-preserving correction is deterministic **pre-admission** refusal. Universal no-touch is incompatible with the accepted before-send marker when a process can stop or crash between synced disk publication and socket transmission. Publishing only at entry installation would let different-address first writers send before one marker wins; removing a marker on no-send can split directory and lock inode ownership among live users. Do not weaken 0065 to obtain a sentence that cannot hold atomically across a filesystem and a network.

The hash-only marker also cannot supply the bound URL requested by the original diagnostic. The fixed public cure completes the actionable part on libraries, while the command already names the requested URL and cure. A read-only comparison against a caller-supplied candidate address could answer whether that **candidate** matches, but cannot reconstruct the bound URL and would change public status output; it is optional redesign, not this ticket. A raw-address version-two marker or sidecar would newly persist an address, expose it to folder readers, and cause older closed-schema readers to refuse. It needs a separately reviewed privacy and migration choice if Ian requires the literal two-address wording. No such choice is needed to implement this ticket. The root should retain the register row until a fresh reviewer checks and the coordinator records the precise original-versus-accepted-contract disposition.

## What the preparation taught us

The after-admission marker is not an accidental leftover: it is the durable protection against the cross-address paid-call mistake that opened 0065. The remaining avoidable refusal is narrower than “bad key”: only the existing CR/LF check is certain and safe to reuse before first admission. The same second read-only observation that protects 0228 and 0246 also protects a racing writer here. The public library lacks a cure, but a safe fixed cure does not require recovering or storing the bound URL.
