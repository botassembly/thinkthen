# Merge trial as a build: the break list and the retarget question

Status: Closed on 2026-09-25 as superseded. The branch merge never happened; the per-surface port replaced it.

Date: 2026-09-22. Source: the library-team lane ran the trial merge in a throwaway copy of the surfaces worktree (`/tmp/thinkthen-merge-trial`, deleted after the run). Nothing in the real worktree was touched and nothing was pushed from the copy; the merge commit `1c45dcf` exists only in the deleted copy and is not a branch anywhere.

## The setup

- Surfaces side: `6a829b8` (tip of `surfaces` at the time, in sync with `origin/surfaces`).
- Main side: `b154f27`, fetched fresh from GitHub (`git ls-remote` confirmed `b154f27` = local `origin/main`), 213 commits ahead of the merge base `b11a2b0`.
- `git merge --no-ff origin/main` reported **two text conflicts**, both in issue files, both additive sections of the same documents:
  - `sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md` (branch's "Phase A landed" section against main's Polars ruling and experiment close-out);
  - `sdlc/issues/2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md` (branch's "Lane B item 8" section against main's "Sharpened by external comparison" section).
  - Resolution used: the union, keeping both sides' sections in file order; only the six conflict marker lines were removed. Main's copies do not carry the branch sections and the branch copies do not carry main's, so nothing was duplicated and nothing was dropped.
- Everything else auto-merged. Surfaces changed nothing under `crates/` or the root `Cargo.toml` since the fork (verified: `git diff --stat b11a2b0..6a829b8 -- crates Cargo.toml` is empty), so main's fold — `5e1dafd` deleted `crates/thinkthen-core` into `crates/thinkthen` — applies wholesale and silently. That is exactly why the text view looked harmless and the build does not.

## The break list

### 1. Both manifests point into the deleted crate — cargo exits 101 before compiling anything

`contract/Cargo.toml:19` and `standin/Cargo.toml:19` still declare:

```
thinkthen-core = { version = "0.0.1", path = "../crates/thinkthen-core" }
```

Verbatim output for `cargo check --manifest-path contract/Cargo.toml` on the merged tree:

```
error: failed to load manifest for dependency `thinkthen-core`

Caused by:
  failed to read `/tmp/thinkthen-merge-trial/crates/thinkthen-core/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
```

exit code 101, with no compiler output at all. `standin/Cargo.toml` fails identically (cargo exit 101).

### 2. The code imports a public API the folded crate no longer exposes

`contract/src/lib.rs:46-47` imports `question_sha256` and `{Question, QuestionFile, QuestionFileError, Threshold, Typed, Verb, resolve}`. `standin/src/lib.rs:51-52,173,639-652,722` imports `adapters::built_in`, `{Backend, Evidence, ModelName, Plan, Reply, Value}`, `Answer`, `Outcome`, and `recording::Exchange`.

The consolidated crate keeps `mod core;` and `mod engine;` **private**; its public surface is the CLI `entry` and the `__internal_doctest` shim. None of the symbols above is reachable.

### 3. The break propagates to every Rust consumer of `contract`

`standin` plus all eight Rust surfaces declare a path dependency on `contract/`: `libraries/{python,typescript,ruby,r,c}`, `databases/{duckdb,sqlite,postgresql}`. Eleven lockfiles also record the old path dependency and must regenerate on any route: contract, standin, and the nine consumer workspaces (duckdb, postgresql, sqlite, c, python, r/thinkthen/src/rust, ruby, rust, typescript/addon).

### 4. What does not break

- Main's own crate builds in the merged tree: `cargo check --workspace` exit 0 (`Finished dev profile` in 6.71s).
- `sdlc/scripts/policy.py` passes on the merged tree: exit 0, `policy: checked 95 resolved packages`, `policy: every accepted table, ban list, and dependency matches`. The single-crate allowance does not fire — the script checks only the root workspace and `crates/thinkthen`, and never looks at `contract/`, `standin/`, `libraries/`, or `databases/`. That is the review's coverage gap, not a failing check.
- `sdlc/scripts/ratchet.mjs` passes on the merged tree: exit 0, `ratchet: crates 33840/33840` (main raised the ceiling 23755 → 33840; the surfaces' roughly 18,800 Rust lines sit outside its scope).
- Remaining `thinkthen-core` strings outside the code are comments and docs only: `crates/thinkthen/clippy.toml` reasons, `crates/thinkthen/src/cli/asked.rs:4`, `conformance/tools/build_conformance.py:76`, `conformance/tools/validate_conformance.py:57`, and issue/planning prose. Ignored build artifacts in the copy (dist/, build/) are not committed material.

## The retarget question, answered with evidence

**No published crate exists to depend on.** Both the deleted crate and the consolidated crate declare `publish = false`; nothing carries these names on a registry.

**The only in-tree candidate is `crates/thinkthen`** (path `../crates/thinkthen`, package name `thinkthen`). Tested as the minimal retarget: two manifest lines in `contract/Cargo.toml` and `standin/Cargo.toml`, no source edits:

```diff
-thinkthen-core = { version = "0.0.1", path = "../crates/thinkthen-core" }
+thinkthen-core = { package = "thinkthen", version = "0.0.1", path = "../crates/thinkthen" }
```

The manifest now resolves, and compilation dies at the first import. Verbatim:

```
error[E0432]: unresolved import `thinkthen_core::question_sha256`
  --> src/lib.rs:46:5
   |
46 | use thinkthen_core::question_sha256;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `question_sha256` in the root

error[E0432]: unresolved imports `thinkthen_core::Question`, `thinkthen_core::QuestionFile`, `thinkthen_core::QuestionFileError`, `thinkthen_core::Threshold`, `thinkthen_core::Typed`, `thinkthen_core::Verb`, `thinkthen_core::resolve`
  --> src/lib.rs:47:22
   |
47 | use thinkthen_core::{Question as CoreQuestion, QuestionFile, QuestionFileError, Threshold, Typed, Verb, resolve};
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^  ^^^^^^^^^  ^^^^^  ^^^^  ^^^^^^^ no `resolve` in the root
```

exit code 101; the standin check fails on the contract dependency with the same errors. Renaming the import paths cannot help — the items do not exist publicly in the folded crate. **So the minimal retarget does not make the merged tree build.**

**The route that does build, proven:** restore the deleted crate as of the merge base and leave the manifests alone.

```
git checkout b11a2b0 -- crates/thinkthen-core
cargo check --manifest-path contract/Cargo.toml   # exit 0, Finished dev profile in 2.98s
cargo check --manifest-path standin/Cargo.toml    # exit 0, Finished dev profile in 3.48s
```

Both checks re-run with cargo exit 0. On this route the merged tree's contract and standin compile exactly as on the surfaces branch; the 11 lockfiles keep resolving; nothing else changes.

## Decision points for the build team

- **(a) Keep `crates/thinkthen-core` through the merge** (the restore above, as the surfaces side's merge resolution). Proven to build; keeps the surfaces' gate meaningful; costs one crate the fold intended to remove. Retirement condition: when the production engine consumes `contract/`, the crate disappears with the stand-in.
- **(b) Expose an equivalent public API from `crates/thinkthen`.** That is not a retarget but an API design decision on the folded crate (today it is deliberately private), and it belongs with the architect's production API plan.
- **(c) Leave `contract/` and `standin/` unbuilt on main until the production engine lands.** This empties the surfaces' acceptance coverage on main and widens the already-open gate gap; recorded for completeness, not recommended.

Recommendation: **(a)**, with the retarget to the production API recorded as the follow-on when that API exists — at which point the "one changed dependency per surface" claim becomes true for the two crates that still name the stand-in directly.

## Attachment: exact commands and outputs

```
# 1. the merge (throwaway copy; merge commit local only)
git merge --no-ff origin/main -m "Trial merge: main into surfaces (build trial)"
# -> two CONFLICT lines (both issue files above), "Automatic merge failed"

# 2. pre-retarget build attempts
cargo check --manifest-path contract/Cargo.toml   # exit 101, manifest load error (quoted above)
cargo check --manifest-path standin/Cargo.toml    # exit 101, same error

# 3. minimal retarget (both manifests), then:
cargo check --manifest-path contract/Cargo.toml   # exit 101, E0432 pair (quoted above)
cargo check --manifest-path standin/Cargo.toml    # exit 101, contract dependency fails

# 4. restore route
git checkout b11a2b0 -- crates/thinkthen-core
cargo check --manifest-path contract/Cargo.toml   # exit 0
cargo check --manifest-path standin/Cargo.toml    # exit 0

# 5. context checks on the plain merged tree
cargo check --workspace                           # exit 0 (main's crate)
python3 sdlc/scripts/policy.py                    # exit 0, 95 packages
node sdlc/scripts/ratchet.mjs                     # exit 0, crates 33840/33840
```

The copy was deleted afterwards with `rm -rf /tmp/thinkthen-merge-trial`. No key was used, no paid call was made, and no loopback stub was needed (build-only trial).
