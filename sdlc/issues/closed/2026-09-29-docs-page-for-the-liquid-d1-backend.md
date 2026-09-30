# Docs: a page for the Liquid d1 backend, its API, and signing up

Status: Closed on 2026-09-30. Merged into `../2026-09-29-docs-page-naming-supported-providers.md`, section 2, which marketing owns.

Ian asked on 2026-09-29 for a page about Liquid's d1 decision model, its API, and how a user signs up. `site/src/pages/install/backends.astro` stays vendor-neutral, and no page walks a user from Liquid's console to a working `thinkthen` call.

## Facts for the page

Sources: experiment 413's `RESULTS.md`, the README's Liquid line, and the closed [null-criteria issue](2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md).

- The address is `https://api.liquid.ai/decisions/v1`. ThinkThen posts to its `systemone` endpoint with no adapter change.
- The model is `d1:free`, listed by `GET /decisions/v1/models`. That endpoint's metadata reads release date 2026-09-22, while Ian named the public launch day as 2026-09-29. The page carries the launch day and notes the metadata.
- A key comes from `console.liquid.ai` under Dashboard, API Keys, prefixed `liquid_`. Today it goes in `THINKTHEN_API_KEY` beside `THINKTHEN_BASE_URL`. Ticket 0334 (ADR 0114) will add a named `liquid` backend that reads `LIQUIDAI_API_KEY`; the page should follow whichever form has landed.
- Output tokens are always zero, and usage reports input tokens only. Liquid publishes no price. `d1:free` billed nothing during the experiment.
- `thinkthen check` passed on the 2026-09-30 hosted recheck (`09ebcc5ce`), after ticket 0301 stopped sending explicit null descriptions.
- A first `check` can outlast the default 30-second timeout. The Beatles Bench team reported on 2026-09-30 that its first `check` against d1 timed out at 30 s and passed with `--timeout 90`. The page tells the reader to pass `--timeout 90` on a first run. Coordinator default, which Ian can overturn: say so in the docs now. A longer default timeout on the `liquid` built-in is a per-backend limit, which ticket 0334 defers, so it waits for 0334's follow-up. Stumble register row 19 tracks the stumble.

## Done when

A page under `site/src/pages/` walks a reader from Liquid's console to a passing `thinkthen check` and a first answered question, and names the model, the address, and the capture date of every claim.
