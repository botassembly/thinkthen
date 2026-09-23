# Ian's rulings on the surfaces, and the next experiment brief

Status: Closed on 2026-09-22. The rulings are recorded, and the surfaces experiment closed on all seven brief items.

## Rulings, 2026-09-21

1. **ADR 0017 is accepted.** Ian: "We can always fix it." The build team's review still runs before the merge ticket, and a finding amends the page.
2. **The cache is on by default** at the XDG cache folder. The three guards in the ADR ship with it.
3. **One repository.** All six libraries and three database extensions live in this repository, organized by language and by database. One version number.
4. **Linux and macOS first.** Windows is out of the first release.
5. **The registry names are Ian's.** His list is at the root of his notes folder.

## Ruled: the default cache limit is 100 MB

Ian said "let's just do a 10 MB limit" and asked whether 10 MB is enough. Measured on the 19 recorded entries behind the slides: an entry averages 554 bytes and takes one 4 KB block on disk. 10 MB holds about 2,500 answers by disk use. That serves a shell user. It does not serve a database: `thinkthen_warm` fills the cache and the query then reads it, so a table over 2,500 rows would evict its own warm answers before the query reads them. The marketing side recommends a 100 MB default, about 25,000 answers, as a setting in the configuration file, and a line in each database page that tells the reader to raise it for a large table. Ian ruled 100 MB on 2026-09-21: "That's a good compromise." ADR 0017 carries it.

## The goal

Ian: the code for the eight functions is "basically perfect and ready to bring over", and it comes over "in a maintainable, organized way".

## What blocks the goal today

- The nine experiment surfaces are bench code. The hand-off page says so: "the experiment surfaces, not the ruled shapes". Most run `decide` and `filter` only.
- The surfaces call a stand-in engine, and no written contract fixes the function list both engines must meet.
- The 20 conformance cases do not exercise all eight functions on all nine surfaces.
- No surface has been packaged, installed on a clean machine, and run.
- The code has no agreed layout.

## The brief for the experiment team

The work lands in a worktree of this repository, in the final layout, and touches nothing under `crates/`.

1. **The contract.** One Rust interface and one C header from ADR 0017: the engine value, a built question, the eight functions, `details`, `usage`, cancel, the six error kinds. The stand-in implements it now. The real engine implements it later, and the move is one changed dependency.
2. **The layout.** `libraries/<language>/` and `databases/<engine>/`, one conformance file, one script that builds and checks all nine, one version number. A short page says how to add a surface and how to add a function.
3. **All eight functions on all nine surfaces, in the ruled shape.** The acceptance test is the slide code in the marketing repository's `surfaces.md`: each sample runs as drawn against the stand-in and gives the answer in its comment. A sample that cannot work as drawn is a finding, and the slide changes.
4. **The cases grow** to cover every function, the "not sure" value, the six error kinds mapped to each host's own error classes, and cancel, on every surface.
5. **The packaging rehearsal.** For each of the nine: build the real installable package on Linux and on macOS, install it in a clean container or a clean account, run one slide sample, record what broke. R goes through R-universe first. The default cache folder follows XDG on Linux and the platform folder on macOS.
6. **The maintainability test.** Add a throwaway ninth function to all nine surfaces and count the files touched. If the count is high, generate the repeated parts from one table of functions: name, inputs, output, and the help line from the public vocabulary. Remove the throwaway function afterward.
7. **Cancel in the hosts not yet proven:** TypeScript's `AbortSignal`, Ruby, and R.

No paid call is needed. Every check runs against the stand-in or a recording.

## What Ian can overturn

The brief's order and scope.

## Phase A landed, 2026-09-21, on branch `surfaces`

The contract and the layout, in six commits on `surfaces` (worktree `worktrees/thinkthen-surfaces`), touching nothing under `crates/`:

- `ef3767c` **The contract.** `contract/` holds one Rust crate and one C header (`contract/include/thinkthen.h`). The trait covers the eight verbs, `decide_many` as decide's bulk spelling, `details`, `usage`, a cancel token and a deadline in one options value, the six error kinds with the retry signal, and `Settings` reading `THINKTHEN_BASE_URL` and `THINKTHEN_CACHE`. The header carries the JSON door, `thinkthen_decide`, the ruled `thinkthen_decide_many`, engine new/free, the error codes, and `THINKTHEN_UNSURE`. Questions are built from parts through the one file grammar, so parts and files give the same digest; `from_file` fails with the local kind. Seven contract tests, green.
- `db2eace` **The stand-in.** `standin/` is experiment 211's engine moved in and grown to the whole contract: every verb through the true core wire shapes, the null backend answering every verb with the conformance file's own numbers, the pid check behind the atomic slot, the process width gate (default 4, `ENGINE_WIDTH` overrides), sends counted per send, tokens counted from replies, the request limit refusing a bulk call before its first request. Null suite 4 green; wire suite 3 green against the stub (`cancel: 40 records, token set at tick 2: wall 312 ms, requests at return 32`; 100 single calls held at 32; the forked child answers).
- `69e9385` **The layout.** `libraries/{python,typescript,ruby,r,rust,c}` and `databases/{duckdb,sqlite,postgresql}`, each README naming its slide sample; `VERSION` holds 0.0.1 for every surface.
- `731a058` **The conformance file in.** `conformance/conformance.json` unchanged from job 3, the validator beside it, and `DIVERGENCES.md` giving the 12/3/5 record its named home. `python3 tools/validate_conformance.py conformance.json` → `OK: 20 cases validated`.
- `7145073` **The one script.** `scripts/check_surfaces.sh` builds and tests the contract and the stand-in, runs the wire tests when the stub is up (and says `wire tests skipped: no stub on the loopback` when it is not), validates the conformance file offline, and calls each landed surface's `check.sh`. Green end to end both ways.
- `b622e5a` **The page.** `SURFACES.md`: the layout, the acceptance rule (the slide samples as drawn), how to add a surface, how to add a function.

One contract decision to name: `rank` refuses a question that itself names a threshold and takes the grammar's default cut as no rule at all, because the default lands on every decide question and rank would otherwise refuse them all. The distinction rides a `threshold_named` flag the contract records from the file's own JSON.

What is unchecked, stated plainly:

- `find` on the stand-in judges each unit alone and takes the best; the relative one-request form belongs to the real engine, and no slide sample draws find.
- The five `shaped-to-contract` exchanges stay shaped until the real engine's first live run re-captures them.
- The cache settings are carried (`Settings.cache`, `cache_bytes`) and unspent: the stand-in holds no disk cache, `cache_answers` stays zero, and the 100 MB cap is enforced by nothing in this tree yet.
- `max_requests` refuses bulk calls only; single calls do not count against it.

Phase B inherits: a trait to bind, a header to compile, one conformance file to read, a check script to hook into, and nine folders with their samples named. The Python band sample's `# None` comment will not reproduce against the stub's three-bucket rule (`I was charged twice` holds no keyword, so the stub answers 0.03 and a band calls it No): that is a finding to report when it fires, not a surprise.

## Ruled by Ian, 2026-09-21: the data frame is Polars

Ian ruled: "Polars. The Rust-based data frame project that's similar to pandas, but let's just do Polars." Python's data frame container is Polars, not pandas. What follows:

- The Python surface's `annotate` and `recognize` data-frame forms take and return Polars DataFrames; the bulk form accepts a Polars column. pandas leaves the surface entirely and the slide sample runs on Polars.
- Polars is an optional dependency: the wheel does not carry it, the surface feature-detects it when a DataFrame arrives, and the install line is `pip install thinkthen[polars]`.
- The slide sample's shape does not change — `tt.annotate("form.json", df, on="body")` — and no slide text names a data frame library, so the deck needs no change unless the marketing side wants Polars named.
- R's data frame stays R's own; the databases are untouched.
- Open, not ruled: a native Polars door on the Rust surface (Polars is itself a Rust library, so the door is cheap if ever wanted). Nothing builds on this until asked.

What Ian can overturn: the optional-dependency shape (a hard dependency instead), and the open Rust question.

Clarified by Ian, 2026-09-21: the Python surface supports both pure Python and Polars DataFrame Python — both containers, first-class. All scalability and vectorization, whatever Polars offers, is done in Rust code, not Python code: Python never loops a row, never chunks, never bridges through a Python lambda. The proof is equality at the gate — the width bench through a Polars column must read the same wall time and the same 32 in flight as the plain-list form.

2026-09-21: the Polars plan exists at `sdlc/planning/polars-plan.md`, written after the Polars ruling and Ian's clarification that both Python containers ship first-class with all scaling and vectorization in Rust. It lists the risks and experiments 212 through 216 and authorizes nothing.

## The surfaces experiment closes, 2026-09-21

All seven brief items landed on branch `surfaces` (base `b11a2b0` through `35f5044`), plus the Polars door and the pandas checks under Ian's later rulings. The findings page at the branch root carries the whole report in the experiment's habit: what was observed by command, what diverged, what Ian can overturn. Headlines: the contract and the stand-in; nine surfaces with every slide sample running as drawn (three sample findings filed, one defect fixed); twenty-seven conformance cases; the packaging rehearsal with real macOS artifacts for C and Python and the exact blockers for the three databases; the maintainability test's two honest counts; cancel proven in TypeScript, Ruby, and R; the Polars equality proof at 0.016 percent with identical buffer addresses; the pandas checks closed with both corrections to their own issue. The merge of `surfaces` into main is the build team's gate. The recognize work follows Ian's signal.
