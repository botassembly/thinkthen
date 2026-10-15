# 0590 — typesafe-jev-images

Status: OPEN.

Milestone: 0.2

## Outcome

`decide`, `choose` and `score` send JPEG and PNG images to the named `typesafe` backend on `jev-1.13.0` and `jev-1.14.0`, and refuse an image outside TypeSafe's published limits with zero sends.

## Evidence

- Ian, 2026-10-15: "I'm not going to release 0.2 without supporting Jev's images."
- TypeSafe blog post "Jev 1.14" (dated 2026-10-14) and docs.typesafe.ai/go/images, both read 2026-10-15. `jev-1.14.0` ships under the alias `jev-preview`; `jev-latest` moves from `jev-1.13.0` to `jev-1.14.0` about a week later; `jev-1.13.0` "was multimodal all along". The docs say Jev 1.13 and 1.14 accept images for every question type, and TypeSafe says images are not ready for production.
- Opt-in: every request carrying an image needs the header `TypeSafe-Danger-Zone-Images: true`, or it fails with 400.
- Wire: a top-level `assets` map lists each image once under a caller-chosen key, as `{"type": "image", "format": "png"|"jpg"|"jpeg", "data_base64": ...}` (`type` optional, `format` lowercase). The server replaces a key it finds as a string value in `state`, `instructions` or `criteria`. An asset referenced nowhere fails with 400. Each key is at most 1,024 bytes, all keys at most 65,536 bytes.
- Limits: 16 MiB per image file; width and height each 256 to 1568 pixels; width times height 78,400 to 2,458,624; request body 32 MiB; expanded request 32 MiB, each reference adding the image again; all images 32 MiB after server processing, which re-encodes and can grow them. Still images only; animated PNG is refused. TypeSafe never resizes.
- Starts from the release branch `ticket/0543-final`, where `release-notes/` lives; main lacks it. Read there: `crates/thinkthen/src/core/adapters/systemone/images.rs` (`ImageRoute`, `named`, `ImageRoute::admit_header`, `ImageRoute::admit`, `ImageWire`, `check`), its `image_tests.rs`, the `typesafe` row in `backends.rs` (model `DEFAULT_MODEL`, `jev-1.13.0`, from `systemone.rs`), `State::images` and `State::body` in `crates/thinkthen/src/core/pack.rs` (which appends a top-level `images` key, not `assets`), and `send` in `crates/thinkthen/src/engine/http/send.rs`, which sets a fixed header list and has no per-request header today. Liquid sends data URLs in a top-level `images` array; Perplexity puts `image_url` parts in `state`. Neither sends an extra header.
- `crates/thinkthen/tests/images.rs` pins `("typesafe", "jev-1.13.0")` as refused; ADR 0121, `specification/backends.md`, `release-notes/0.2.0-known-gaps.md` and `site/src/pages/start/files.astro` all say TypeSafe refuses images.
- 0543 depends on this ticket.

## Decisions

- PM decision, 2026-10-15: add `ImageRoute::TypeSafe`, returned by `named("typesafe", "systemone")`. `admit_header` admits exactly `jev-1.13.0` and `jev-1.14.0`. The moving aliases `jev-latest` and `jev-preview`, `jev-1.13` and other names refuse, so 0.2 behavior never changes when TypeSafe moves an alias. This supersedes the TypeSafe clause of ADR 0121.
- PM decision, 2026-10-15: the request carries `TypeSafe-Danger-Zone-Images: true` only when its body carries `assets`. The adapter decides; the engine only attaches the header. Recordings keep bodies only, so the loopback test reads the header from `conformance_backend`'s recorded request.
- PM decision, 2026-10-15: each asset key is `thinkthen-image-N-H`, where N is the image's 1-based order in the call and H is the first 16 hex digits of its SHA-256. The fixed-length hash keeps user text from matching a key by accident and keeps `image-1` from matching `image-10`. `state` is a JSON array: the text state, left out when it is the empty string as on Perplexity, then each key as a string in image order. Each image appears once in `assets` and once in `state`. Paths never reach the model.
- PM decision, 2026-10-15: ThinkThen never resizes. Admission refuses an image file over 16 MiB, a side outside 256 to 1568, or a pixel count outside 78,400 to 2,458,624. The body limit is 32 MiB. An oversized value uses the existing `ImageLimit::Dimension` sentence. An undersized value needs a new sentence, `image N is below KIND: limit L, actual A`, because "exceeds" would be false. ThinkThen cannot check the post-processing total or animated PNGs; the server's 400 reports those.
- PM decision, 2026-10-15: TypeSafe publishes no image token count, so `image_tokens_known` is false and `tokens_per_question` is 0, as on the local route. Questions have no count limit.
- PM decision, 2026-10-15: build on `ticket/0543-final`; the release notes stay there. Animated PNGs are left to the server's 400.
- PM decision, 2026-10-15: Box answers and the long-context header are out of scope.

## Scope

- Changes: `crates/thinkthen/src/core/adapters/systemone/images.rs`, `crates/thinkthen/src/core/adapters/systemone/image_tests.rs`, `crates/thinkthen/src/core/pack.rs`, `crates/thinkthen/src/engine/http.rs`, `crates/thinkthen/src/engine/http/send.rs`, `crates/thinkthen/src/engine/pipeline/send.rs`, `crates/thinkthen/tests/images.rs`, `crates/thinkthen/tests/images/`, `specification/fixtures/images/typesafe-decide-reply.json`, `specification/fixtures/images/typesafe-choose-reply.json`, `specification/fixtures/images/typesafe-score-reply.json`, `specification/fixtures/images/README.md`, `specification/backends.md`, `sdlc/planning/adr/0121-native-image-input-and-route-admission.md`, `release-notes/0.2.0-known-gaps.md` and `site/src/pages/start/files.astro`. If the header travels as a new `Exchange` field, each test that builds an `Exchange` also changes; the coordinator narrows this claim when the design lands.
- Checklist:
  1. Add the route, its admission and its wire, with unit tests at each published limit and one past it, both directions.
  2. Write `assets` instead of `images` in `State::body` for this route, and attach the header in `send` only for image bodies.
  3. In `tests/images.rs`, drop `typesafe` from the refused list, keep `("typesafe", "jev-1.13")` refused, and add one replayed decide, choose and score case against loopback. Each pins the `assets` map, the `state` array and the header, and a text-only TypeSafe request pins no header and no `assets`.
  4. Update the TypeSafe row and the "Native image route admission" section of `specification/backends.md`, ADR 0121, the known-limits page and the files page: TypeSafe moves to the supported list, and TypeSafe labels its image input not ready for production. Ian re-approves the changed release text.
  5. One live call per function on `jev-1.13.0`, and one `decide` on `jev-1.14.0`, through `sdlc/scripts/live`, by hand, only after Ian authorizes the token cap.
- Keeps: every other backend's image behavior; text-only TypeSafe request bodies and headers byte-for-byte; the "a supported route makes no claim about image accuracy" line.

## Proof

`sdlc/scripts/test` (routine suite): the new replay cases pass, and a refused oversized and a refused undersized image each count 0 loopback requests.

## Defers

Box answers and long context go to a later ticket if a user asks for them. Animated PNG detection waits for a user report; the server refuses it today.
