Status: Open. Filed 2026-09-29 from Ian's dictation. He asked for a documentation page about providers and which providers ThinkThen supports, including System One providers such as Liquid's d1.

# Docs: a providers page that names what ThinkThen supports

## The problem

ThinkThen supports any server that speaks the System One interface, and the docs say so without naming anyone but TypeSafe. Two providers exist today and a third is announced:

- TypeSafe Jev, the default backend at `https://api.typesafe.ai/v1`, model `jev-1.13.0`.
- Liquid d1, `https://api.liquid.ai/decisions/v1`, model `d1:free`, verified by experiment 413 on 2026-09-29 through `thinkthen check` and a 505-question Beatles Bench hard-subset run.
- OpenAI's Decisions API, announced 2026-09-29 at DevDay and not yet released. Ian will supply its details. The page should hold a place for it and claim nothing unverified.

A reader has no page that answers "which providers can I use?" The Awesome ThinkThen list (`github.com/botassembly/awesome-thinkthen`) names servers that pass the compliance check; the docs page and the list should agree and link.

## Done when

A providers page under `site/src/pages/` lists each supported provider with its address, its model names, the status of our verification and its date, and a link to its own page or its signup. It marks announced-but-unreleased providers as such, links the Awesome ThinkThen list for the rest, and adds no provider without a named check run or a named announcement.
