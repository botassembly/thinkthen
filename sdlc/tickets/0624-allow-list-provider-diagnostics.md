# 0624: Pass on only known provider error fields instead of guessing at secrets

Status: OPEN.

Milestone: 0.2.1

Phase: 1

## Outcome

- Provider error diagnostics carry only fields from an allow-list: the status code, the provider error type or code from known response fields, and fixed thinkthen text for each failure cause. Free provider prose is never passed on.
- The deny-word list, the opaque-token check and the related guessing helpers in `diagnostic.rs` are deleted. A key-echo check stays only if an allowed field could still contain the key.
- Error facts, the CLI, the C interface and the bindings all show the same allowed fields.

### Proof

- On the M5 through pm job, the installed `thinkthen` binary runs against the loopback backend's status and malformed arms (`/arm/status/CODE/v1`, `/arm/refuse/v1`, `/arm/malformed/CAUSE/v1`). Provider bodies there contain key text, encoded key text, header lines and long tokens. stderr, the JSON error facts and every `Debug` line contain none of them and show the allowed fields.
- The existing secrecy regressions in `crates/thinkthen/tests/backend/status_reason.rs` pass. Cases that only proved the deny list are deleted.

## Evidence

`crates/thinkthen/src/engine/http/diagnostic.rs:83-128` decides whether a provider's error text is safe to show. It blocks a list of words ("bearer ", "api_key=", "sha256:" and others), strips long tokens that look opaque, and checks for echoes of the key and of the request. Every new provider message format can find a gap. The fixes keep coming: ffa0cb509, 59bbe9cf2, 3a0ff2e2c and 234ef4472, all on 2026-10-13, plus 3d39770af, c4257158c and facc6d280 earlier. Fifteen commits since 2026-09-15 add more hiding rules.
