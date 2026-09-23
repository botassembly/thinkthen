# NOTES — the gate-process lane (third review)

The fail-then-pass probes for every fix this lane owns, per the closure
rule. Each probe names its command and both outputs.

## 1. The ratchet, green and blocking

**Probe (failing, the review's finding):** `node sdlc/scripts/surfaces-ratchet.mjs`
at tip `37240fd` with ceiling 22,039 → `is 22,086 non-blank .rs lines,
ceiling is 22,039 ... raise it` and exit 1, while
`scripts/check_surfaces.sh` called the tip green — the ratchet was not in
the gate's path.

**Fix:** the gate now runs the ratchet and the workspace Clippy pass as
its first steps (`df8a9c7`), so a red tip cannot buy a green gate. The
ceiling lands at the committed tip's measured actual (23,833, counted
from `git archive HEAD`, not the working tree) with the raise's
justification in the commit: the third-review fix waves — the
memory-safety guards, the SELECT-only relate, the cancel reworks, the
strict-parser and state-table fixes, and their tests — earn their lines;
duplication was searched first (the panic guard and skip-table reader
moved into the contract this wave; the DuckDB panic-guard adoption is
recorded for its lane).

**Interleaving, stated:** the duckdb-3 and r-ruby-3 lanes were mid-flight
at this writing with 14 dirty files; their commits land after this one
and will raise the measured total again. The ceiling is at MY tip's
actual; their lane or the verifier sets the next number deliberately.

## 2. Lints and deny coverage

**Probe (failing):** `[lints]` tables absent from
`tools/wire-stub/Cargo.toml` and `libraries/typescript/addon/Cargo.toml`
(`grep -c "\[lints\]" → 0`); `cargo deny` covered 1 of 13 lockfiles (the
root's; the review's count).

**Fix and passing outputs:** both workspaces carry the surface rules
(`77a22de`); the wire stub is strict-Clippy clean after one
redundant-binding fix (`cargo clippy --locked --quiet --all-targets --
-D warnings` → exit 0); the addon gains Debug derives, docs, and its
unused-import removal (its napi-macro items stay recorded). The R crate's
rust-version rises to the repository pin 1.93.1 (`507b978`, its `cargo
test --locked --quiet --lib` → `ok. 3 passed`). `lint-workspaces` now
answers `cargo deny` for every workspace from one config generated out of
the root rules, ending `lint-workspaces: every landed workspace is clean`
with eleven `deny: … advisories ok, bans ok, licenses ok` lines, DuckDB
deferred with its stated reason (cargo metadata needs crates this machine
never fetched; the lane owns the offline fetch story) and Ruby skipped
(ruby not on PATH). The per-surface license widenings (LLVM-exception,
Zlib, BSL-1.0) and the two recorded unmaintained-crate advisories are
named in the script where they apply.

**Bug found on the way, fixed:** the workspace path contains a slash and
my first deny prefix used it inside a sed expression (`s/^/deny:
libraries/rust: /` → `unknown option to \`s'`); replaced with printf.

## 3. Hermeticity truths

- `--locked` on every cargo build/test in the gate and the seven clean
  surface scripts (`df8a9c7`; two misplacements found and fixed by
  `bash -n` + inspection before commit). DuckDB's and Ruby's scripts get
  theirs when their lanes' in-flight files land.
- The Ruby container builds `--release --locked --offline` with the
  first-pull story recorded at the build site (this lane's commit): one
  `cargo fetch --locked` on a fresh clone, no network after.
- Python pins carry hashes: `uv pip compile --generate-hashes --offline
  requirements-dev.txt` (works from the local cache) and `uv pip install
  --offline -r requirements-dev.txt --dry-run` → `Audited 12 packages …
  Would make no changes`.
- Not mine and open: the DuckDB build's unpinned `duckdb` and the
  git+https reference — their lane's dirty files were mid-fix
  (`tools/version.env` staged for exactly that).

## 4. The private-reference checker

**Probe (failing, the review's crash):** `python3
scripts/check_no_private_refs.py --root /tmp/notgit` (a tree with a
planted private name) → `subprocess.CalledProcessError … git ls-files
… exit status 128`, a traceback.

**Fix and passing output:** the checker walks the plain tree when the
root is not a checkout, skipping ignored dirs and packaged suffixes:
same probe → `FAIL: 1 private reference(s) outside sdlc/` +
`a.md:1: names the private repository`, exit 1. Its test suite passes
(`ok: the checker refuses a planted private name and home path, exempts
sdlc/, and passes a clean tree`); the repo-wide run ends `ok: no private
repository names or home paths outside sdlc/`.

**sdlc/ cleaned with the scope decision recorded at the enforcement
point** (`507b978` + this lane's commits): the historical records keep
their evidence with the private name genericized to "the deck
repository" and home paths made relative; the Mac log's absolute home-directory
forms became `~/…`; `libraries/typescript/NOTES.md`'s literal path in
prose became words. The private-name and home-path grep over `sdlc/`
(spelled in `scripts/check_no_private_refs.py`, not here, so this file
carries no literal to find) → no matches.

## 5. Records against the tip

- `MERGE-NOTE-INPUT.md`: 72 → 84 cases, and main's consumer path
  corrected to `crates/thinkthen/src/cli/conformance_tests.rs` (main
  deleted thinkthen-core; the old path was the review's stale line).
- `MERGE-NOTE.md`: the break list gains the third thinkthen-core
  reference (`libraries/r/tools/make-tarball.sh:31`); the same stale
  consumer line fixed.
- The duplicate 0069 renumbered ON MAIN where the collision lived
  (`07f9d77`): the wave-two lane had landed the R-interrupt record
  straight into main's series beside main's own 0069; it is 0074 there
  now.
- The Mac evidence states its provenance: built at `c3616dc`, not the
  tip, with the re-prove path named.
- The rulings awaiting ADRs are listed for the ADR owner on main
  (`35e4377`, record 0075): seven rulings, where each lives, what each
  ADR decides.
- The Ruby collector proof already observes a real collection at the tip
  (`libraries/ruby/tests/test_tick_gc.rb`: a finalizer on the block,
  `GC.start` from a second thread for the whole batch) — the review's
  finding was written against `37240fd`; their lane's landed work
  carries the proof. Their lane confirms it runs green in their gate.

## 6. The R install and the SQLite text

- The R install builds release (`--release` in `tools/config.R`, present
  at the tip) and now installs offline (`--offline` in
  `src/Makevars.in`, this lane's commit); `libraries/r/check.sh` →
  86 ok, `* DONE (thinkthen)`, `production install answers`.
- The SQLite packaging truth: the README carries the 3.50.0 floor and the
  direct-link story (`e5e19b9`, verified by grep); the one stale NOTES
  number is annotated inline as superseded.

## Open, with owners

- The DuckDB unpinned/git+https build inputs: the duckdb-3 lane's
  in-flight files.
- `--locked` for the duckdb and ruby check scripts: same lanes.
- The full `-D warnings` rollout across every workspace: measured at 105
  errors in nine clean workspaces (counts per workspace recorded in this
  lane's session), a mechanical wave with an owner — NOT claimed done
  here. The wire stub is the first strict-clean workspace.
- The final ceiling: the verifier sets it after the interleaved lanes
  land, per the raise rule.

## The re-measure (this lane's final item)

`bash scripts/check_surfaces.sh` (detached run, 2026-09-23 00:16–00:22)
ended `summary: green=841 skipped=63 diverged=18 failed=5 (wire: stub
up)` with five failure lines. Attribution, by command:

- Three lines were one drift: `FAIL duckdb: 12 public names vs 13
  expected` twice plus the checker's own self-test line — the DuckDB
  lane's landed commit removed the `thinkthen_instance_token` SETTING
  (the third review's unforgeable-identity fix) while the checker still
  expected it. **Probe (failing):** `python3 scripts/check_public_names.py`
  → `FAIL duckdb ... missing: thinkthen_instance_token`. **Fix:**
  the duckdb entry carries the twelve-function set with the
  LOAD-time-identity change recorded in the comment; **passing:**
  `ok duckdb: 12 names, all ruled or documented`, exit 0, and
  `python3 scripts/test_check_public_names.py` → `name-check tests
  green`.
- Two lines stay open in other lanes, stated in HANDOFF beside the
  summary: the TypeScript `25-find-none-fits` wrong answer (proven
  pre-existing by stash-and-rerun in this lane's earlier session) and
  the DuckDB recognize acceptance tripping its own new 255-record cap
  (`run.log`: `the relate query returned 256 records and relate asks
  about at most 255`).

HANDOFF carries the measured line with the five named and the two owned.
