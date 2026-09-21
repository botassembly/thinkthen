# The surfaces experiment: nine surfaces over one contract

Closed 2026-09-21 on branch `surfaces`, from base `b11a2b0` to this commit. Everything ran against the stand-in engine and the loopback stub: no paid call, no key, nothing published. The habit holds: what was observed by command, what diverged, what Ian can overturn. The lane notes under `libraries/*/NOTES.md`, `databases/*/NOTES.md`, `NOTES-packaging.md`, and `NOTES-maintainability.md` carry every command and output line.

## The seven brief items

1. **The contract** (`ef3767c`). One Rust interface crate and one C header: the engine value from settings, a question built from parts, the eight verbs plus `decide_many` as decide's bulk spelling and `thinkthen_decide_many` on the C door, `details`, `usage`, cancel, deadline, the six error kinds with the retryable signal. The stand-in implements it whole, everything 211 proved carried over.
2. **The layout** (`69e9385`, `731a058`, `7145073`, `b622e5a`). Nine folders under `libraries/` and `databases/`, one conformance file, one check script, one version number 0.0.1, and `SURFACES.md` on how to add a surface or a function.
3. **All eight functions on all nine surfaces**. Every slide sample runs as drawn with the answer in its comment, on the deck as rebuilt mid-run. Three findings: the Ruby score comment pins a distribution nobody guaranteed (filed in mktg, `3041f17`); the TypeScript last-object defect the packaging probe caught (fixed, `998b3a1`, with the ruled shape and a usage error for unknown keys); the band-decide comment is the real backend's answer against the stand-in's keyword rule (the sample's own recorded finding).
4. **The cases grew** (`28e4025`). Twenty to twenty-seven: `rank` and `find` gained their first cases, the local, deadline, and cancelled kinds theirs, the unsure value its choose and find shapes. `defect` has no case on principle: a defect is a broken engine invariant, not a property of a recorded reply. The schema stayed `thinkthen.conformance/1`, additive.
5. **The packaging rehearsal** (`NOTES-packaging.md`). All nine packaged from local files and installed into clean containers, one slide sample each, every container removed and recorded. Real macOS artifacts cross-built for C and Python. SQLite, DuckDB, and PostgreSQL cannot cross from this box — exact blockers recorded (the missing `libsqlite3` and CoreFoundation SDK stubs; pgrx driving the host compiler) with the ready commands for the Mac. Fixed in-lane: the R tarball's build junk (12.8 MB to 16.5 KB behind a `.Rbuildignore`) and the Rust path-dep versions. By design, recorded: the R tarball installs where the repository layout exists, which is the R-universe path.
6. **The maintainability test** (`NOTES-maintainability.md`). Hand-adding a ninth function touched 19 files, 2.1 a surface. The generator (`functions.toml` plus two generated files plus a `--check` in the script) protects the two pure name-list files from drift forever; re-adding by the new path touches 21, because surface bodies are inherently hand-work. The count was never the cost; the duplication was. The probe leaves zero net change: the tree differs from its base by exactly the generator's eight files.
7. **Cancel in the named hosts**. TypeScript: an `AbortSignal` stops a batch with the stub frozen after the return, the event loop free, the blocking engine on a libuv worker. Ruby: the tick interrupt — the VM lock releases for the wait, each tick re-takes it, a raise cancels, sent requests finish, the exception re-raises. R: the default handler proven; a user-installed handler after `library(thinkthen)` is marked unchecked.

## Added beyond the brief, by Ian's rulings

- **The Polars door** (`8fc2db7`). Both Python containers first-class: a plain list and a Polars column cross the same Rust spine at the same gate — 9.660 s against 9.662 s on the width bench, 0.016 percent apart, both 32 in flight — and the buffers Rust reads are the buffers Polars handed out, identical addresses. No Polars import exists in the wheel; the `[polars]` extra is a convenience, not a code path. The plan for the Rust door and the plugin expression is `sdlc/planning/polars-plan.md`, experiments 212 through 216.
- **The pandas checks** (`NOTES-packaging` is not where these live; see `libraries/python/NOTES.md`, "pandas checks", closed by `1ac9250`). Checks 1, 3, 4 pass on pandas 2.2.3 through 3.0.6; the width bench holds 0.18 percent across list, object pandas, Arrow pandas, and Polars. Two corrections to the issue: `str[pyarrow]` is not a valid spelling past 2.2.3 (use `string[pyarrow]` or the default `str` on pandas 3), and the fast path is pandas-3-only — pandas 2 columns cross at list speed and still work. The frame call now refuses with both remedies named, meeting the issue's bar.

## Not run, honestly

- macOS artifacts for SQLite, DuckDB, and PostgreSQL (the Mac's commands are recorded).
- The plugin expression (experiment 216) and the warm-Polars-pool fork proof (214): planned, not run.
- A user-installed R signal handler after load.
- The defect kind in the conformance file, on principle.

## What Ian can overturn

The generator (drop it; the two counts are the evidence). The optional `[polars]` extra versus a hard dependency. The plugin-versus-Series door question the plan leaves him. Naming pandas on the Python slide once the product side's corrected sentence is live. The merge of `surfaces` into main, which is the build team's gate to run.
