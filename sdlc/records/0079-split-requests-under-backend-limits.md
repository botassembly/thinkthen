# 0079: Split requests under backend limits

Status: Accepted after independent review. Integrated verification and landing remain.

## Result

The private engine now splits an ordered multi-question plan into the fewest contiguous chunks that satisfy an explicit backend profile's exact request-byte and expanded-question limits. It prepares every chunk before replay, cache access, key access, or the first send. A plan that fits keeps its historical body and digest.

Backend profiles may now set `max_options` for one choice question. An exact limit passes. A choice over the limit fails locally because dividing its options would change the question. Split results preserve logical answer order, per-chunk request identities, checked usage, actual sends including retries, model consistency, and all-chunks cache truth.

## Review and verification

Independent design review corrected the ADR ownership, stale option and size claims, aggregation and accounting rules, dependencies, source-file split, and complexity floor. Independent code review rejected stale whole-group cache wording and missing exact-boundary option proof. Remediation documented chunk-level cache identity and added the positive boundary test. The same reviewer accepted the final diff.

The implementer observed the focused tests fail before the splitter existed. The final focused splitting suite passes six tests. Annotate, profile, usage, interruption, formatting, Clippy, policy, exact ratchet, and whitespace checks pass. The implementation adds 331 nonblank production Rust lines and 675 nonblank Rust lines including tests, within the accepted budgets. Every Rust file remains below 500 nonblank lines. The exact crate ceiling is `38070/38070`. ADR 0032 remains byte-identical.

The coordinator's current-main integrated ladder remains before landing. No live, paid, or external request ran.
