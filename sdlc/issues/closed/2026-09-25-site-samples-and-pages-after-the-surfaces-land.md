# The site's samples and pages after the surfaces land

Status: Closed on 2026-09-30. Merged into `../2026-09-30-site-replay-folders-have-no-fixture.md`, items 3 and 4, which marketing owns.

## 1. Run the library and database samples

`site/scripts/smoke.mjs` runs only the command-line `.sh` samples. `site/examples/SKIP` excludes every `.py`, `.ts`, `.rb`, `.R`, `.rs`, `.c` and `.sql` sample, and `site/README.md` lists an installed-host smoke runner under "What is not here yet". A library or database tab can drift from its surface with no check failing.

Done when every sample on the site is run by its surface's check or by a site runner against the installed package, and prints the output the page shows.

## 2. A check that refuses status words

Ian, 2026-09-24: every surface ships together, so no copy says one part is done and another is not. The pages no longer show preview, planned, beta, ships first, not run yet or comes with 0.1. No check keeps them out.

Done when a site check fails the build on any of those words.

## 3. A pandas page

The catalog lists pandas among the bindings, but the site has no pandas page. `site/README.md` lists it under "What is not here yet".

## 4. A stray code tag after a table

An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag after `</table>`. The rest of the page renders in monospace. The smallest case is `<table><tr><td><code>{v}</code></td></tr></table>`, which builds to `</table><code> <p>After the table.</p>`. Site tables avoid it by building rows with `.map()`. No check guards against a return.

Done when a check over `dist/`, run beside `check-links.mjs` in `npm run build`, fails on `</table>` followed by a start tag.
