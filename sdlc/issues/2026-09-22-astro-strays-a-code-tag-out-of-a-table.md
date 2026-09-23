# Astro strays a code tag out of a table

Status: Open. Worked around in `site/src/pages/install.astro`. Found 2026-09-22 in a visual pass over the built site.

## What happens

An expression inside a `<code>` element written straight into a `<table>` makes the Astro compiler emit a second, unclosed `<code>` start tag right after `</table>`. The browser leaves it open, and every heading and paragraph after the table renders in monospace.

The install page carried this. Its first table was written row by row, so everything from "The download script will be served from this site" to the footer was monospace, in both themes and at every width.

## The smallest case that shows it

```astro
---
const v = 'brew install x';
---
<table>
  <tr><td><code>{v}</code></td></tr>
</table>
<p>After the table.</p>
```

The build writes `</table><code> <p>After the table.</p>`.

Measured with the version of `astro` in `site/package-lock.json` on 2026-09-22.

## What changes it

- A `<span>` in place of the `<code>` emits clean HTML.
- A literal string in place of the expression emits clean HTML.
- Rows built by `.map()` over an array emit clean HTML, whatever is inside the cell.

The last one is the workaround, and it is what every other table on the site already does. That is why no other page carried the bug.

## What to do

Build table rows from data with `.map()`. Never write `<code>{expression}</code>` straight into a table. A check over `dist/` for a start tag directly after `</table>` would catch a return of this, and belongs with the link check if it comes back.
