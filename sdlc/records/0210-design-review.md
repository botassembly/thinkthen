# 0210 design review brief

Status: awaiting fresh independent read-only design review at main `a09ceb8f`; no runtime source or tests changed.

Review the ticket and preflight against `AGENTS.md`'s credential rule, `specification/backends.md`'s key and dry-run clauses, register 106, the independent premise review, and current source. Check the exact-byte collision rule, false positives for short keys, timing before any URL output or hash, no change to gateway transport or replay identity, and every CLI/library/Debug entry point. In particular, assess the proposed amendment that dry-run may inspect a configured key solely to prevent disclosure; it must remain a proposed outward choice for Ian. Check the alternative's cost without inventing secret heuristics. Require exact source claims, focused compiled-boundary proof and measured size before implementation. Do not treat design acceptance as code acceptance or close register 106 yet.
