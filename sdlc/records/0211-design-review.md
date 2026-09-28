# 0211 design review handoff

Status: draft awaiting fresh independent design review; no ACCEPT result or runtime authority is claimed. Candidate base: main `5d15138e`. Review the proposed ticket and [source preflight](0211-tls-preflight.md) against the packaged ureq 3.4.2 API and accepted 0155 proof.

Review questions:

1. Does the named PEM replacement of Mozilla roots, with Mozilla as the unchanged default, give a clear and safe first setting? Is a platform verifier necessary for this outcome despite its feature, dependency and host variability?
2. Is `THINKTHEN_CA_BUNDLE` plus a Rust builder setter enough for CLI, environment-built language libraries and SQL processes, with no SQL `SET` or native per-instance options? Ian must approve this outward-facing reach before runtime work; 0149/0157 cannot silently absorb it.
3. Are absolute path, 2 MiB/256-certificate limits, explicit private-key refusal, local setup errors and replay-only validation acceptable? ureq's PEM conversion alone does not validate every DER trust anchor; should that failure remain a genuine TLS handshake error or justify a direct rustls dependency for early semantic validation?
4. Does the proposed root snapshot preserve fork/model state and avoid reads after a key can be sent? Verify all `facade::Settings` and direct `Client::new` initializers, particularly near-500-line tests, before granting runtime claims.
5. Is the local CA/leaf/wrong-host proof feasible in the pinned test host and sufficient to show real trust decisions, exact fixed diagnostics, zero HTTP requests on failed trust and no secret leakage? Keep 0155's accepted `InvalidData` limitation distinct.

No ADR 0088 text is drafted. The reviewer should return concrete corrections or a design-quality assessment; the coordinator routes any public-setting decision to Ian. Register 118 remains open. No tests, build, provider call or certificate generation ran during this design pass.
