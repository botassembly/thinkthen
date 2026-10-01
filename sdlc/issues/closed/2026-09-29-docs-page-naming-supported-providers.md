# Docs: a providers page, and a page for the Liquid d1 backend

Status: closed 2026-10-01 by site tickets 0042 (`8c5aabc00`) and 0043 (`23d845868`), as the site owner reported. Owner: marketing. Resolution: the Backends table and each provider page show the newest recorded `check` for TypeSafe Jev (2026-09-26) and Liquid d1 (2026-09-30). The Liquid d1 page tells a reader to pass `--timeout 90` on a first `check`. The table links Awesome ThinkThen. The OpenAI Decisions API has a placeholder page marked announced and not released. It cites OpenAI's DevDay announcement of 2026-09-29. Each provider page links all twenty language pages.

Ian asked on 2026-09-29 for a page about providers and which ones ThinkThen supports, including System One providers such as Liquid's d1. He also asked for a page about Liquid's d1 decision model, its API, and how a user signs up. The README names TypeSafe Jev and Liquid d1 and links Awesome ThinkThen. No site page answers "which providers can I use?" `site/src/pages/install/backends.astro` stays vendor-neutral, and no page walks a user from Liquid's console to a working `thinkthen` call.

## 1. The providers page

Providers today:

- TypeSafe Jev, the default backend at `https://api.typesafe.ai/v1`, model `jev-1.13.0`.
- Liquid d1, `https://api.liquid.ai/decisions/v1`, model `d1:free`. Experiment 413 ran it through `thinkthen check` and a 505-question Beatles Bench hard subset on 2026-09-29. The hosted `check` passed on 2026-09-30 (`09ebcc5ce`).
- OpenAI's Decisions API, announced 2026-09-29 and not released. The repo holds no record of it. List it only from a named announcement, and claim nothing unverified.

Ticket 0334 (ADR 0114) landed named backends: `typesafe` reads `TYPESAFE_API_KEY` and `liquid` reads `LIQUIDAI_API_KEY`. Ticket 0339 (ADR 0115) designs a built-in `ollama` backend; the page adds it when 0339 lands.

Done when a page under `site/src/pages/` lists each provider with its address, model names, the date and result of our check, and a link to its own page or signup. It marks announced providers as unreleased, links the Awesome ThinkThen list (`github.com/botassembly/awesome-thinkthen`) for the rest, and adds no provider without a named check run or announcement.

## 2. The Liquid d1 page

It follows the providers page. Sources: experiment 413's `RESULTS.md`, the README's Liquid line, and the closed [null-criteria issue](2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md).

- The address is `https://api.liquid.ai/decisions/v1`. ThinkThen posts to its `systemone` endpoint with no adapter change.
- The model is `d1:free`, listed by `GET /decisions/v1/models`. That endpoint's metadata reads release date 2026-09-22, while Ian named the public launch day as 2026-09-29. The page carries the launch day and notes the metadata.
- A key comes from `console.liquid.ai` under Dashboard, API Keys, prefixed `liquid_`. Since ticket 0334, `--backend liquid` reads it from `LIQUIDAI_API_KEY`. The older form, the key in `THINKTHEN_API_KEY` beside `THINKTHEN_BASE_URL`, still works.
- Output tokens are always zero, and usage reports input tokens only. Liquid publishes no price. `d1:free` billed nothing during the experiment.
- `thinkthen check` passed on the 2026-09-30 hosted recheck (`09ebcc5ce`), after ticket 0301 stopped sending explicit null descriptions.
- A first `check` can outlast the default 30-second timeout. The Beatles Bench team reported on 2026-09-30 that its first `check` against d1 timed out at 30 s and passed with `--timeout 90`. The page tells the reader to pass `--timeout 90` on a first run. Coordinator default, which Ian can overturn: say so in the docs now. A longer default timeout on the `liquid` built-in is a per-backend limit, which tickets 0334 and 0339 defer. Stumble register row 19 tracks the stumble.

Done when a page under `site/src/pages/` walks a reader from Liquid's console to a passing `thinkthen check` and a first answered question, and names the model, the address, and the capture date of every claim.
