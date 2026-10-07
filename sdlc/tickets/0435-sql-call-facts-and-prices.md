# 0435: Return isolated SQL call facts and caller-priced cost

Status: in progress. Complete SQL family adoption in warm lane 1.

Milestone: 0.2

Owner: builder.
Ticket review: ACCEPT, 2026-10-06; fresh read-only review. Conflicting image deferrals corrected where found.

## Outcome

Each SQL invocation returns final facts and optional exact caller-priced cost from the same judgment. DuckDB, SQLite and PostgreSQL agree.

## Evidence

- Starts from: Current main c64b71859; 0405 audit is historical and predates 0377/0420. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 4, 6 and 9.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Add facts to the existing details result from its owning Rust Call, including call_id. Preserve ordinary scalar/table outputs and cumulative usage APIs. Adopt the engine-scoped price pair and shared exact arithmetic from 0300.
- Proof: A details call sends/evaluates once; concurrent invocations retain isolated counts. Packed rows repeat one invocation’s facts and call_id, so deduplicating that ID counts its sends once. Never subtract cumulative usage snapshots or sum historical per-row usage shares. Test exact rounding, incomplete usage, overflow, cache/replay zero spend and started-failure facts through the reviewed native SQL error route.
- Defers: Proxy service/screens, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.

## Dependencies and ownership

0300 supplies shared price settings/arithmetic; 0442 supplies facts metadata. This ticket owns SQL adoption and updates 0423 guidance to supersede its earlier per-call-facts deferral. A statement containing several calls has several invocation facts; statement-wide accounting is outside this contract.

## Design notes

Specify an accessible started-failure facts route for each SQL host before code. If a native error cannot carry an object, use a bounded reviewed additive failure-result route rather than a cumulative counter. No second evaluation. Caller prices are estimates, never provider bills.

## Full-result answer identity

0450 IDs belong on SQL full details/observation results from the same native evaluation. Ordinary scalar values retain compatibility. Expose per-logical-result IDs independently from invocation call_id; repeated packed invocation facts do not collapse distinct logical answers. Coordinate image overloads in 0452 without a second judgment.

## Complete family adoption (2026-10-07)

Risk: High. Native invocation ownership, public facts and PostgreSQL file privileges require one fresh High review of the whole family. This branch starts at published main 0179d16ad and owns databases and their actual SQL consumers, documentation and packages. Root owns the single landing record and full landing gates. Accepted 0434/0417/0452 slices, ordinary scalar/table signatures, SQLite's wrong-model correction, backend selection, settings and usage remain supported. No native/C/shared fixture changes or new dependency are authorized.

Concrete additive route, fixed before code: each of the ten functions exposes `thinkthen_FUNCTION_complete(question TEXT, inputs TEXT, settings TEXT := NULL)`. DuckDB and SQLite return JSON text; PostgreSQL returns json. A complete call returns the formal native `Call::complete()` envelope, plus the actual ordered native observer snapshots. Admission and started failures return `Error::complete()` with those same snapshots. No SQL error handler reruns a judgment, subtracts usage counters, fabricates identity, or retains a process-global last error. Ordinary SQL functions continue raising their existing errors. This bounded failure-result route is explicit at the named complete door and covers all six native error kinds. Native facts are absent before a call starts and final after its workers join. SQL interrupts retain the host cancellation behavior.

Inputs are explicit ordered record descriptors: `records` contains `text` or `json`, optional per-record `context`, `options`, `images` and `source` (file and paired line coordinates). Images contain explicit compressed bytes as integer arrays and media. DuckDB/SQLite additionally admit explicitly requested native file reads through `files` (paths and native unit/window). PostgreSQL admits only client-read descriptors and refuses server evidence/image readers; question references continue through its accepted confined privileged loader. Native RecordReading controls in `reading` select fields/context/options; `incremental` selects the existing native fallible record iterator. This is SQL edge serialization of existing native controls, not another question/declaration/image parser. NULL question/inputs yields NULL without sends; NULL settings uses defaults. Empty record arrays use native empty-input semantics.

Settings retain the existing host engine selection. The complete door admits native per-call context/batch/deadline/attempt controls and literal reading overrides; engine-scoped caller prices use the existing exact native arithmetic. Cache, record and replay use the host's existing engine settings, never a SQL cache. Complete rank detects saved rank sets and delegates to native turns merging. Formal result JSON is checked using typed SQL field extraction at each actual installed consumer, reusing the existing shared fixture projector and counted owned loopback.

Ian can overturn these additive spellings and the bounded complete failure-result choice. Any material departure from accepted native or PostgreSQL contracts returns to root for review before dependent implementation. Completion requires every applicable shared case at each of the three real SQL doors (currently 247 each), metadata/answer identities, changed-reading cache zero calls, retained PostgreSQL privilege checks, and installed extension checks, after merging newly published main.

The bounded complete envelope also exposes typed `ordinals` for row association and `selection` for find; these use the native result getters because native canonical documents omit occurrence ordinals and find selection. Incremental failures retain completed canonical rows from that same batch, under `completed`, beside native terminal error facts. `cancelled` selects a native pre-cancelled token. Explicit named loading uses `@@NAME` (native load_named); ordinary `@reference` retains native reference precedence. These are additive complete-door controls only.

Engine settings adopt native `base_url`, `refresh_cache` and the price pair where needed. An explicit address is essential for counted fixtures with named backends: backend selection otherwise outranks the environment address. Tests pass only owned loopback addresses and fake keys. PostgreSQL exposes address and cache-refresh through administrator settings, retaining existing privilege boundaries.

Native prerequisite raised to root: PostgreSQL named/reference resolution needs a public native resolution operation that does not open content, or an authorized reader callback. Native load_named/load_reference currently opens the file itself and cannot preserve the accepted PostgreSQL descriptor privilege checks. Inline and explicitly confined question-file loading proceed independently; no bypass is permitted.

Implementation checkpoint: all ten SQL complete doors and shared native composition/observation serialization are implemented. Preliminary actual-consumer counts are SQLite 244/247 (remaining three corrected in focused checks) and DuckDB 241/247. PostgreSQL corpus execution is underway. Named questions remain refused at DuckDB pending the authorized resolver; PostgreSQL never bypasses its descriptor loader. This is not a completion claim. Shared host-neutral Rust code resides under SQLite and is included by the other SQL adapters. Source growth adds the complete API and real shared consumers while retaining ordinary compatibility code; no dependency or lockfile changes.

The explicit PostgreSQL client-reader workaround serializes an actual native reader failure as a terminal `read_error` record containing the native Error::complete envelope. Only pre-start Local/Usage failures without facts/IDs are admitted. The SQL native input iterator recreates that typed failure; eager admission returns it before starting, while incremental admission obtains any started facts from its own native batch. DuckDB authorized-reader failures use the same descriptor, and host admission failures return native pre-start error envelopes. `document` selects native annotation-document composition; `json_text` defers JSON-line admission until the owned iterator consumes it. These transport fields never supply facts, spend or identities. Root reviews this concrete bounded extension as part of the complete family contract.

Strict-envelope correction before implementation: the native complete schema forbids unevaluated root properties. The complete SQL return therefore contains `native` holding the untouched formal Call::complete or Error::complete document, plus separate `observations`, `ordinals`, `selection` and incremental `completed` supplements where applicable. This supersedes flattening supplements into the formal native envelope. Actual consumers validate `native` against the existing generated complete schema and extract its known fields with SQL types. No native/shared schema changes. Root reviews this concrete carrier spelling in the whole-family High review; Ian can overturn it.

Authorized-reader native API decision (2026-10-07): add `QuestionFileReference::named`, `reference` and `reference_in`, with `path`, `named_root` and content-only `parse(original_json, QuestionFileRole, parser)`. Re-export the existing pure `QuestionRole` as `QuestionFileRole`; retain its seven roles and ordinary loader behavior. Resolution opens no content. The selected absolute path preserves symlink spelling and `..`; named selection captures the checked canonical questions root. Parsing caps caller-read text, compares the original ordered authored name, checks the existing role classifier, then calls the supplied existing native grammar once and maps grammar failures to Local. Wrong roles and invalid explicit names remain Usage. A missing authored name stays absent. The value has private fields and redacted Debug. Existing private CLI/MCP entrypoints remain. PostgreSQL uses its configured relative lookup directory with `reference_in`, authorizes before resolution, and checks its opened descriptor against the captured named root before content reading. DuckDB keeps its session reader. These adapters adopt the API after the bounded native prerequisite; resolution grants no read permission. The existing whole-family High review covers this choice; Ian can overturn it. Source growth adds 358 nonblank Rust lines (157204 → 157562) for the shared selection/content boundary and focused public regressions; the existing loaders share the name comparison and selector instead of duplicating them.

### Added public declarations

```text
struct QuestionFileReference
enum QuestionFileRole
QuestionFileRole::Atomic
QuestionFileRole::Choose
QuestionFileRole::Rank
QuestionFileRole::Set
QuestionFileRole::Find
QuestionFileRole::Recognize
QuestionFileRole::Relate
fn QuestionFileReference::named(&str) -> Result<QuestionFileReference, Error>
fn QuestionFileReference::reference(&str) -> Result<QuestionFileReference, Error>
fn QuestionFileReference::reference_in(&str, &Path) -> Result<QuestionFileReference, Error>
fn QuestionFileReference::path(&self) -> &Path
fn QuestionFileReference::named_root(&self) -> Option<&Path>
fn QuestionFileReference::parse<T>(&self, &str, QuestionFileRole, impl FnOnce(&str) -> Result<T, Error>) -> Result<T, Error>
```
