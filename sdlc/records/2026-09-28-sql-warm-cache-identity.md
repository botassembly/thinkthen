# SQL warm follows exact request cache identity

Status: coordinator clarification under existing ADR 0048 and B0, after the 0219 builder reproduced a stale compatibility assumption. This introduces no cache projection or new request format. Ian can overturn the interpretation. Independent code review must check the corrected ticket, both cache-use proofs and honest documentation before B13e closes.

## Evidence and governing rule

SQLite 0219 WIP `601e4ad0` produces the old slide's correct answers but six listener sends: one packed warm request and five later singleton requests. The old test expected five sends, all during warm. The builder separately verified that a repeated identical packed warm hits the cache and that shared context does not make packed and singleton digests identical. Its build record pins source, artifact hash, exact requests and full digests. This is a deterministic request-identity difference, not test flakiness or machine load.

ADR 0048 item 6 keys cache entries by the batch's exact request digest; item 1 preserves old single-record bytes at batch one without context. B0 explicitly retains the need to choose batch one for old recordings/caches and allows an answer to change with its batch neighbours. The 0219 sentence promising cached scalar reads after any ordered warm is therefore too broad. The folder is shared; the request key is not. Manufacturing singleton keys for packed answers would misstate the request and provenance.

## Implementation and proof

Keep default Max packing and the exact-request cache. The legacy warm-then-scalar recipe selects `thinkthen_batch(1)` or the existing batch-one environment setting before engine creation. Its functional proof still requires no sends after warm; do not weaken that assertion. Separately prove default packed warm's bounded send count and reuse by an identical complete cohort. A later scalar request is a different request and may send; document that limit. Shared context changes request identity too. No hidden fallback to batch one, row-answer cache, duplicate paid warm pass or new batch-handle API belongs to 0219.

Update the ticket's retained behavior, README and library-owned slide/test together. Historical exact-body fixtures may select batch one locally. The user-facing recipe must expose that choice so a test-only environment setting cannot hide a misleading example. Route marketing-owned SQL samples through the existing site issue. Carry the same clarification into PostgreSQL 0217 preparation and DuckDB B13c documentation while preserving each host's actual request grouping. Ordinary single-row throughput and register 73 remain open. A new API to consume a packed warm's per-row answers would be separate reviewed work, not necessary to preserve cache honesty.
