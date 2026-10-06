# 0449: Keep one configured route per SDK engine

ADR 0119 and sdk-boundary.md define one resolved endpoint, effective key and provider API type per engine. The keep/refuse table preserves explicit model selectors, reading rules, resources and offline tools; model groups, fallback, A/B policy, curation and automatic threshold tuning belong to the proxy. Existing product code already captures the route, so no runtime refactor was needed.

Existing counted-loopback retry, three-stage recognition and split fixtures now assert posting path, fake effective key and literal model throughout. Those tests and direct CLI/Rust backend precedence passed, along with settings, formatting, policy, ratchet and whitespace checks. One fresh review found an accidental prohibition on audit’s caller-requested in-place writes. The corrected contract preserves both explicit write forms; coordinator inspection confirmed it against the existing audit contract. Full tests and lint run on the landing commit before push.

0442–0445 and 0450 still own metadata, storage and proxy reservations. No paid call or second proof framework was added.
