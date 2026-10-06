# 0451: Create production Guides and a reusable lesson template

Status: in progress. Lane 2 builds the accepted section/template on ticket/0451-guides-section; whole-change code review and landing checks remain with the queue owner.

Milestone: 0.2
Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

The documentation site has a Guides section for using System One models in production and one reusable lesson template.

## Evidence

- Starts from: Reviewed main 0f87a50bd and the existing SDK plan. PM message `2026-10-06-pm-vision-ships-in-0-2-and-the-sdk-stays-one-endpoint-while-the-proxy-owns-the.md`, ask 5; experiment 0034 design, recorded spike at 8ec4fbffc0dbf8ceaa0fea5607749ff2f683a2a0 and subsequent OpenRouter controls. 0036 reports are pending.
- Keeps: Existing text behavior, typed SDK parity, six errors, cancellation, secrecy, spend limits and zero-send strict replay. Core remains free of I/O; the one Rust engine and native file reader remain shared.
- Changes: Add /guides/ navigation/index and a template using existing layouts/content/draft selection. Each published guide teaches one lesson, declares prerequisites and limits, links to a short video and blog article, and uses checked examples. Topic inventory includes batching, caching, curation, retrieval-augmented decisions, vision and fine-tuning.
- Proof: Existing site build, links, search/sitemap/navigation checks prove the index/template work. A sample draft fixture stays out of normal published indexes; a complete lesson fixture renders its actual video/blog links. Reuse existing docs replay for executable examples rather than new receipt tooling.
- Defers: Producing videos/blogs, a full lesson catalog, training services and proxy implementation.

## Dependencies and ownership

Marketing supplies experiment-based lesson drafts and actual video/blog URLs. This ticket ships the section/template; article/video production is marketing’s work and does not justify invented URLs. Coordinate approved image/SDK rulings from 0447–0450.

## Design notes

Draft lessons can omit media until marketing supplies them; published lessons require real links. Do not publish blank lesson stubs or placeholder media links. Index and template can land without a full topic catalog. Existing Functions/Learn/Recipes content stays reachable; guides do not add business policy to the SDK. Public examples use generic data and compatibility claims, never model rankings.

The section ships without a published lesson catalog. `site/templates/guide.md`
is the reusable authoring starting point; lesson Markdown under `site/src/guides/`
uses the existing renderer and base layout. Local draft pages remain excluded
from public lists, search, sitemap and Markdown exports. Independent rendered
fixtures live under `site/scripts/fixtures/guides/`, outside the content catalog.
Marketing retains ownership of real lesson bodies, media and editorial URL checks.
The shared mailroom contains the PM ask and experiment 0036 batching guidance;
no marketing guide draft or media URLs were available at implementation intake.
