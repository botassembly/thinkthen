# 0594 — backend-capabilities-as-configuration

Status: OPEN.

Milestone: 0.3

## Outcome

ThinkThen describes every backend and every model as data, in one catalog with one schema. The ten functions stay the same while backends change underneath them. A built-in catalog ships today's backends and models with today's values. A user adds a backend with a name, an address, an API type, a key variable and a model, and adds or adjusts a model on any backend, built-in ones included, without a ThinkThen release. ThinkThen enforces every limit the catalog states before it sends anything.

## Evidence

- Ian, 2026-10-15: images, context window length "and really anything else that's kind of hard-coded into all these different models should be decoupled and turned into a configuration per backend."
- Ian, 2026-10-15, after reading the first draft of this ticket: think through "the full graph of meaning: the nodes and edges, the type of graph, and how to configure providers, models, capabilities, attributes, and things like context window, endpoints, options." The functions stay the same and backends swap in and out. Adding a backend should take little more than a URL, a path, a name, a key and its environment variable.
- Today the facts sit in five places:
  1. `crates/thinkthen/src/core/adapters/systemone/backends.rs` and the OpenAI adapter: each built-in's name, base, path, API type, key variables, default model, description form and input accounting.
  2. `core/adapters/systemone/images.rs`: the admitted image models; Liquid's rule (32-pixel patches, 1.5 tokens each, aspect ratio at most 100, 10,000 patches per request, 4,499,999-byte body); Perplexity's rule (32-pixel tiles rounded to nearest, at least one, 2,048 per image, 128 questions, 32 MiB body).
  3. `images/local.rs`: three measured local setups, chosen by ID in a version-two `--profile`; two images of at most 1 MiB and 1,024 pixels on the longest edge, a 2,800,000-byte body, image tokens unknown.
  4. Ticket 0590, on the 0.2 release branch: TypeSafe's `assets` route for exactly `jev-1.13.0` and `jev-1.14.0`; lower bounds as well as upper ones (each side 256 to 1,568 pixels, 78,400 to 2,458,624 pixels); 16 MiB per image; a 32 MiB body; the header `TypeSafe-Danger-Zone-Images: true` on image requests only; the moving aliases `jev-latest` and `jev-preview` refused.
  5. Prose only, in the provider table of `specification/backends.md`: Perplexity's 255 options, 10 score levels and 262,144 input tokens; Jev's 32,000-token context on OpenRouter; Clef's 65,536-token context; MLX's 2 to 10 score levels; TypeSafe's 65,536-token refusal; OpenAI's 128 images per request. ThinkThen enforces none of these as written, and the table is kept by hand.
- The configuration file already holds `backends` entries with `url`, `path`, `key_env`, `model`, rate, prices, `both_sides` and an inline profile (ADR 0114, ADR 0117). Configured entries always speak System One. Built-in entries take only rate, prices and profile. `--profile` is a second file with byte, question and option limits.
- One model runs under different names and limits at different addresses. Jev is `jev-1.13.0` at TypeSafe and `typesafe/jev-1.13` at OpenRouter, where its context is 32,000 tokens. Clef runs routed through OpenRouter, directly on Cloudflare and locally on llama.cpp. Neither the model name alone nor the address alone can key these facts.
- Some limits protect ThinkThen itself and describe no backend: 8 images and 24 MiB per image in the decoder, 8,192-pixel sides, 1 MiB replies, 400 pair questions per relate request.

## The model

The catalog is a small directed graph with three kinds of node and two kinds of edge. Names are the references. One JSON document holds it, and ThinkThen checks every reference when it loads.

- **API type** (node, defined in code): a wire shape ThinkThen speaks, `systemone` and `openai-decisions` today. Each owns its request encoding, answer decoding, image wire forms and key header. The catalog names one and cannot define one.
- **Backend** (node): one address that speaks one API type. It holds the name, base, path, API type, key variables, whether a loopback address needs a key, request rate, description form, input accounting and limits that hold for every model there. The provider is a label on the backend, not a node, because ThinkThen behaves by address and not by company.
- **Model** (node): facts that hold wherever the model runs: whether it reads images, its image token rule, its context length, and the option and score-level limits the model itself sets.
- **Offering** (edge, backend serves model): the model ID that backend expects, plus the facts that differ at that address, such as OpenRouter's 32,000-token Jev context, prices, or the TypeSafe image header. A backend's default model is one of its offerings.
- **Like** (edge, model or offering starts from a model): the new entry takes another model's facts and states only what differs. `"jev-1.15.0": {"like": "jev-1.14.0"}` on `typesafe` is the whole declaration for a later Jev.

Offering edges are many-to-many between backends and models; `like` edges form chains and may not loop. The graph is a few dozen nodes, so a validated JSON map is enough and a graph store would add nothing.

A run picks one backend and one model name, as it does today. ThinkThen finds the offering whose ID equals that name exactly. Each fact comes from the most specific place that states it: the run's profile, which may only lower a limit, then the offering, then the model and its `like` chain, then the backend, then the API type. A model name with no offering still works for text under the backend's limits. It gets no images and no model limits, because ThinkThen never guesses a capability.

The facts form a closed vocabulary, and each key names its unit:

- Transport: base, path, API type, key variables in reading order, key optional on loopback, requests per minute, and headers sent only with images.
- Encoding: description form (as authored, as text, both sides) and input accounting.
- Request limits: request bytes, evidence bytes, questions per request, options per choice, score levels, context tokens.
- Images: formats; images per question and per request; bytes per image; least and most pixels per side; longest edge; least and most total pixels; aspect ratio; body bytes with images; and an image token rule picked from methods defined in code with numbers from the catalog (patches, tiles, or unknown).
- Prices: input and output US dollars per million tokens.
- Provenance, required in the built-in catalog: the source link or experiment and the date checked.

## Decisions

- PM decision, 2026-10-15: the catalog describes what a backend and model accept, never which backend to choose. Fallback, model groups, routing and A/B policy stay with the proxy. In 0.5 the proxy reads this same catalog for what each backend accepts and keeps its routing in its own settings.
- PM decision, 2026-10-15: ThinkThen enforces every limit before sending, whether built in or declared. A declared value the API type cannot honor, such as image facts on an API type with no image wire, is refused at load with a plain sentence that repeats no value.
- PM decision, 2026-10-15: each image wire shape (Liquid `images`, Perplexity parts in `state`, TypeSafe `assets`, OpenAI parts) stays code owned by its API type. The catalog picks a shape and supplies the models, formats and limits for it.
- 0.3 PM decision, 2026-10-15: the built-in catalog is one data file in the repo, compiled into the binary. The named-backend and provider tables in `specification/backends.md` are generated from it and checked by `spec`, so code, documentation and enforcement cannot drift.
- 0.3 PM decision, 2026-10-15: the user's catalog lives in the existing configuration file: a top-level `models` map and a `models` map inside each `backends` entry. The schema becomes `thinkthen.config/2`, with no compatibility for version 1 before 1.0. The file still never holds a key and must still be writable only by its owner when it holds backends.
- 0.3 PM decision, 2026-10-15: adding a backend takes `url`, `key_env` and `model`, with `path` and `api` optional (`systemone` by default). Configured backends may now name `openai-decisions`. A built-in backend's transport stays fixed; a user may add offerings to it and override its limits, rate and prices.
- 0.3 PM decision, 2026-10-15: capabilities attach to exact model IDs. A moving alias such as `jev-latest` gains no capability unless the user declares it, so a provider moving an alias never changes what ThinkThen admits.
- 0.3 PM decision, 2026-10-15: headers sent with images are header names and literal values only. They never carry `Authorization`, a key, or a value read from the environment.
- 0.3 PM decision, 2026-10-15: `--profile` stays as a per-run overlay that uses the catalog's limit keys and may only lower a value; a higher value is refused at load. The version-two `image_profile` and the three local setup IDs go. The three measured setups become built-in models, and a user names one with `like` on the `llamacpp` backend under the alias their server uses.
- 0.3 PM decision, 2026-10-15: `thinkthen help backends` lists every backend and model with its resolved facts and where each came from: built in, configuration or profile. A person or an agent can see what a declaration did. No key value appears.
- 0.3 PM decision, 2026-10-15: the catalog changes what ThinkThen admits and never a question's cache or recording identity.
- 0.3 PM decision, 2026-10-15: ThinkThen's own safety limits and the limits that belong to a function's meaning stay in code. The catalog may lower a safety limit and never raise one.
- 0.3 PM decision, 2026-10-15: ThinkThen fetches nothing to fill the catalog. It reads no provider model list over the network.
- Ian can overturn each of these.

## Scope

- Changes: `crates/thinkthen/src/core/backend_profile.rs` and `backend_profile/images.rs`, `core/adapters/systemone/backends.rs`, `core/adapters/systemone/images.rs`, `core/adapters/systemone/images/local.rs`, the OpenAI adapter's built-in row, `crates/thinkthen/src/config.rs`, `crates/thinkthen/src/config/backends.rs`, the help topics, `specification/backends.md`, `specification/settings.md`, `specification/recording.md`, a new ADR, the built-in catalog file and its generator check. The coordinator narrows this when the ADR lands.
- Starts after 0590 lands on main, because it moves the TypeSafe image route.
- Checklist:
  1. Finish the inventory above on main after 0590: every per-backend and per-model fact, its source and its date. Each becomes a built-in catalog entry with provenance.
  2. Write the ADR for the catalog's schema and resolution order; a reviewer accepts it before code. It replaces the admitted-model clause of ADR 0121 and the parts of ADR 0114 and ADR 0117 it changes.
  3. Add the built-in catalog and its loader, with values equal to today's.
  4. Read the user's catalog: `api` on configured backends, the `models` maps, `like`, image headers, and refusals at load for an unknown key, an unknown reference, a `like` loop, an image fact on an API type with no image wire, and a limit above a safety limit.
  5. Resolve the catalog at the command edge and pass typed facts inward; `core` reads no file or environment.
  6. Move `--profile` to the catalog's limit keys, lower only, and delete `image_profile` and the setup IDs.
  7. Generate the specification tables from the catalog and check them in `spec`.
  8. Add `thinkthen help backends`.
  9. Delete the in-code tables the catalog replaces.
- Keeps: every 0.2 request byte for byte under the built-in catalog; every refusal sentence that still applies; zero sends on a refused input.

## Proof

The routine suite passes unchanged under the built-in catalog. Replayed cases against loopback show:

- `"jev-1.15.0": {"like": "jev-1.14.0"}` on `typesafe`: one image within the limits sends with the image header, and one past them is refused with zero sends.
- A new backend declared with only `url`, `key_env` and `model` answers a text question at loopback with that variable's key.
- `jev-latest` refuses an image with zero sends until the user declares it.
- A profile that raises a limit is refused at load.
- `thinkthen help backends` shows OpenRouter's Jev at 32,000 context tokens and TypeSafe's Jev without it, each with its source.

## Defers

- New image wires, such as OpenRouter's image format and OpenAI's image parts, and new formats, such as WebP, are separate tickets that add an API type's wire and then use this catalog.
- Addresses built from environment variables, such as Cloudflare's account ID in its URL, wait for a user who needs a direct Cloudflare backend.
- Storing the catalog's model name with each answer, so 0.4 can compare one model across backends, goes to the 0.4 grading work.
- The proxy's use of the catalog belongs to 0.5.
