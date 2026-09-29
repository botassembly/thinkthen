Status: Open. Filed 2026-09-29 from Ian's dictation after experiment 413 ran Liquid's d1 through `thinkthen check` and Beatles Bench's hard subset. Depends on the providers page issue filed the same day; this page names one provider that page lists.

# Docs: a page for the Liquid d1 backend, its API, and signing up

## The problem

Ian asked for a documentation page about the Liquid d1 decision model, the API it exposes, and how a user signs up for it. A reader who hears "Liquid works with ThinkThen" has nowhere to land: `site/src/pages/install/backends.astro` stays vendor-neutral, the README names only TypeSafe, and no page walks a user from Liquid's console to a working `thinkthen` call.

Experiment 413's `RESULTS.md` reports the following evidence for the page:

- The address is `https://api.liquid.ai/decisions/v1`, and ThinkThen posts to its `systemone` endpoint unchanged. No code change and no new adapter.
- The model is `d1:free`, listed by `GET /decisions/v1/models`. That endpoint's metadata reads release date 2026-09-22, while Ian named the public launch day as 2026-09-29; the page should carry the launch day and note the endpoint metadata. Output tokens are always zero and usage reports input tokens only.
- A key comes from `console.liquid.ai` under Dashboard, API Keys, prefixed `liquid_`, and goes in `THINKTHEN_API_KEY` beside `THINKTHEN_BASE_URL`.
- `thinkthen check` passes except one critical: a one-sided `decide` criteria question sends a null criteria value Liquid refuses (issue `2026-09-29-systemone-adapter-sends-null-criteria-liquid-d1-refuses.md`). The page should say two-sided or criteria-less decide questions pass until that lands.
- Liquid publishes no price. `d1:free` billed nothing during the experiment.

## Done when

A docs page under `site/src/pages/` walks a reader from Liquid's console to a passing `thinkthen check` and a first answered question, states the null-criteria limitation, and names the model, the address, and the capture dates of every claim.

## Builder handoff correction

Reviewed0301 source8286b99d7 corrects explicitly null `noul` descriptions. An absent `--true` or `--false` side was already omitted; do not repeat the original missing-side diagnosis. The code and local Rust/R/TypeScript checks are accepted, while the hosted Liquid check remains pending. Use its eventual receipt before describing the corrected build as remotely verified.
