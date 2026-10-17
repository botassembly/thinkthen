# 0572: Docs sitemap

Ian approved this shape on 2026-10-12, with all five recommendations below. It serves [ticket 0572](../tickets/0572-refresh-the-docs-to-sell-one-idea.md) and the [ruling](../decisions/2026-10-12-docs-sell-one-idea.md). It says what the site holds, not what is done.

## Rulings

1. Models is its own top menu item.
2. The Beatles Bench talk becomes one page with an anchor per slide. The 29 slide pages redirect to their anchors.
3. URLs move to match the menu. Every old address redirects straight to its final page, with no second hop.
4. pandas and Polars keep their own language pages. The Python page links both.
5. The shell techniques live under Languages › Bash, not Recipes.

## Top menu

Start · Functions · Models · Languages · Databases · Recipes, then search and the theme switch.

## Pages

- **Start**
  - `/`: what you get, whether it fits your setup, what to paste. Bash only.
  - `/start/`: install and get a first answer in two minutes, even with no key. Merges `/install/` and the install part of `/install/shell/`.
  - `/start/tutorial/` from `/learn/tutorial/`.
  - How it works, listed on the Start page, each built around what the reader gains:
    - `/start/pay-once/`: many records share one request, and each question keeps its own saved answer. From `/learn/functional/` and `/learn/caching/`.
    - `/start/replay/`: record and replay for offline tests. From `/learn/recording/` and `/learn/caching/`.
    - `/start/tune/`: re-read saved answers at a new cut, `runs audit`, `runs diff`, and "a high probability is not proof".
    - `/start/plan/`: see the request and its cost before you send.
    - `/start/your-data/`: the field pointer, key secrecy, it never acts, stop and resume.
    - `/start/speed/` from `/learn/overhead/`: one engine and its measured overhead.
  - Reference, listed on the Start page: `/start/files/` (from `/learn/files/`), `/start/settings/` (from `/install/settings/`), `/start/configuration/` (from `/install/configuration/`).
- **Functions**
  - `/functions/`: pick the right function from one table.
  - The ten function pages. Bash only, and each says once that the function runs in every binding. Their language samples move to the language pages.
  - `/functions/answers/` from `/learn/answers/`.
  - `/functions/question-file/`, with `/learn/question-sets/` merged in.
  - Tools, grouped on the Functions page: `/functions/audit/`, `/functions/diff/`, `/functions/transform/`.
- **Models**, from `/install/backends/`
  - `/models/`: point the same call at TypeSafe, another provider or your own machine.
  - `/models/{typesafe,liquid,openai,perplexity,openrouter}/`, `/models/{ollama,local}/`, `/models/system-one/` (retitled without the jargon), `/models/other-servers/`, and `/models/check/` from `/functions/check/`.
  - Room later: tested model guides, one per model, with test date and timings.
- **Languages**
  - `/languages/`: find your language, install it, paste a first call. Grouped by kind.
  - `/languages/bash/`: the command-line facts from `/install/shell/` and the technique index from `/how-tos/bash/`, with the 11 techniques under it.
  - 21 pages: Python, pandas, Polars, TypeScript, Ruby, R, Rust, C, C++, Objective-C, COBOL, Ada, Java, Kotlin, Scala, C#, Go, Swift, Zig, PHP, Dart. Each holds install, a first call, all ten functions, lists and streams, errors, settings and caching, models, what differs in that language, and links back to the Bash function pages.
  - `/languages/agents/`: the MCP server and the agent skill. New.
- **Databases**
  - `/databases/`: all ten functions in one set of SQL, DuckDB syntax, with SQLite and PostgreSQL differences inline.
  - `/databases/{duckdb,sqlite,postgresql}/`: load and first query. PostgreSQL operator detail lives on its page.
- **Recipes**
  - `/recipes/`: finished jobs grouped by reader. Merges `/how-tos/`, the shell jobs from `/how-tos/bash/`, `/recipes/` and `/guides/`.
  - About 18 recipes: business teams, data science, ops and security, shell jobs, pipelines.
- **Footer only**: the Beatles Bench talk at `/learn/beatles-bench/`, the blog, About, Trust, What it will not do, GitHub, the specification, the agent skill, `llms.txt`, `llms-full.txt` and the sitemap.

## Cut

`/guides/`, `/learn/`, `/install/`, `/how-tos/`, `/how-tos/bash/`, `/learn/recording/`, `/learn/question-sets/` and the 29 slide pages. Each becomes a redirect. About 106 redirects in all.

## README

The one idea and a Bash example; the ten functions linked to their pages; why not call the model yourself, each claim linked; any model (`/models/`); any language or database (`/languages/`, `/databases/`); install with the no-key try; links, with "All how-tos" pointing at `/recipes/`.

## Size

133 listed pages become 101: 67 generated from data, 34 written by hand.
