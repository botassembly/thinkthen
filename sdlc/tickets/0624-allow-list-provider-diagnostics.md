# 0624: Pass on only known provider error fields instead of guessing at secrets

Status: COMPLETE.

Milestone: 0.2.1

Phase: 1

Reviews: revision 09c9fa4, accept

## Outcome

- Changes: `conformance/backend/src/arms.rs`, `conformance/children/provider_diagnostics.py`, `crates/thinkthen/src/engine/error/provider.rs`, `crates/thinkthen/src/engine/http/diagnostic.rs`, `crates/thinkthen/src/engine/http/send.rs`, `crates/thinkthen/src/engine/http/tests/ollama.rs`, `crates/thinkthen/tests/backend/status_reason.rs`, `crates/thinkthen/tests/native_attempts.rs`, `specification/backends.md`.
- Provider error diagnostics carry only fields from an allow-list: the status code, the provider error type or code from known response fields, and fixed thinkthen text for each failure cause. Free provider prose is never passed on.
- The deny-word list, the opaque-token check and the related guessing helpers in `diagnostic.rs` are deleted. A key-echo check stays only if an allowed field could still contain the key.
- Error facts, the CLI, the C interface and the bindings all show the same allowed fields.

### Proof

- On the M5 through pm job, the installed `thinkthen` binary runs against the loopback backend's status and malformed arms (`/arm/status/CODE/v1`, `/arm/refuse/v1`, `/arm/malformed/CAUSE/v1`). Provider bodies there contain key text, encoded key text, header lines and long tokens. stderr, the JSON error facts and every `Debug` line contain none of them and show the allowed fields.
- The existing secrecy regressions in `crates/thinkthen/tests/backend/status_reason.rs` pass. Cases that only proved the deny list are deleted.

## Evidence

`crates/thinkthen/src/engine/http/diagnostic.rs:83-128` decides whether a provider's error text is safe to show. It blocks a list of words ("bearer ", "api_key=", "sha256:" and others), strips long tokens that look opaque, and checks for echoes of the key and of the request. Every new provider message format can find a gap. The fixes keep coming: ffa0cb509, 59bbe9cf2, 3a0ff2e2c and 234ef4472, all on 2026-10-13, plus 3d39770af, c4257158c and facc6d280 earlier. Fifteen commits since 2026-09-15 add more hiding rules.

## Progress

- 2026-10-18 started
- 2026-10-19 next: The combined0624/0631 gate passed Rust and consumer runs but stopped on the old bare-Rust campaign fixture in gate log34ecb520eb78bfe96f01c4abe70a8d68be9dd8d3; gate the accepted0614/0643 cleanup pair now, investigate the fixture against that cleanup, then rerun the accepted diagnostics and installed-consumer pair.
