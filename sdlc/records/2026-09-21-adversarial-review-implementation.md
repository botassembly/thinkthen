# Implementation review — `surfaces` at `0801a32` (worktree `the thinkthen-surfaces worktree`)

## Verdict

**BLOCK on two items, OK with notes on everything else.** The branch's headline claims survive inspection: the four offset worlds are right in every host including the emoji case, the 255 refusal holds at every door that carries `relate`, the ruled vocabulary (`strength`/`probability`/`source`/`target`) is consistent in code, types, and the C header, the conformance validator proves replay rather than schema, and the generator's `--check` really gates. Two promises do not hold: the whole surfaces gate sits **outside the repository's gate ladder**, so nothing in `sdlc/scripts/{install,lint,test,spec}` can catch it rotting, and the **question file's relation ends are `from`/`to` in the landed parser while the ruling, the contract doc, the C header's reader and the manual say `source`/`target`** — eight of nine doors refuse the ruled spelling and the ninth (PostgreSQL) quietly accepts both.

I ran nothing. This session has no shell tool, so every statement below is static evidence (file and line), plus one grep-level proof of absence. The commands a supervisor must run are listed at the end; treat the "not run" label on each as honest.

## Findings, by consequence

**1. P1 — the surfaces' gate is off the ladder; a rotted surface check fails no rung.**
Classification: confirmed defect (process/verification). Violated promise: `AGENTS.md` "The gate ladder is `sdlc/scripts/{install,lint,test,spec}`… Run the whole ladder before handing back" and "A script that checks something runs from a rung, or it rots"; `SURFACES.md` "`scripts/check_surfaces.sh` | The one script that builds and checks everything landed".
Evidence: `sdlc/scripts/lint` runs policy, pages, ratchet, `cargo deny`, `fmt`, `clippy`, `doc`; `sdlc/scripts/test` runs `cargo test --locked --workspace --all-targets` plus probes/transforms/demos; `sdlc/scripts/spec` runs `mustmatch` and the demos. A grep of `sdlc/scripts/` for `check_surfaces|contract|standin|libraries|databases` returns **no matches**. `contract/Cargo.toml` declares `[workspace]` of its own, so `cargo test --workspace` at the root cannot reach the contract or the stand-in; `libraries/*` and `databases/*` are separate workspaces too. `sdlc/ratchet.json` counts `crates/**/*.rs` only (`{"directory": "crates", "max": 23755}`), so the new code carries no size ceiling either.
Consequence after merge: main's ladder stays green while any surface, the validator, or the generated name lists could be stale or broken. `fmt`/`clippy`/`deny` also never see `contract/`, `standin/`, `libraries/`, `databases/`.
Smallest fix: one rung, e.g. `sdlc/scripts/surfaces`, that runs the two offline, toolchain-free checks today — `python3 scripts/generate_functions.py --check` and `(cd conformance && python3 tools/validate_conformance.py conformance.json)` — and calls `scripts/check_surfaces.sh` when the host toolchains are present; call it from `test`.

**2. P1 — the question file's relation ends contradict the ruling and the contract's own doc.**
Classification: confirmed defect (contract vs code, user-visible). Violated promise: `contract/src/lib.rs:887-895` — "the spelling of the ends is `source` and `target` on every surface, C's returned JSON included, the command's own JSON, and the question file — no door converts anything"; `contract/include/thinkthen.h` — `spec_json` is "the recognize section of the question file".
Evidence: `contract/src/lib.rs:773-783` (`relation_from_value` requires `object.get("from")` and `object.get("to")`, and errors `the relation rule {name} has no {key} end`), reached from both `Recognize::from_json` and `Relate::from_json`. Every lane hit it and filed it in prose only: `databases/postgresql/src/lib.rs:175-206` accepts both spellings and normalizes, `databases/postgresql/NOTES.md:91`, `libraries/ruby/NOTES.md:152`, `libraries/python/NOTES.md:326-329`, `libraries/c/NOTES.md:254-257`, `libraries/python/tests/test_recognize_relate.py:114-116`. The conformance cases themselves carry `from`/`to` (`conformance/tools/build_recognize_cases.py:130-135`).
Reproduction: static only — a file holding `{"relations":[{"name":"works_for","source":"person","target":"organization"}]}` cannot pass `relation_from_value`, so it is a usage error on eight of nine doors and accepted on the ninth.
Smallest fix: make `source`/`target` the primary spelling in `relation_from_value` and keep `from`/`to` as the recordings' alias, or rule the other way and change the contract doc, four NOTES, and the deck. One edit either way.

**3. P2 — the settled `strength` rename never reached the docs, and the generator would revert it.**
Classification: confirmed inconsistency. Violated promise: `FINDINGS.md` "the rename from the interim `number` is `78204cb`/`14f418e`/`2356976`" and "every folder pasted its vocabulary sweep".
Evidence: `contract/src/lib.rs:851-860` ("The number field is the interim name…"), `standin/src/replay.rs:24` (module doc lists `confidence` among "the ruled field names"), `standin/src/replay.rs:61-63` ("the ruled host-facing field is `number` (the interim name)"), `standin/tests/recognize_replay.rs:9,151,204`, `conformance/NOTES.md` ("the numbers by their ruled names (`number` on a name…)"), and the generator `conformance/tools/build_recognize_cases.py:58-69` which writes `"number": entity["confidence"]`.
Consequence: the documented generator is one run away from failing the tree's own validator — `validate_conformance.py:262-266` requires each expect entity's key set to be exactly `{id, text, kind, start, end, strength}`.
Smallest fix: rename the key in `ruled_entity` to `strength` and correct the four prose lines.

**4. P2 — two of nine doors bypass `relate_checked`, and three docs claim every surface inherits the guard through it.**
Classification: inconsistency / design gap. Evidence: `databases/sqlite/src/lib.rs:931-933` and `libraries/ruby/src/lib.rs:213` call `engine.relate_opts(...)` directly; the contract's guard lives in `relate_checked` (`contract/src/lib.rs:957-981`) and in the trait's default `relate` (`contract/src/lib.rs:1834-1837`). The stand-in's own `relate_opts` repeats the guard (`standin/src/lib.rs:527`), so behaviour is correct and tested today (`databases/sqlite/tests/tvf_suite.py:226-242`, `libraries/ruby/tests/test_surface.rb:189-194`). The claims are wrong for those two: `databases/duckdb/src/relate.rs:17-18`, `libraries/typescript/addon/src/lib.rs:244-246`, `databases/duckdb/NOTES.md:68`.
Smallest fix: call `relate_checked` in both doors, or state that the guard's home is the engine.

**5. P2 — the C header's JSON door promises a numeric code that does not exist, and the two new functions' docs say `-1`.**
Classification: confirmed docs-vs-code inconsistency, already filed and unfixed (`libraries/c/NOTES.md:39-44`, `:246-253`). Evidence: `contract/include/thinkthen.h` — `thinkthen_call` "Returns NULL on failure, with the code as the return of the next `thinkthen_error_message`", and `thinkthen_recognize`/`thinkthen_relate` "Returns 0 on success and -1 on failure"; the code returns `code_of(kind)` 1..6 (`libraries/c/src/lib.rs:96-111`), and the exported symbols (`libraries/c/NOTES.md:148-159`) carry no `thinkthen_error_code`.
Smallest fix: one corrected sentence per function, or add the accessor.

**6. P2 — `databases/duckdb/extension-ci-tools` is an unversioned nested clone the build path needs.**
Classification: risk / repo hygiene. Evidence: the directory contains `.git`; `databases/duckdb/Makefile:17-18` includes its makefiles and `databases/duckdb/package.sh:39` runs its metadata script; `databases/duckdb/.gitignore` does not list it, and there is no `.gitmodules` anywhere in the tree (checked). Nothing pins its commit, and `git add -A` in a shared worktree would sweep it as a gitlink — the same accident already recorded once (`libraries/c/NOTES.md:292-302`).
Smallest fix: add `extension-ci-tools/` to `databases/duckdb/.gitignore` and record the clone command and commit in NOTES, or register a submodule.

**7. P2 — the validator never checks case-id uniqueness.**
Classification: design gap. Evidence: `conformance/tools/validate_conformance.py` checks only `case_count == len(cases)`; there is no id-uniqueness or id-shape check anywhere in the file. The 72 ids are unique and sequential today (verified by reading every `"id"` line). The repo's own review lessons already record "Duplicate case ids were silently counted twice by four of five metric recipes".
Smallest fix: `check(len({c['id'] for c in data['cases']}) == len(data['cases']), "case ids unique")`.

**8. P2 — two bench claims mean less than they read.**
Classification: inconsistency / risk. Evidence: `libraries/python/tests/bench_width_polars.py` (and `bench_width_pandas.py`) assert a wall-time spread `< 0.05` over a run whose floor is the 300 ms stub — the honest evidence is the stub's `requests`/`max_in_flight`, which the bench does assert; the "0.016 percent apart" headline is largely a statement about the stub. `libraries/python/tests/bench_cost_pandas.py` runs the four containers once, in one fixed order, so `libraries/python/NOTES.md:187`'s "three passes in two orders… the differences move with run order" is not reproducible from the committed script.
Smallest fix: loop the containers twice with the second order reversed in `bench_cost_pandas.py`, and say in the width benches that equality is read from the stub counters.

**9. P2 — the branch is behind main, and both trees own `conformance/`.**
Classification: risk at the merge gate. Evidence: the branch base is `b11a2b0`; main (`db19349`) owns `conformance/cases.json` plus `crates/thinkthen-core/tests/conformance.rs:18` (`include_str!("../../../conformance/cases.json")`) and records `0047`–`0052+`, while the branch owns `conformance/conformance.json` with a different schema and its own validator; the branch's `crates/thinkthen-core/tests/` does not exist. Every branch citation to `sdlc/issues/2026-09-21-update-for-the-library-team-recognize-and-relate.md`, `sdlc/planning/recognize-design.md`, `relate-design.md`, and `2026-09-21-one-rule-for-every-number-the-tool-prints.md` resolves **on main only** (I read them there), so inside the branch snapshot those references are dangling. After a merge both conformance files exist, which falsifies `SURFACES.md`'s "The one conformance file every surface reads".
Smallest fix: on the merge, pick the one conformance file, re-point or retire the core test if the branch's file wins, and run the root ladder on the merge result — not on the branch alone.

## Lens premises I had to correct

- **The swept-file incident was not in `libraries/python`.** It was the C lane's `git add -A` in commit `692ccbf`, sweeping six of the **R** lane's in-flight files (`libraries/c/NOTES.md:292-302`, `libraries/r/NOTES.md:170-173`). I checked both aftermaths: `libraries/python/src/lib.rs` has exactly one definition per registered function, every file `libraries/python/check.sh` names exists, and the R lane's six files are present with its later fixes landed as their own commits (`ac10fe5`, `21abc77`, `a0c05d7`, `fc64f0a`, `5ea93ff`). No duplicated or lost work found.
- **Untracked junk:** nothing suspicious is unignored. `libraries/python/.pytest_cache/` self-ignores (pytest writes `*` inside it), `databases/sqlite/thinkthen.so`, `.runtimes/`, `dist/` and every `target/` are ignored. The one real hygiene item is finding 6.

## What is already strong

- **Offsets are right in all four worlds.** The contract counts code points (`contract/src/lib.rs:849-872`). JavaScript converts once with a correct code-point→UTF-16 scan (`libraries/typescript/index.js:360-372`) and pins 11/21; Rust and C convert to bytes (`libraries/rust/src/lib.rs:325-358`) with the emoji case asserting `(10,20) → (14,24)` and slicing the name (`libraries/c/tests/door.rs:391-411`); Python, R, SQLite, DuckDB and PostgreSQL use their own code-point identity with slice assertions per host (`libraries/python/tests/test_recognize_relate.py:55-62`, `libraries/r/recognize_check.R:66-72`, `databases/sqlite/NOTES.md:74-78`, `databases/duckdb/tools/conformance.py:171-176`, `databases/postgresql/NOTES.md:89`). The validator re-slices every recognize case independently (`validate_conformance.py:259-261`).
- **The 255 refusal is enforced and tested at every door that carries `relate`**, and the database doors bound the refusal read at 256 rows (`databases/postgresql/src/lib.rs:442`, `databases/sqlite/src/lib.rs:495-503`) rather than draining the table.
- **The validator proves replay, not schema**: it recomputes digests, decide/choose/tag/score/filter/rank/find answers from the recorded exchanges and the recognize/relate expectations from the replay table (`validate_conformance.py:160-200`, `330-360`), and it refuses a bare `from`/`to` or `confidence` in an expectation.
- **The generator really gates**: `--check` writes nothing and exits 1 on staleness (`scripts/generate_functions.py:118-133`), and `scripts/check_surfaces.sh:36` runs it under `set -e`. The committed `generated.rs` matches all 15 `functions.toml` rows, and `index.mjs` is generated from the same table.
- **The host-facing vocabulary is clean in code and types**: no surface, database, type file, or the C header emits `number`, `from`, `to`, or `confidence`; the stand-in asserts the door JSON carries none of them (`standin/tests/recognize_replay.rs:148-153`, `220-225`). The recorded number the stand-in returns is genuinely the computed strength (`experiments/225-recognize-harvest-package/tools/pipeline.py:230-255`), so the definition holds.
- **Packaging honesty**: all three database package scripts trap and report leftovers; the R tarball's junk fix and the Rust path-version fix carry before/after numbers; the macOS table states failures plainly; the C and Python macOS artifacts exist on disk (`libraries/python/target/wheels/thinkthen-0.0.1-cp310-abi3-macosx_11_0_arm64.whl`, `libraries/c/target/aarch64-apple-darwin/`).
- **History hygiene**: linear, no force-push, the four probe commits kept as the measurement record and reverted in `121abdd`, the `git add -A` incident recorded by both lanes.

## Roadmap, in order

1. Wire the surfaces' gate into a rung (finding 1); the offline half costs nothing.
2. Settle the question file's `source`/`target` spelling and make one edit (finding 2).
3. Fix the generator's `number` key and the four stale prose lines (finding 3) — the tree is one generator run from failing its own validator.
4. Point SQLite and Ruby at `relate_checked`, or name the guard's home (finding 4).
5. Add the validator's id-uniqueness check; correct the C header's two doc lines or add `thinkthen_error_code` (findings 5, 7).
6. Merge hygiene: choose the one conformance file, re-point the core test, ignore or pin `extension-ci-tools`, and run the root ladder on the merge result (findings 6, 9).
7. Bench honesty: record the reversed second order; attribute the width equality to the stub counters (finding 8).
8. Still owed to the Mac: TypeScript and Ruby macOS artifacts, the three databases' macOS artifacts, the PostgreSQL glibc pin, and the R handler case — all recorded as unchecked, none of them re-verified here.

## Open questions

- Does the contract owner rule the question file's spelling, or does the surfaces team patch the parser? Different answers change four NOTES, the deck, and the conformance builder.
- Does `scripts/check_surfaces.sh` belong in a rung (slower, toolchain-heavy) or beside the ladder as a documented manual gate? The two offline checks can go in a rung either way.
- Is main's `conformance/cases.json` superseded by the branch's `conformance/conformance.json`, or are they two artifacts with two consumers?

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Nine findings with file and line evidence and classifications, each with a smallest remediation: surfaces gate off the ladder (sdlc/scripts/lint,test,spec have no check_surfaces reference), question-file from/to vs the ruled source/target (contract/src/lib.rs:773-783 vs :887-895, postgresql/src/lib.rs:175-206), stale strength rename (contract/src/lib.rs:851-860, standin/src/replay.rs:61-63, conformance/tools/build_recognize_cases.py:58-69), relate_checked bypass (databases/sqlite/src/lib.rs:931-933, libraries/ruby/src/lib.rs:213), C header code promise (contract/include/thinkthen.h vs libraries/c/src/lib.rs:96-111), unversioned extension-ci-tools clone, validator id-uniqueness gap, bench claims, branch-behind-main conformance collision."
    }
  ],
  "changedFiles": [],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "git -C the thinkthen-surfaces worktree status --porcelain=v1 --branch",
      "result": "not-run",
      "summary": "No shell tool in this session; untracked/uncommitted state unverified. Read-only inspection found no unignored junk except databases/duckdb/extension-ci-tools (finding 6)."
    },
    {
      "command": "cd the thinkthen-surfaces worktree && python3 scripts/generate_functions.py --check && (cd conformance && python3 tools/validate_conformance.py conformance.json)",
      "result": "not-run",
      "summary": "Static read shows both should pass on the committed tree; must be run to attest."
    },
    {
      "command": "cd the thinkthen-surfaces worktree && ENGINE_NULL=1 bash scripts/check_surfaces.sh",
      "result": "not-run",
      "summary": "The branch's own gate; the wire half needs the loopback stub on the per-surface ports."
    },
    {
      "command": "cd the thinkthen-surfaces worktree && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec",
      "result": "not-run",
      "summary": "The repository ladder; as configured it never touches the surfaces (finding 1)."
    },
    {
      "command": "docker ps -a --filter name=pkg211 --filter name=dbpkg211 --format '{{.Names}} {{.Status}}'",
      "result": "not-run",
      "summary": "Leftover containers unverified; the package scripts trap and print a leftover count."
    }
  ],
  "validationOutput": [
    "Static verification only. Confirmed by reading: conformance/conformance.json holds case_count 72 with ids 01..72 unique and sequential; libraries/python/src/generated.rs matches all 15 functions.toml rows; the stand-in's replay maps recorded confidence to Entity.strength (standin/src/replay.rs:127-135); C's fail() returns code_of(kind) 1..6 (libraries/c/src/lib.rs:96-111); sdlc/scripts/{lint,test,spec} contain no reference to check_surfaces.sh, contract, standin, libraries or databases; sdlc/ratchet.json counts crates/**/*.rs only."
  ],
  "residualRisks": [
    "Nothing was executed: no test suite, validator, generator --check, git status, or docker state was observed by command. Every 'passes' statement above is a static reading.",
    "The branch snapshot cannot resolve its own citations to sdlc/issues/2026-09-21-update-for-the-library-team-recognize-and-relate.md and sdlc/planning/recognize-design.md; they exist on main (db19349) only.",
    "Root-owned artifacts under libraries/ruby/target from the root builder container are recorded in NOTES-packaging and can break a host rebuild without a fresh CARGO_TARGET_DIR.",
    "PostgreSQL's artifact is not glibc-pinned (recorded, unchecked); TypeScript, Ruby and the three databases have no macOS artifact from this box.",
    "If the surfaces' gate stays off the ladder after the merge, surface rot and stale generated lists will pass every rung."
  ],
  "noStagedFiles": true,
  "diffSummary": "Read-only review of the surfaces branch at 0801a32; no files changed, created, staged or committed by this session.",
  "reviewFindings": [
    "blocker: sdlc/scripts/{lint,test,spec} - the entire surfaces gate (scripts/check_surfaces.sh, the conformance validator, the generator --check, fmt/clippy/deny over contract/standin/libraries/databases) runs from no rung, so a rotted surface check fails nothing; add one rung that runs the two offline checks now and check_surfaces.sh when toolchains exist",
    "blocker: contract/src/lib.rs:773-783 - the question file's relation ends are from/to while the ruling and the contract doc say source/target on every surface including the question file; eight of nine doors refuse the ruled spelling and PostgreSQL accepts both",
    "major: conformance/tools/build_recognize_cases.py:58-69 - the documented generator still writes the interim `number` key, which would fail the tree's own validator (validate_conformance.py:262-266) and revert the settled strength rename; four prose leftovers cite `number` too",
    "major: databases/sqlite/src/lib.rs:931-933 and libraries/ruby/src/lib.rs:213 - two doors call relate_opts directly instead of relate_checked, and three docs claim every surface inherits the 255 guard through relate_checked",
    "minor: contract/include/thinkthen.h - thinkthen_call promises a numeric code the door never returns and the two new functions' docs say -1 while the code returns the kind code 1..6 (already filed in libraries/c/NOTES.md:39-44, :246-253)",
    "minor: databases/duckdb/extension-ci-tools - an unversioned nested git clone the Makefile and package.sh require, not ignored and not a submodule, sweepable by git add -A",
    "minor: conformance/tools/validate_conformance.py - no case-id uniqueness or id-shape check; ids are unique today but the file would not catch a duplicate",
    "minor: libraries/python/tests/bench_width_polars.py and bench_cost_pandas.py - the wall-time equality is set by the 300 ms stub, and the order-sensitivity claim in libraries/python/NOTES.md:187 is not reproducible from the committed script",
    "minor: conformance/ - the branch and main each own a conformance file (cases.json consumed by crates/thinkthen-core/tests/conformance.rs:18 on main; conformance.json with the validator on the branch), so the merge must choose one and re-run the root ladder"
  ],
  "manualNotes": "Nothing was run: this session has no shell tool, so the report is static evidence plus one grep-level proof of absence. Three lens premises were corrected: the swept-files incident was the C lane's git add -A sweeping the R lane (not libraries/python, which shows no duplicated or lost work); untracked junk is limited to the extension-ci-tools clone (the python .pytest_cache self-ignores); and the stand-in's strength value is genuinely the computed number, so that claim holds. The two blockers are cheap: one rung and one parser spelling."
}
```