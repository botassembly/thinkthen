# Area 11: Settings across tiers and surfaces

Commit `58014776c`. Reviewer: Sonnet 5.5, fresh, read only. Rubric: `README.md` in this folder.

## Scope

Settings are the values a caller chooses. One table lists each of them on every surface, the typed, environment, question-file, configuration and default tiers resolve them, and a script keeps the table true to the help, the code and the keys.

All paths below are under `crates/thinkthen/src/` unless they start with `crates/`, `specification/`, `sdlc/`, `conformance/` or `libraries/`.

| Kind | Paths (nonblank lines) |
| --- | --- |
| Code | `core/settings.rs` (476), `public/settings.rs` (474) with `public/settings/{budgets,environment,prices,server}.rs` (189), `public/options.rs` (442) with `options/observer.rs` (71), `cli/args.rs` (496) with `cli/args/*.rs` (771): about 2,900 lines of Rust. `sdlc/scripts/settings` (314, Python). The CLI tier code in `cli/edge.rs` (486) and `cli/judge.rs` is counted under area 1 |
| Tests | About 38 Rust tests: `core/settings.rs` (3), `public/options/tests.rs` (2), `crates/thinkthen/tests/backend/settings_cases.rs` (2), `question_file/overrides.rs` (6), `crates/thinkthen/tests/library/public_env.rs` and `public_env/` (25). Nine shared cases in `conformance/settings.json`, each run by ten runners (command, Rust, Python, TypeScript, Ruby, R, C, DuckDB, PostgreSQL, SQLite) |
| Contract | `specification/settings.md` (62 rows, 18 columns), `specification/question-file.md` "One table for every setting" (:79), "Precedence" (:97), "Where each setting came from" (:117); ADRs 0105, 0033, 0048, 0051, 0085, 0032, 0041, 0104, 0114 |

## Complexity: 4 of 5

| Driver | Score | Measured fact |
| --- | ---: | --- |
| Code size | 4 | About 2,900 nonblank lines of Rust across 16 files, plus a 314-line script |
| States and concurrency | 2 | Sequential. Five tiers and two split orders (engine above environment for backend, address and model) resolved at the edge, then passed inward as typed values |
| Rules and refusals | 5 | Over 50. 62 table rows each with allowed values, 13 `SettingsError` variants (`core/settings.rs:18-56`), 12 key rules in `engine_settings` (`:333-360`), and more than seven documented precedence orders (`specification/settings.md:11-28`) |
| Surfaces touched | 5 | All 22. The table has 10 surface columns, and the rows reach from 28 (R) to 49 (command) of 62 |
| Settings | 5 | 62 rows, the whole table |
| Contract weight | 5 | Over 12. Three spec pages with sections, plus at least nine ADRs |
| Churn and debt | 5 | 133 commits on the code paths in 7 days, 109 on `settings.md`, 14 on the script. One post-landing regression in the window: audit and diff lost the batch setting (`e877e43df`, closed issue `2026-09-30-audit-and-diff-lost-the-batch-setting.md`) |

Mean 4.4, rounded to 4.

## Grades

| Dimension | Grade | Evidence |
| --- | :---: | --- |
| Quality | B | The table is checked by a script: every long flag, every `THINKTHEN_` name and every question-file key needs a row, and a stale row fails (`sdlc/scripts/settings:1-8`). The `spec` rung runs it and its self-test (`sdlc/scripts/spec:30-31`), and the `gate` workflow runs `spec` (`.github/workflows/gate.yml:98`). Primary paths match. Drift: `specification/settings.md:13` says an engine-level library setting "sits in the environment tier", and `:20-21` then put the engine setting above the environment for backend, address and model. The library takes any nonzero `Duration` for `timeout` (`public/settings.rs:301-306`), while the command bounds it to 1 through 86,400 because the client can overflow (`cli/mod.rs:161-168`). The table states the difference (`settings.md:77`), but the reason for the bound does not reach the library |
| Reliability | B | The nine `conformance/settings.json` cases count sends per setting on ten surfaces. They cover timeout, retries, profile, model, record, replay, request limit, cache and request size, and leave out throttle, batch, address, prices and the two totals (`conformance/settings.json`, `settings` fields). Batch tiers have eight tests (`tests/library/public_env/batch.rs`) and six overrides tests (`tests/backend/question_file/overrides.rs`). One post-landing regression in the window: audit and diff lost the batch setting (`e877e43df`), since fixed. Heavy churn (133 commits) caps the grade at B |
| Maintainability | C | The batch rule is written in five places with the same sentence: `cli/judge.rs:61-78`, `cli/annotate.rs:180-210`, `public/settings.rs:372`, `public/native_batch.rs:68` and `core/settings.rs:37`. `THINKTHEN_MAX_REQUEST_BYTES` is read and parsed twice (`cli/edge.rs:188-199`, `public/settings/environment.rs:75-84`). The engine JSON grammar re-implements each setter's rule (`core/settings.rs:333-360`), and each of the nine hosts has its own copy named in `sdlc/scripts/settings:59-66`. Six files sit within 26 lines of the 500 cap: `tests/library/public_env.rs` (500), `cli/args.rs` (496), `cli/edge.rs` (486), `config.rs` (478), `core/settings.rs` (476), `public/settings.rs` (474) |

## Strengths

- One table is the contract, and a script enforces it from the `spec` rung with a self-test that plants each fault (`sdlc/scripts/settings:1-8`, `sdlc/scripts/spec:30`).
- Closed, shared test cases run on ten surfaces and count listener sends, so a setting that does nothing shows up (`conformance/README.md:5`, `conformance/settings.json`).
- Portable settings have a pure, closed grammar that keeps duplicate names and refuses a key that does not belong to the verb (`core/settings.rs:18-56`, `:317-323`).
- The table is honest about gaps: cells read "not on this surface" with a reason, including ADR 0114 build slice 2 for eight surfaces (`specification/settings.md:71-74`).
- No lint suppression outside test files, and each has a reason (`cli/args/debug.rs:67`).

## Cleanup

1. **Resolve the batch setting in one function.** Where: `cli/judge.rs:61-78`, `cli/annotate.rs:180-210`, `public/settings.rs:372`, `public/native_batch.rs:68`, `core/settings.rs:37`. Why: five copies of the same tier order and sentence. The closed issue about audit and diff losing the batch setting is the same family. One resolver in `core/batch.rs` that takes the tiers as typed values would settle it. Size: M. Blocks 0.1: no.
2. **Parse `THINKTHEN_MAX_REQUEST_BYTES` once.** Where: `cli/edge.rs:188-199`, `public/settings/environment.rs:75-84`. Why: the same digit check and sentence, written twice. Size: S. Blocks 0.1: no.
3. **Bound the library timeout, or show it is safe.** Where: `public/settings.rs:301-306`, `core/settings.rs:346` (`"timeout" => number().is_some()`), `engine/http.rs:153`. Why: the command refuses more than a day because the client "adds the timeout to the clock and a larger number can overflow there" (`cli/mod.rs:161`). The library and the JSON settings accept any nonzero value. Whether a huge `Duration` panics is unconfirmed and untested. Add a case with the largest value, then either cap at 86,400 or pin the result. Size: S. Blocks 0.1: no until shown to panic, and yes if it does.
4. **Fix the precedence paragraph.** Where: `specification/settings.md:13`, `:20-21`. Why: `:13` puts engine-level settings in the environment tier, and two bullets below it put them above. State the split once, where it is first named. Size: S. Blocks 0.1: no.
5. **Extend the shared settings cases.** Where: `conformance/settings.json` (9 cases). Why: throttle, batch, address, prices and the two totals have no shared case on ten surfaces, so a binding can drop one without a failing check. Size: M. Blocks 0.1: no.
6. **Generate the engine JSON grammar from one rule list.** Where: `core/settings.rs:317-372`, `public/settings.rs:215-311`, the nine `ENGINE_SOURCES` in `sdlc/scripts/settings:59-66`. Why: the setters and the JSON grammar each state the same ranges, and the script pins the cells but not the rules. Size: L. Blocks 0.1: no.
7. **Split the files at the cap.** Where: `tests/library/public_env.rs` (500), `cli/args.rs` (496), `cli/edge.rs` (486). Why: the next setting added to each fails the cap. Size: S each. Blocks 0.1: no.
8. **Reach the named-backend settings on other surfaces.** Where: `specification/settings.md:71-73`. Why: Backend, Named backends and Backend key are command and Rust only. See area 17, item 3. Size: L. Blocks 0.1: no.

## Confidence: medium

What was read: `specification/settings.md` (all sections outside the table, and the address, backend, model, timeout and plan rows), `question-file.md` "Precedence", the header and structure of `sdlc/scripts/settings`, `core/settings.rs` (first 120 lines and the engine grammar), `public/settings.rs` (setters and first 80 lines), `public/settings/environment.rs`, the batch and request-size resolvers in `cli/judge.rs`, `cli/annotate.rs` and `cli/edge.rs`, `conformance/settings.json` and its README, and the file line counts.

Not checked: no test was run. `cli/args.rs` with `args/*.rs`, `public/options.rs` and the ten runners' own settings checks were counted but not read. The 62 rows were counted, and only about ten were read in full, so row-level drift between the table and the code is unchecked beyond what the script proves. The script was not run. The commit count for area 11 includes 109 edits to `settings.md`, most of them table rows, so the churn score reads higher than the code alone would. The huge-timeout panic is unconfirmed.
