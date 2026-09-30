# Docs: a providers page that names what ThinkThen supports

Status: open for marketing's site page, which owns `site/`. Shortened 2026-09-30. Close this issue when the page lands.

Ian asked on 2026-09-29 for a page about providers and which ones ThinkThen supports, including System One providers such as Liquid's d1. The README names TypeSafe Jev and Liquid d1 and links Awesome ThinkThen. No site page answers "which providers can I use?"

## Providers today

- TypeSafe Jev, the default backend at `https://api.typesafe.ai/v1`, model `jev-1.13.0`.
- Liquid d1, `https://api.liquid.ai/decisions/v1`, model `d1:free`. Experiment 413 ran it through `thinkthen check` and a 505-question Beatles Bench hard subset on 2026-09-29. The hosted `check` passed on 2026-09-30 (`09ebcc5ce`).
- OpenAI's Decisions API, announced 2026-09-29 and not released. The repo holds no record of it. List it only from a named announcement, and claim nothing unverified.

Ticket 0334 (ADR 0114) adds named backends: `typesafe` reads `TYPESAFE_API_KEY` and `liquid` reads `LIQUIDAI_API_KEY`. The page should match whatever has landed.

## Done when

A page under `site/src/pages/` lists each provider with its address, model names, the date and result of our check, and a link to its own page or signup. It marks announced providers as unreleased, links the Awesome ThinkThen list (`github.com/botassembly/awesome-thinkthen`) for the rest, and adds no provider without a named check run or announcement.
