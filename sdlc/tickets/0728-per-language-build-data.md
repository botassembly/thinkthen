# 0728 — Each language owns its build data

Status: OPEN.

Milestone: 0.2.1

Depends on: 0727

Reviews: revision 440c8ba, accept

## Outcome

- Changes: `sdlc/tools.json`, `sdlc/tools/`, `sdlc/scripts/tools.py`, `sdlc/scripts/release-pack`, `sdlc/scripts/release-pack.d/`, `build-tools/results/registry.py`, `build-tools/results/registry/`, `site/src/data/catalog.mjs`, `site/src/data/catalog/`.
- The tool manifest keeps shared groups in `sdlc/tools.json` and moves each language's group to `sdlc/tools/<lang>.json`; `tools.py` reads both. The manifest's `schema` member goes, per the no-version ruling.
- `release-pack` keeps its shared driver and runs one step file per language from `sdlc/scripts/release-pack.d/<lang>`.
- `build-tools/results/registry.py` and `site/src/data/catalog.mjs` read one small data file per language.
- Every phase 3 ticket's Changes claim names its own language's file in these folders, not the shared files.

### Proof

- `tools.py` resolves every group, `release-pack` builds the same archive list (checked with `package-inventory.py`), the generator's output is unchanged, and the site builds, all on the M5, before and after.
- `pm next --milestone 0.2.1` shows no collision between phase 3 tickets for different languages on these files.

## Evidence

After 0727, the ticket review of 2026-10-19 found more shared files that make language tickets collide: `sdlc/scripts/release-pack` (12 tickets), `sdlc/tools.json` (11), `build-tools/results/registry.py` (7), `site/src/data/catalog.mjs` (7). Each ticket edits only its own language's part, but one shared file forces them to land one after another and blocks batching. Smaller overlaps between two or three tickets (`sdlc/scripts/children`, `release-workflow`, `build-lines.mjs` and a few coverage scripts) land in sequence and need no split. This lands right after 0727 and before the first phase 3 batch.
