# Public library API gaps

Status: open for items 1, 2, 3, 6, 7 and 9. Shortened 2026-09-30. Items 4 (ticket 0148), 5 (ticket 0150) and 8 (ticket 0136) are settled, and git history holds their text. No ticket owns the rest. Ticket 0304 slice 3b moves the SQL hosts onto the one batching path and touches the code of items 1 and 9, so it may settle them.

Each item names a place where the public API (ticket 0084, `crates/thinkthen/src/public/`) falls short of what the command, a spec, or a ruling promises. Each gap makes a binding copy engine code or drift from the other bindings. Ian's ruling of 2026-09-25 applies throughout: no setting that does nothing.

## 1. Engine counters count per engine, not per process

`Engine::usage()` documents "This process's totals" (`public/engine.rs`), but `EngineBuilder::build` gives each engine its own in-memory `Counters` (`public/settings.rs`). A second engine in the same process starts from zero. The PostgreSQL binding sums its engines' counters itself (`databases/postgresql/src/call.rs`, `engines()`). ADR 0113 now writes every engine's sends to the durable usage totals, so `status` sees them. The in-memory `usage()` value is still per engine.

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

Rust ships `Description` and `DescriptionBuilder`. TypeScript's `index.d.ts` now declares `Description` as a string or a JSON object per option, label and level. Python still has no documented `Description` dataclass. No binding has a typed annotate class, and no cross-surface test compares the digests of one description object sent from Rust, Python, TypeScript and a question file. Ian's ruling 7 of 2026-09-29 defers type-review recommendations beyond ADR 0112, which may cover the typed annotate class.

The product-side asks of 2026-09-22, which Ian can overturn: a string stays valid everywhere; the library never reorders or normalizes the object; the same object gives the same digest on every surface. A typed annotate class serializes to the same JSON as the question file, so the cache is shared. Python follows the Pydantic AI mapping: `bool` is decide, `Literal` or `Enum` is choose, an ordered `IntEnum` is score, `list[Literal]` is tag, a nested class is an annotate form. Rust uses builders with no derive macro.

Done when Python accepts a typed description, the typed annotate form is built or ruled out, and the cross-surface digest test passes.

## 9. Every SQL surface matches both arms of `LoadedQuestion` by hand

Only `Question` and `BandedQuestion` implement `DecisionQuestion`. SQLite (`many.rs`, `scalars/plan.rs`) and PostgreSQL (`lib.rs`, `keyed.rs`) repeat a `match` on both arms of `LoadedQuestion`.

Done when the public API says whether a loaded question is asked directly, and the surfaces follow it.
