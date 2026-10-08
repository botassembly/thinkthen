# 0463: Document the image cache key domain

Starting source: `60890f8e30aa89a2259144110d365a8bede892e1`.

The cache specification scopes the existing v2 formula to text and gives the separate image tag with the same ordered, length-prefixed parts. Comparison with `core/pack/key.rs::{complete,complete_for,complete_image}`, `core/identity/framing.rs` and `engine/store/versioned.rs::{complete_key,canonical_state}` confirmed both formulas and durable image domain selection. ADR 0121 supplies the image envelope and original byte, media, order and duplicate identity contract. Native code, fixtures and existing v1/v2 rows are unchanged.

The existing ticket checker and its self-test passed. The existing Markdown link checker and its self-test passed; its link parser also checked every relative link in the changed cache specification. `git diff --check` passed. `pm lint` reported zero findings. No provider call, broad build or product test ran for this documentation correction.

Fresh read-only review accepted source `281f586225577cba3851c54f1a4a8e1787d68f15` after comparing both domain tags, framing and ordered parts with native derivation and durable validation. It found no change to v1/v2 row semantics or fixtures.

## What the build taught us

A key contract must name each domain tag as well as its framed parts. Sharing the parts does not make text and image keys interchangeable. Durable validation distinguishes explicit image state by its digest before selecting the tag.
