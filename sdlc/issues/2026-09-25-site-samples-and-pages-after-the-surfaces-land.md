# Check the built site for a stray code tag after a table

Status: open for marketing, which owns `site/`. Shortened 2026-09-30 from "The site's samples and pages after the surfaces land". The site rebuild (`201bfcca0`, `fa3e9848e`, `e0989cd87`) settled items 1 to 6 and 8 to 12: no status words, samples under `site/examples/functions/` that `site/scripts/smoke.mjs` runs on every surface, a Beatles Bench section, current prune and timeout lines, `.value` accessors for Ruby, TypeScript and R, and the SQL warm blurb's `thinkthen_batch(1)` condition. Git history holds their text.

## The problem

An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag right after `</table>`. The rest of the page then renders in monospace. The smallest case is `<table><tr><td><code>{v}</code></td></tr></table>`, which builds to `</table><code> <p>After the table.</p>`. Site tables avoid it by building rows with `.map()`. No check guards against a return.

## The fix

Add a check over `dist/` that fails on `</table>` followed by a start tag. Run it beside `check-links.mjs` in `npm run build`.

Done when the build fails on the smallest case above and passes on main.
