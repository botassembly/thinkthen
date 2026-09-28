# 0250 cache remainder preflight

Notes-only source review from `origin/main` `bfe0791c`; no product, test, marker or public page changed. Register 10's literal criteria are in experiment 284/10. Accepted 0065/ADR 0035 establish the before-send marker; ADR 0099/0228 and 0246 add limited read-only first-use refusals. The earlier [six-row preparation](2026-09-28-cache-replay-remainder-preparation.md) predates 0246 and is not evidence that its explicit-zero case remains open.

## Current sequence and concrete counterexample

`engine/request.rs::ask_prepared` invokes `first_use_key` and then `Recorder::prepare_checked_refresh`. `Recorder::gate` calls `make_folder`, opens `FolderGate` and invokes `identity::check`. `identity::publish_from` writes and syncs a private temporary marker, hard-links its final name and syncs the directory. `prepare_in` may then return a live `WritePermit`. `observe_cancel` checks stop **after** preparation; a stopped live permit cancels its temporary entry but does not remove the marker. `finish_or_cancel` likewise cleans only the entry on a later error. `http.rs::post_marked_with_retry` rejects CR/LF in the key before `acquire_open`, usage admission or `reserve_send`, yet this is after `gate`. A key such as `first\nsecond` is therefore a deterministic zero-attempt source path that can leave a marker in an otherwise new default cache. The existing compiled `backoff` case proves the fixed refusal and zero listener traffic only under `--no-cache`; the cache side effect follows from the inspected call order and is the proposed outside-in regression. No new provider call or compiled reproduction ran in this notes pass.

The post-admission cancellation path is distinct. A process can stop after the durable marker and before the HTTP attempt, or crash at that boundary; no disk/socket transaction can make “zero listener sends implies untouched folder” universally true while binding before any permitted send. `FolderGate` retains an open directory handle and digest locks retain inode names. A naive remove/recreate rollback can let concurrent writers operate in different namespaces. Publishing at response installation would reopen 0065's different-address first-writer paid-send race. An attempted transport can also reach a backend without a local reply, so listener observation is not a sound rollback authority. Existing `engine/request/tests.rs` permit cleanup and `cache_identity.rs` concurrent-first-writer proof are retained; a single controlled after-gate no-send fixture is the smallest further witness during implementation.

## Exact proposed source and validation

| File | Planned bounded change or retained proof |
| --- | --- |
| `crates/thinkthen/src/engine/http.rs` | Give private `Key` one CR/LF validation method; reuse it at the existing HTTP boundary. No general header parser or second error format. |
| `crates/thinkthen/src/engine/request.rs` | In existing `first_use_key`, after the explicit-zero decision, validate a successful early key. On refusal, repeat the read-only `unbound_empty` probe and cancellation check before returning; if a writer appeared, enter the original gate. Keep one key lookup. |
| `crates/thinkthen/src/public/error.rs` | Add a fixed, credential-free recovery instruction to `RecordingBackendMismatch`, retaining Local and nonretryable. |
| `crates/thinkthen/src/engine/request/tests.rs` | Reuse controlled key closure for same/different writer race and one after-gate no-send marker witness. |
| `crates/thinkthen/tests/backend/cache_identity.rs` | Reuse folder and listener helpers for absent/present-empty CR/LF no-touch, later second-address bind, bound hit, legacy/malformed/mismatch precedence. |
| `crates/thinkthen/tests/public_env/cache_budget.rs` | Update the exact safe public mismatch sentence assertion without weakening kind, facts or marker checks. |
| `crates/thinkthen/tests/backend/backoff.rs` | Run unchanged CR/LF zero-send check. |
| `specification/recording.md`, `sdlc/ratchet.json`, 0250 build record | State the narrow no-touch rule and accepted limits; measure counters. The specification is a prospective claim, not edited in this preparation. |

No change is proposed to `recorder.rs`, `recorder/identity.rs`, `core/recording_identity.rs`, `cache_lock.rs`, CLI messages, marker schema, request bytes, digest, replay, output schema, status command or persisted folders. The key-value helper reveals no key through `Debug` or the fixed Usage sentence. Existing bound cache hits and strict replay keep their keyless behavior. Explicit-zero precedence follows 0246; CR/LF remains checked again before every live HTTP attempt. A probe that sees storage damage returns Local, not a misleading Usage. A writer appearing after the second observation is ADR 0099's accepted finite-observation race.

## Review decisions and remaining criteria

Fresh design review should verify the exact precedence when a line-break key coincides with a zero limit, cancellation or a writer's new marker, and whether the fixed public cure is actionable for default and named folders without exposing values. The proposed answer is zero-limit first on still-empty folders; stop before CR/LF on still-empty folders; an appeared writer follows the existing gate when no stop has won at its checkpoint. This makes no new marker or privacy decision, so no ADR 0101 is proposed.

The literal all-zero-send criterion conflicts with durable before-send authority in 0065. The literal bound-address criterion cannot be derived from the version-one SHA-256 marker. The CLI already gives requested address plus cure; the library cure proposed here is fixed text. A raw URL marker/sidecar would expose newly retained address text and break closed-schema old readers, so that is a separate Ian-only privacy/compatibility decision **if** the literal bound-address request is pursued. Options then are to retain safe version-one diagnostics and explicitly narrow that criterion, or design a version-two recoverable address with old-reader migration and access policy. No decision is needed for the proposed deterministic refusal or safe library cure. Register 10 remains open through this ticket and needs a reviewed original-criterion disposition before any status change.

## What this preflight taught us

The prior 0246 build intentionally left CR/LF validation at HTTP because it was outside its explicit-zero slice. Reusing that predicate makes one further pre-admission refusal attainable without replacing the marker protocol. The no-send promise must be stated at a decision point, not inferred backward from a listener count after a filesystem publication.
