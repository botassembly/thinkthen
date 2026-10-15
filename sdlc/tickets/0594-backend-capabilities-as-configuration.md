# 0594 — backend-capabilities-as-configuration

Status: OPEN.

Milestone: 0.3

## Outcome

Every per-backend and per-model fact ThinkThen enforces, such as image support and limits, context length and request limits, comes from one backend configuration. ThinkThen ships the current values as built-in defaults, and a user can declare or override them for any backend or model without a new ThinkThen release.

## Evidence

- Ian, 2026-10-15: images, context window length "and really anything else that's kind of hard-coded into all these different models should be decoupled and turned into a configuration per backend."
- In 0.2, `crates/thinkthen/src/core/adapters/systemone/images.rs` names the image models in code (`ImageRoute::admit_header`: Liquid `d1`, Perplexity `pplx-decider-v1-27b`, TypeSafe `jev-1.13.0` and `jev-1.14.0`) and writes each route's pixel, byte, aspect-ratio and token arithmetic in code. `images/local.rs` admits only three named tested local setups. A new image model, such as a later Jev, needs a ThinkThen release.
- `crates/thinkthen/src/core/adapters/systemone/backends.rs` names each backend's path, key variables and default model in code. `crates/thinkthen/src/core/backend_profile.rs` already reads a profile with evidence-byte, request-byte, question and option limits and a closed `image_profile`; it is the starting point.
- Image formats are JPEG and PNG only (`core/image.rs`, `ImageMedia`).
- ThinkThen keeps one route per engine; business routing stays in the proxy.

## Decisions

- PM decision, 2026-10-15: the configuration describes what a backend and model accept, never which backend to choose. Fallback, routing and A/B policy stay out.
- PM decision, 2026-10-15: ThinkThen still enforces every limit before sending, whether built in or declared. A declared value the wire route cannot honor is refused at load with a plain sentence.
- PM decision, 2026-10-15: the image wire shape (Liquid, Perplexity, TypeSafe assets, local) stays a code-defined route. Configuration picks a route and supplies the models, formats and limits for it.

## Scope

- Changes: `crates/thinkthen/src/core/backend_profile.rs`, `crates/thinkthen/src/core/adapters/systemone/images.rs`, `crates/thinkthen/src/core/adapters/systemone/images/local.rs`, `crates/thinkthen/src/core/adapters/systemone/backends.rs`, `crates/thinkthen/src/config/`, `specification/backends.md`, `specification/settings.md` and the built-in defaults file the design adds. The coordinator narrows this when the design lands.
- Checklist:
  1. Inventory every per-backend and per-model fact in code: image models, formats, limits and token arithmetic; context and token limits; request, question and option limits; default models; key variables; probability and token-count support. Name each one's source.
  2. Move each into one configuration schema with built-in defaults equal to today's values.
  3. Let a user declare a new image model on an existing route, with its limits, and override any limit for a backend or model.
  4. Delete the in-code tables the configuration replaces.
- Keeps: every 0.2 request byte for byte under the built-in defaults; every refusal sentence; zero sends on a refused input.

## Proof

The routine suite passes unchanged under the defaults, and a replayed case declares an image model ThinkThen does not ship, sends one image within its declared limits, and refuses one past them with zero sends.

## Defers

New wire routes, such as OpenRouter's image format, and new image formats, such as WebP, are separate tickets that use this configuration.
