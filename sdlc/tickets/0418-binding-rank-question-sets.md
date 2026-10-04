# 0418: Language rank accepts a question set

Status: ready. Planning only. Depends on reviewed 0401 slice D. Fresh ticket/design review precedes product changes.

Milestone: later

## Outcome

C and each language binding, including supported dataframe variants ranks over the same saved decide question set as the planned CLI/Rust rank_set API. Use the pure turns merge from 0401D, preserve input identities and member name, and report combined facts. A one-member set matches the existing single-question route. A reviewed additive API/carrier retains every existing row shape and plain-text rank behavior.

## Evidence

- Starts from: landed `ff047d8c50ce39ed9ab0acd22695c4960a963d38`, [0405 report](../records/0405-audit-report.md) and planned [0401D](0401-rank-search-flags.md). Current public rank accepts one question; the question-set behavior is planned and must not be described as shipped. Ian’s audit treats cross-surface parity separately from the original Rust-only implementation default.
- Keeps: single-question bytes, stable tie order, documented host position mapping, cache identities, ordinary score/criteria work in 0406, and all existing output types.
- Changes: an additive question-set rank route on the named surfaces, using the already reviewed shared merge rule. Settle public carrier and key-order semantics in a fresh design slice before code.
- Proof: cross-surface comparison to independently declared turns cases, duplicate rows, distinct question scales, ties, top/LIMIT composition, one-member equivalence, strict replay of individual member runs with zero sends, and member naming/facts. Refuse invalid member kinds/cuts/on before sending. Keep host-specific indexes and SQL keys explicitly mapped.
- Defers: new merge rules, score members, filter question sets, provider work and a breaking existing-result migration. Foreign release work is later in 0418; SQL parity is 0.2 in 0417.
