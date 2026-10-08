# 0463: Document the image cache key domain

Status: OPEN. The after-sprint review confirmed that the v2 cache formula omits the implemented image domain.

Milestone: 0.2

Owner: builder.
Severity: medium documentation inconsistency.

Reviews: revision e71fa0b01, accept

Reviews: revision 281f586225577cba3851c54f1a4a8e1787d68f15, accept

## Outcome

The public v2 key formula reproduces both text and image keys offline. Existing image rows and native key validation remain unchanged.

## Evidence

- Starts from: 0462 core/engine review at 7ea661c1e; specification/cache.md describes only thinkthen.question-key/2, while core/pack/key.rs::complete_image and engine/store/versioned.rs use thinkthen.image-question-key/2. ADR 0121 already separates the original image domain.
- Keeps: Existing v1/v2 rows, image byte/media/order identity, secrecy and native derivation. No cache-key code or migration changes.
- Changes: State the separate v2 image tag and identical length-prefixed parts in specification/cache.md, and clarify the text formula's scope.
- Proof: Fresh ticket and whole-change reviews; compare the documented tags/parts against the native derivation and durable validator. Run existing ticket/whitespace checks. No calls or broad builds for a documentation correction.
- Defers: No new cache format, migration, release workflow or proof tooling.
