# 0572: Refresh the docs to sell one idea

Status: WITHDRAWN.

Opened as: 2026-10-13, superseded by 0575. In progress. Jarvis owns it as the docs team, on local branch `docs/0.2-refresh` in its own worktree, kept in sync with main and not merged until 0.2 ships and Ian reviews it. The 0.2 team does not work on it.

Milestone: 0.2.1

Superseded by: 0575

## Outcome

The README and thinkthen.dev sell one idea: put Jev-class decision models in your code, whatever model, language or database you use. ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it, and it makes these models simple to call. A first-time Python developer, DuckDB analyst or agent builder can say in one minute what they get, whether it works in their setup and what to paste.

- Main pages show Bash only and say once that every function runs across the bindings. Each other language has its own page. One SQL section covers the three databases, with differences shown inline.
- The home page and README answer "why not call the model yourself?" with claims that each link to evidence. Docs may sell; every claim is true.
- No page carries release status, contract jargon or unshipped work. Fine-tuning, GEPA, the proxy and the hosted service stay out.

## Evidence

- Starts from: Ian's rulings on 2026-10-12, and a read-only docs review of main at `d53df9d2a` that found the home page buries the ten functions under 25 language tabs, never says why to use ThinkThen, shows no code for 15 bindings on main, and that the README and function pages carry release and contract prose.
- Keeps: replayed examples as smoke tests, how-tos named by job and reader, the Markdown twins and llms files, the build checks.
- Changes, in waves: README and home page; status and contract prose out of every page; one page per language; one SQL section with setup pages; menu merged to Start, Functions, Recipes, Languages, Databases; the agent skill says why to pick ThinkThen; a page test where a fresh agent given one page plays each reader and must succeed.
- Proof: `npm run build` passes in `site/` on the branch; each wave gets an independent review; the page test passes for the home page, one function page and one language page.
- Defers: fine-tuning, GEPA, proxy and hosted pages to 0.3 and later.
