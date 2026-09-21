# The maintainability test: a ninth function, counted twice

2026-09-21, the surfaces worktree (`surfaces`), brief item 6. The
throwaway function was `probe`: one text in, the same text out, help line
"Return the text unchanged, for testing." It was added by hand to every
surface, counted, then generated from one table, removed, and added again
by the new path.

## The first count: 19 files, by hand

Base `28e4025` to `4b386ed`. Files touched, per surface:

| Where | Files |
| --- | --- |
| shared | `contract/src/lib.rs`, `contract/include/thinkthen.h`, `standin/src/lib.rs` (3) |
| Python | `src/lib.rs`, `thinkthen/__init__.py` (2) |
| TypeScript | `addon/src/lib.rs`, `index.js`, `index.d.ts`, `index.mjs` (4) |
| Ruby | `src/lib.rs`, `lib/thinkthen.rb` (2) |
| R | `src/rust/src/lib.rs`, `R/extendr-wrappers.R`, `R/thinkthen.R` (3) |
| Rust | `src/lib.rs` (1) |
| C | `src/lib.rs` (1) |
| DuckDB | `src/lib.rs` (1) |
| SQLite | `src/lib.rs` (1) |
| PostgreSQL | `src/lib.rs` (1) |
| **total** | **19** |

Nineteen files, 2.1 a surface on average, TypeScript at 4 and R at 3. The
judgment, by the brief's own rule (more than about two a surface, or
boilerplate typed twice): **high — generate the repeated parts.**

The count came from `git diff --name-only 28e4025..4b386ed` after the four
probe commits; the probe was also proven to answer on all nine surfaces at
runtime (a one-liner a surface, unicode in the text), and removed with
`git` edits back to the same anchors.

## The generator

- `functions.toml` at the root: one row a function — name, inputs, output,
  and the help line. The eight verbs take their lines from the public
  vocabulary; the helpers' lines are authored there. The table's order is
  the contract's own listing of the eight verbs.
- `scripts/generate_functions.py` writes the two files that are pure name
  lists: `libraries/python/src/generated.rs`, the pyo3 registration block,
  and `libraries/typescript/index.mjs`, the ESM re-export face. With
  `--check` it writes nothing and fails when either file is stale;
  `scripts/check_surfaces.sh` runs it that way, so a forgotten
  regeneration fails the tree.
- `SURFACES.md` now says: edit the table, run the generator, write the
  surface-specific parts by hand.

The generator commit is `39cadca`. The Python surface's registration moved
from a hand list in `_thinkthen` to `generated::register(module)?`, and
every loaded Python and TypeScript check still passes
(`libraries/python/check.sh`, `libraries/typescript/check.sh`, run
directly).

## The second count: 21 files; 19 by hand, 2 by the generator

The probe was re-added by the new path — one table row, the generator, then
the hand parts — uncommitted, measured, and reverted. `git status
--porcelain` showed 21 changed files:

- the table row: `functions.toml` (1)
- hand-written: the same 19 files as the first count, minus
  `libraries/typescript/index.mjs`, plus `functions.toml` (19)
- generator-written: `libraries/python/src/generated.rs`,
  `libraries/typescript/index.mjs` (2)

**The number of files a function touches did not drop.** What changed: two
name lists are no longer hand-maintained, drift is impossible while
`--check` runs, and the name is written once for those lists. The cost of
that is the table row and the generator run.

## Why the count cannot drop further

Each ruled verb's body is its own shape in its own host, and `name,
inputs, output` cannot derive a binding signature. The repeated parts a
name table can own are the lists, and the two clean lists are owned now.
The remaining hand cost, in order: the contract trait and the stand-in
implementation (3 shared files, unavoidable — a function is a method and
an implementation), then per surface the function's body and its
registration/wrapper line. Candidates for a later reduction: the Python
package's `__init__` name list (a pure list, currently hand), and the
Ruby/TypeScript/R registrations, which need a shape vocabulary (arity and
per-host signatures) larger than this test should decide.

## What was left green

- `scripts/check_surfaces.sh` end to end, with the loopback stub up on
  every surface's port: `all landed checks green`, exit 0 — including the
  PostgreSQL surface's disposable container, the Ruby builder container,
  the DuckDB build, and the conformance slices.
- `python3 scripts/generate_functions.py --check`: `generated files match
  functions.toml (13 functions)`.
- Probe remnants: `grep -rn
  "thinkthen_probe\|tt_probe\|Op::Probe\|ProbeScalar\|\"probe\""
  contract/src contract/include standin/src libraries/*/src libraries/*/lib
  databases/*/src functions.toml scripts` prints nothing (exit 1). The
  conformance file was never touched by the probe.
- The probe removal commit is `121abdd`; the four probe commits
  (`a43f6aa`, `c0a29b7`, `09ed18c`, `4b386ed`) stay in the branch history
  as the measurement's record.

## Unchecked

- The second-path probe was compile-verified only; the first-path probe,
  whose bodies are identical, was proven at runtime on all nine surfaces.
- `databases/sqlite/thinkthen.so` is a pre-existing untracked build
  artifact (the check script's own copy step), not staged and not part of
  the counts.
