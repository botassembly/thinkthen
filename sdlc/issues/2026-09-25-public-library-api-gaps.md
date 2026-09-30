# Public library API gaps, and the SQL host copies they cause

Status: open for items 1, 2, 3, 6, 7, 9 and 10. Shortened 2026-09-30. Items 4 (ticket 0148), 5 (ticket 0150) and 8 (ticket 0136) are settled, and git history holds their text. Rechecked on main `88649ec0d` after 0304 slices 3b and 3e: items 1, 2, 3 and 9 still stand. Item 10 merged in on 2026-09-30 from the hand-built plan JSON issue, now in `closed/`.

Ticket: 0347, for items 1, 2, 3, 9 and 10, ready since 2026-09-30; batch order in `../planning/issue-priorities-2026-09-30.md`.

Owner: one SQL host ticket for items 1, 2, 3, 9 and 10 after 0304 slice 4, which rewrites the relate path item 2 reaches; item 10 alone can go first as a Quick Fix. Items 6 and 7: a future ticket after 0.1.

Kind: debt

Pay when: before 0.1 for items 1, 2, 3, 9 and 10; item 10 sooner, at the next change to either host's `thinkthen_plan`. Items 6 and 7 are features, paid when a user asks after 0.1.

Debt: 018

Severity: medium

Keeping it leaves the SQL hosts holding copies of engine code, so a new plan member, rule form or refusal can reach the C door and miss SQL.

Each item names a place where the public API (ticket 0084, `crates/thinkthen/src/public/`) falls short of what the command, a spec, or a ruling promises. Each gap makes a binding copy engine code or drift from the other bindings. Ian's ruling of 2026-09-25 applies throughout: no setting that does nothing.

## 1. Engine counters count per engine, not per process

`Engine::usage()` documents "This process's totals" (`public/engine.rs`), but `EngineBuilder::build` gives each engine its own in-memory `Counters` (`public/settings.rs`). A second engine in the same process starts from zero. The PostgreSQL binding sums its engines' counters itself (`databases/postgresql/src/call.rs`, `engines()`). Under ADR 0113, engines built from the environment and the SQL hosts write their sends to the durable usage totals, so `status` sees them; `EngineBuilder::new()` still names no usage folder. The in-memory `usage()` value is still per engine.

Done when two engines built in one process report one shared total, or the `usage` doc and ticket 0084 say per engine and every SQL surface sums through one shared helper.

## 2. No public parser for inline relate rules

The command parses `NAME` or `NAME=SOURCE:TARGET` in the private `inline_rule` (`core/relate_file.rs`). PostgreSQL's `thinkthen_relate(query, rules text[])` copies it (`databases/postgresql/src/relate.rs`, `inline_rule`).

Done when PostgreSQL calls a public parser such as `RelationRule::parse_inline(text, either)` and drops its copy.

## 3. `thinkthen::Error` has no public constructor

`Error::of`, `usage`, `local`, `defect` and `cancelled` are all `pub(crate)` (`public/error.rs`). PostgreSQL keeps its own `Refusal` type for bare text, a spent request total, an over-cap file and a cancelled wait (`databases/postgresql/src/call.rs`).

Done when PostgreSQL raises its refusals as `thinkthen::Error` and `Refusal` is gone, or a written pattern replaces it.

## 6. The engine reuse test and a library warm call

`sdlc/planning/quality-plan.md` has the row "Engine reuse: two calls in one process, module-level and object API both, open one connection on a counting stub; each database extension does the same after a warm call." No such test exists for the library free functions, one engine object, or a database after `thinkthen_warm`. The libraries have no explicit warm call. The product-side reading of 2026-09-22, which Ian can overturn, keeps the test at low priority and says construction never touches the wire.

Done when the quality-plan row passes on every surface, and the library contract says whether a library `warm` exists.

## 7. Typed descriptions and typed annotate forms outside Rust

Rust ships `Description` and `DescriptionBuilder`. TypeScript's `index.d.ts` now declares `Description` as a string or a JSON object per option, label and level. Python still has no documented `Description` dataclass, and its recognize builder takes text descriptions only (`libraries/python/src/asked.rs`, `_build`). No binding has a typed annotate class, and no cross-surface test compares the digests of one description object sent from Rust, Python, TypeScript and a question file. Ian's ruling 7 of 2026-09-29 defers type-review recommendations beyond ADR 0112, which may cover the typed annotate class.

The product-side asks of 2026-09-22, which Ian can overturn: a string stays valid everywhere; the library never reorders or normalizes the object; the same object gives the same digest on every surface. A typed annotate class serializes to the same JSON as the question file, so the cache is shared. Python follows the Pydantic AI mapping: `bool` is decide, `Literal` or `Enum` is choose, an ordered `IntEnum` is score, `list[Literal]` is tag, a nested class is an annotate form. Rust uses builders with no derive macro.

Done when Python accepts a typed description, including for recognize kinds, the typed annotate form is built or ruled out, and the cross-surface digest test passes.

## 9. Every SQL surface matches both arms of `LoadedQuestion` by hand

Only `Question` and `BandedQuestion` implement `DecisionQuestion`. Twelve SQL host files repeat a `match` on both arms of `LoadedQuestion`: DuckDB's `bridge/src/ffi/{plan,portable,portable_aux,portable_many,scalar}/ffi.rs` and `bridge/src/warm/ffi.rs`, SQLite's `many.rs`, `scalars.rs` and `scalars/plan.rs`, and PostgreSQL's `forms.rs`, `keyed.rs` and `lib.rs`.

Done when the public API says whether a loaded question is asked directly, and the surfaces follow it.

## 10. SQLite and PostgreSQL build their plan JSON by hand

Found while building ticket 0314 slice 4a on main at `6c29039b9`. Slice 4a gave `PlanEstimate` a `Serialize` impl, and the generated result schema now holds its `plan` definition. The C door writes it with `serde_json::to_string`. SQLite (`databases/sqlite/src/scalars/plan.rs:77-91`) and PostgreSQL (`databases/postgresql/src/keyed.rs:184-199`) still build the same six members with `json!`. Both already print `first_body_utf8`, the member name the crate took, so switching each to `serde_json::to_string(&estimate)` changes no byte. DuckDB returns a native `STRUCT` with `first_body` and stays as it is. The SQL hosts are outside ticket 0314's port pass (ADR 0112 section 6). Lane 1 was editing the SQL hosts' limits (0304 slice 3e) when this was found, so slice 4a left them alone. The same two files hold item 9's match.

Done when both hosts serialize `PlanEstimate` and their plan checks pass byte for byte.
