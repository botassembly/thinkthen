# Handoff: the `surfaces` branch

For the agent that picks this branch up next. Written 2026-09-22 in the
handoff lane on branch `surfaces` at `8c83765`. Every pointer below was
resolved and every command was run in that lane; the gate output in
section 2 is from its own run. `FINDINGS.md` is the closure record,
`MERGE-NOTE.md` is the build team's list, and this page is the door.

## 1. What this is

Ten functions, nine surfaces, one contract, one conformance file — all
complete and green against the stand-in engine.

| Piece | Where | State |
| --- | --- | --- |
| The contract | `contract/` — the Rust trait, the public types, and `contract/include/thinkthen.h`, the C door | landed |
| The stand-in engine | `standin/` — implements the contract whole | landed |
| Nine surfaces | `libraries/{python,typescript,ruby,r,rust,c}` and `databases/{duckdb,sqlite,postgresql}`, each with its own `check.sh`, README, and NOTES | landed |
| The one conformance file | `conformance/conformance.json`, 74 cases, validator at `conformance/tools/validate_conformance.py` | landed |
| The Polars door | Python's `decide` and `score` take a plain list or a Polars column and return a column over the same Rust spine; the wheel holds no Polars import | landed |
| The C door | the drawn `thinkthen_*` signatures from the slide sample, plus the `_opts` twins (cancel token, per-call deadline); design in `libraries/c/DESIGN.md` | landed |

The ten functions: decide, choose, score, tag, filter, rank, find,
annotate, recognize, relate. `functions.toml` holds the fourteen ruled
public names (the ten plus `decide_many`, `question`, `details`, `usage`).

**The seam, stated once.** Every surface binds the stand-in engine today.
Production is the real engine implementing `contract/`: each surface
changes one dependency line, from `thinkthen-standin` to the real crate,
and nothing above the contract changes. The conformance file is that
swap's acceptance test — the same 74 cases every surface already replays.
The stand-in never invents an answer: recognize and relate replay the
recordings in `standin/data/recognize-replay.json`, and a text the
recordings do not hold is a usage error naming what is missing; the
decide family answers from the stand-in's own keyword rule.

## 2. How to build and check

Toolchain: Rust 1.93.1 (pinned in `rust-toolchain.toml`), python3, Node,
R 4.3.3, a C compiler, and Docker for the Ruby builder and the three
database containers. The Rust surface's Polars-door test runs under
`RUSTUP_TOOLCHAIN=1.95` because Polars needs it there.

**The rung.** `sh sdlc/scripts/surfaces` runs the offline checks and the
conformance validator always. With `cargo` on PATH it runs the full gate
too; without `cargo` it prints the skip line and exits 0.

**The offline checks alone** (no toolchain, no network, no stub):

```
$ python3 scripts/generate_functions.py --check
generated files match functions.toml (14 functions)
$ python3 scripts/check_public_names.py
ok python: 25 names, all ruled or documented
... one line a surface ...
every surface's public names are ruled or documented
$ (cd conformance && python3 tools/validate_conformance.py conformance.json)
OK: 74 cases validated: schema, grammar, digests, wire contract, offline replay
```

**The full gate.** `scripts/check_surfaces.sh` builds and checks the
contract, the stand-in, and every landed surface. The wire suites need
the loopback stub, one instance a surface port:

```
$ STUB=/home/ian/workspace/experiments/205-thinkthen-libs/shared/target/release/stub-backend
$ for p in 8211 8212 8213 8214 8215 8216 8217 8218 8219 8231; do
    STUB_PORT=$p STUB_DELAY_MS=300 "$STUB" &
  done
$ sh sdlc/scripts/surfaces
```

The ports: 8211 Python, 8212 TypeScript, 8213 Rust, 8214 Ruby, 8215 R,
8216 C, 8217 DuckDB, 8218 SQLite, 8219 PostgreSQL, 8231 the stand-in's
own wire test. Without the stub each wire suite skips with its own
message and the rest still runs.

**Run the gate in the foreground.** A gate started as a background job of
a non-interactive shell (a trailing `&`, `nohup`) runs with SIGINT
ignored, and Python inherits an ignored SIGINT, so the Python surface's
fast-cancel test fails with "the batch ran deaf" while the branch is
fine. This lane lost two runs to that before running it in the
foreground; that run is green.

**Verified state.** This lane's own run on `8c83765` with the stubs above,
`sh sdlc/scripts/surfaces`, exit 0:

```
ok       choose
ok       score
ok       tag
ok       annotate
ok       details
ok       usage
ok       warm
ok       recognize
ok       relations
ok       relate
12 of 12 examples ok
== postgres surface: wire suite against the stub on 8219
wire green: decide answers on the wire, usage counts sends and tokens
all landed checks green
```

The run covered every section: the contract tests, the stand-in's null
and wire tests, the conformance validator (74 cases), the generated
lists, the public names, and all nine surfaces' checks — each surface's
slide sample, function examples, conformance slice, and its wire section
where the surface has one (PostgreSQL's own wire suite green on 8219).

## 3. What is stubbed versus real

The stand-in is the stub. Nothing else is: every shim is real code over
the contract — argument conversion, host lifetimes, interrupts, result
presentation, and nothing else. No second scheduler, no second answer
rule, no second bulk implementation exists anywhere in the tree.

- The recordings: `standin/data/recognize-replay.json`, generated from
  `experiments/225-recognize-harvest-package` by
  `conformance/tools/build_recognize_cases.py`.
- The three named divergences that remain:
  1. **The per-subject relate arm.** Case R03 shares its text with R04
     but records `"form": "per-subject"`; the replay serves the pairs
     form, the ruled method, and the validator and the Rust replay test
     skip the case after asserting its marking.
  2. **The defect case, engine-only.** No conformance case exercises a
     defect, on principle: a defect is a broken engine invariant, not a
     property of a recorded reply. Main's `25-defect-fault` stays marked
     engine-only at the merge, no public door gets a fault hook, and each
     surface's binding tests prove the defect kind maps to the host's
     error.
  3. **The SQLite shim cache, marked for the swap.** The temporary answer
     map and hit counter in `databases/sqlite/src/lib.rs` are marked in
     the source and are deleted together when the real engine lands.
- The full list — including the five `shaped-to-contract` exchanges
  (choose, score, tag, annotate, find; the stub cannot distinguish
  options or labels) and case 17's known gap (the stand-in's
  `cache_answers`) — is `conformance/DIVERGENCES.md`.

## 4. What is prepared for the build team

**The merge note** (`MERGE-NOTE.md`), one line a section:

1. The conformance file: the build team picks the one name and owns the
   union; main's `25-defect-fault` stays marked engine-only.
2. Stale planning text on main: four named `file:line` edits.
3. The rulings issue was edited on both sides: hand-merge and keep both
   blocks.
4. The gate ladder: bring `contract/`, `standin/`, the libraries, and
   the databases under main's fmt, clippy, deny, and ratchet.
5. Items other teams own: the 225 check count, the PostgreSQL glibc pin,
   the deck findings.
6. Things the ADR and the real engine must state: the interrupt shape,
   the usage counter, the fork preconditions, and `strength`.
7. The two contracts the branch extended: the question file's
   `source`/`target` ends, and `relate`'s 255-record refusal.

The fuller input with the same items and the blocker proposals is
`MERGE-NOTE-INPUT.md`.

**The packing spec from the wire probe.**
`sdlc/issues/2026-09-22-wire-probe-can-one-request-carry-many-states.md`
(on main; raw rows in
`/home/ian/workspace/experiments/thinkthen-wire-probe-2026-09-22/`).
The batched form exists and is the structured `state` object: one
condition, one row object per state, one named question per row, one
answer per row, with the request floor paid once (two rows billed 372
input tokens against 573 for two single requests; three rows 416 against
857). The engine must encode that object in the `systemone` adapter, name
the row questions `r0` through `rN`, map answers back by position, and key
the recording and cache by the whole request, because a row's answer
depends on its neighbors. A list `state` must never be sent — it answers
one silent, ambiguous judgment. The width, the off-by-default rule, and
which verbs may pack stay the databases ADR's decision.

**The invalid-case fixtures** for the production parser:
`conformance/fixtures/parser-invalid/` (eight files, all stamped
synthetic in the folder's README) with `contract/tests/parser_fixtures.rs`
— eight tests locking today's behavior and two `#[ignore]`d tests that
run at the swap.

**The blocker proposals** (`MERGE-NOTE-INPUT.md` item 9, each marked
"Ian decides the outward act"): SQLite keeps the direct link with the
load-time 3.41 floor and nothing is filed upstream now; DuckDB takes the
vendored ~20-line `duckdb-rs` bind-callback patch, with the C++ fork left
to the build team and the upstream proposal only after the patch runs
in-tree.

**The punch list's waiting-for-capability list**
(`sdlc/records/2026-09-21-punch-list-report.md`), with the engine entry
points named:

- Python `recognize_stream` → `recognize_many(ask, texts)`, and Python's
  `score` column loop → `score_many`.
- R `tt_recognize_column` → `recognize_many`, and R
  `tt_choose`/`tt_score`/`tt_tag` →
  `choose_many`/`score_many`/`tag_many`.
- The production question-set parser (the fixtures and the two ignored
  tests run at the swap).
- Recognition boundary and overlap policy, and the conflicting relation
  request designs — the architect's, in Rust.
- Databases lane: a key channel from the host (PostgreSQL's
  `thinkthen.api_key` refusal becomes delivery), and a re-armable DuckDB
  cancel token or a statement hook.

The C options/ownership item closed after that report: the design is
`libraries/c/DESIGN.md` and the door implements it.

## 5. Where everything is

| Path | Holds |
| --- | --- |
| `FINDINGS.md` | The closure record: the brief's items, the rulings' additions, what was not run, what Ian can overturn |
| `MERGE-NOTE.md` | The build team's seven items, in final form |
| `MERGE-NOTE-INPUT.md` | The fuller cross-side input: ten items including the blocker proposals |
| `SURFACES.md` | How to add a surface or a function |
| `NOTES-packaging.md` | Every packaging rehearsal and the Mac visits, with commands |
| `NOTES-maintainability.md` | The ninth-function test and the generator's two counts |
| `NOTES-rulings-wave.md`, `NOTES-settle-wave.md` | The two fix waves and their verification runs |
| `libraries/<language>/`, `databases/<engine>/` | One folder a surface: README, NOTES, `check.sh`, sources, tests, examples |
| `contract/`, `standin/`, `conformance/` | The contract, the stand-in (recordings in `standin/data/`), the one conformance file with `tools/` and `fixtures/` |
| `scripts/check_surfaces.sh`, `scripts/generate_functions.py`, `scripts/check_public_names.py`, `functions.toml` | The gate, the generator, the name check, the function table |
| `sdlc/scripts/surfaces`, `sdlc/ratchet.json` | The rung and the size ratchet |
| `sdlc/records/2026-09-21-punch-list-report.md`, `sdlc/records/2026-09-21-one-shape-confirmation.md` | The punch-list report and the ten-pick confirmation with the Polars mirror |
| `sdlc/issues/2026-09-21-rulings-on-the-surfaces-and-the-next-experiment-brief.md` | Ian's rulings and the experiment brief (this branch's side of a two-sided page) |

On main, needed by the merge but not on this branch:
`sdlc/issues/2026-09-21-product-rulings-on-the-surfaces-adversarial-review.md`
(the rulings the branch obeys), the settled-details ledger
`sdlc/issues/2026-09-21-the-settled-details-across-all-languages.md`, the
wire-probe issue named in section 4,
`sdlc/planning/library-team-architecture-punch-list.md`, and
`sdlc/planning/polars-plan.md`.

## 6. What is not here and why

- **No publishing.** Nothing is on any registry and no name is claimed.
  The registry names are Ian's list, and the reminders for it belong to
  the marketing side. Every artifact is 0.0.1, built and installed from
  local files only; the first release is 0.1.0.
- **No serve mode, no spreadsheets, nothing HTTP.** Ruled out for now;
  nothing here depends on it.
- **The plugin expression is parked** with its conditions: the
  column-owning shape, the in-body budget rule, and a documented support
  window (experiment 228; the plan is `sdlc/planning/polars-plan.md` on
  main).
- **macOS is done five-for-five.** C and Python carry cross-built darwin
  arm64 artifacts; TypeScript, Ruby, SQLite, DuckDB, and PostgreSQL were
  built on the Mac with their logs beside them in each `dist/`; Rust is
  source by design; R is source through R-universe, whose builders make
  its macOS binaries.
- **The known unchecked list carried from `FINDINGS.md`:** the plugin
  expression (experiment 216), the warm-Polars-pool fork proof
  (experiment 214), an R user-installed SIGINT handler after load, and
  the defect kind in the conformance file (by principle). The macOS items
  that list carried are closed by the Mac lane (`NOTES-packaging.md`,
  "The Mac visits"). Two more carried knowingly: case 17's stand-in
  divergence is the engine's to fix, and the wire probe leaves the rate
  and the partner-lane questions untested by design.
