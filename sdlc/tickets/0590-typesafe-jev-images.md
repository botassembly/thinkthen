# 0590 — typesafe-jev-images

Status: OPEN.

Milestone: 0.2

## Outcome

`decide`, `choose` and `score` send JPEG and PNG images to the named `typesafe` backend on `jev-1.13.0` and `jev-1.14.0`, and refuse an image outside TypeSafe's published limits before sending.

## Evidence

- Ian, 2026-10-15: "I'm not going to release 0.2 without supporting Jev's images."
- TypeSafe released image input on 2026-10-14 (blog post "Jev 1.14", docs page docs.typesafe.ai/go/images, read 2026-10-15). It is opt-in through the header `TypeSafe-Danger-Zone-Images: true`. A top-level `assets` map holds each image as `{"type": "image", "format": "png"|"jpg"|"jpeg", "data_base64": ...}` under a key the caller picks, and the request references that key as plain text in `state`. Every asset must be referenced, or the request fails with 400. Jev 1.13 and 1.14 accept images for every question type. TypeSafe labels the feature not ready for production.
- Limits from that page: 16 MiB per image; width and height each 256 to 1568 pixels; 78,400 to 2,458,624 total pixels; 32 MiB request body; 32 MiB for all images after processing.
- Starts from: `crates/thinkthen/src/core/adapters/systemone/images.rs` (`ImageRoute`, `named`, `admit_header`, `ImageWire`), its `image_tests.rs`, and the `typesafe` entry in `crates/thinkthen/src/core/adapters/systemone/backends.rs`. Liquid and Perplexity already use this route shape.
- 0543 depends on this ticket.

## Decisions

- PM decision, 2026-10-15: add one `ImageRoute::TypeSafe` for the named `typesafe` backend on the `systemone` path, admitting only `jev-1.13.0` and `jev-1.14.0` and their aliases that TypeSafe documents (`jev-latest`, `jev-preview`). ThinkThen sends the opt-in header only on requests that carry an image.
- PM decision, 2026-10-15: ThinkThen never resizes. An image outside the limits above is refused before sending, with the existing `ImageLimit::Dimension` sentence naming the image and limit.
- PM decision, 2026-10-15: the asset key is derived from the image's ordinal in the call, such as `image-1`. Image paths never reach the model, as the known-limits page already promises.
- PM decision, 2026-10-15: the Box answer type and long context are out of scope.

## Scope

- Changes: `crates/thinkthen/src/core/adapters/systemone/images.rs`, `crates/thinkthen/src/core/adapters/systemone/image_tests.rs`, `crates/thinkthen/src/core/adapters/systemone/backends.rs`, the replay fixtures for the new route, `release-notes/0.2.0.md`, `release-notes/0.2.0-known-gaps.md`, and the image rows in `specification/`.
- Checklist:
  1. Add the route, its admission and its wire fields, with boundary tests at each published limit.
  2. Add one replayed decide, choose and score case each against a loopback backend, pinning the header, the `assets` map and the state reference, and pinning that a text-only request carries no header and no `assets`.
  3. Move TypeSafe from the refusing list to the supported list in the known-limits page, and say TypeSafe labels its image input experimental. Ian re-approves the changed release text.
  4. One live call per function on `jev-1.13.0` through `sdlc/scripts/live`, by hand, only after Ian authorizes the token cap.
- Keeps: every other backend's image behavior; text-only TypeSafe requests byte-for-byte; the "a supported route makes no claim about image accuracy" line.

## Proof

The routine suite's new replay cases pass, and a refused oversized or undersized image counts 0 loopback requests.

## Defers

Box answers and long context go to a later ticket if a user asks for them.
