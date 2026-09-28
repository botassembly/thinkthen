# Quick Fix qf-readme-priority-order: reorganize the README around functions, install, and bindings

Ian reviewed the tutorial pass and asked for a different order: what the functions are, then how to install the command, then links to every language and binding. He also asked for a critical read, not another cleanup. This pass reorders the README around that priority and gives every binding a place in one index.

## Result

- **The ten functions are now the first thing a reader sees**, in one table: function, the question it asks, the answer it prints. The old opening showed five commands and never listed the product surface.
- **Install is now its own section**: build from a checkout, put the binary on `PATH`, then check it on a recorded answer. The old "First run" was a demo with no install step. The release-artifact lines wait for `release-and-install-for-0-1`; this section is shaped for them.
- **Languages and bindings get one index table**: Rust, Python, TypeScript, Ruby, R, C, Polars, DuckDB, SQLite, PostgreSQL, each linking its own README. One sentence names the eleven proven bindings that merge next, and the README shape every binding must carry.
- The answer cache, usage counts, and what-it-is-not-for drop below the product surface. The "Four names" table and the planning links move under `## Contributing`.
- The how-to front window keeps its seven rows and its `| How to | The job |` header untouched; only its location moved. `python3 sdlc/scripts/pages` passes: "1 coming, 22 green".

## Claims checked before writing

- The ten functions' printed answers, each against `specification/` and the type contract.
- Two drafts were corrected before landing: `--details` carries probabilities and the request digest, not run facts (`--facts` does), and Rust and Polars link the engine natively rather than binding the C door.
- The batching bullet keeps its sources from the prior pass: `specification/records.md`, `specification/question-file.md` line 103, ADR 0048.

## Known gap for the release scrub

The sentence naming the eleven unmerged bindings is temporary. It goes away when the table carries their rows, and the install section gains its artifact lines at go-live.

## Correction to qf-readme-tutorial-pass

That record claims `sdlc/scripts/lint` passed. It did not. A pipeline hid the exit code behind `tail`. Lint is red on main for an unrelated reason: `CLAUDE.md` is a symlink to `AGENTS.md`, the lint reads through it, and `AGENTS.md` stands at 5,532 characters against the 5,000 cap the check enforces (`b6184355`, 2026-09-24). The overage came with the 2026-09-28 capacity-guidance edits (`46e50418`, `18e3a64b`). Both README passes are unaffected: `pages` passes and no README rule fails. The red lint belongs to the steering file's owner.

## Checks

- `python3 sdlc/scripts/pages`: pass, "1 coming, 22 green".
- `sdlc/scripts/lint`: fails on `AGENTS.md 5532/5000 characters`, unrelated and predating this pass. Not fixed here, because the file is steering text that governs the builders.
