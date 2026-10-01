# Public library API gaps

Status: open for items 6 and 7, both after 0.1. Every pre-0.1 item has landed: items 1, 2, 3, 9 and 10 by ticket 0347, item 4 by ticket 0148, item 5 by ticket 0150 and item 8 by ticket 0136. Git history holds their text.

Resolution: items 1, 2, 3, 9 and 10 paid by ticket 0347. `Engine::usage` says it counts one engine and its clones, and PostgreSQL and DuckDB add their engines with `Sum` for `Counters`. PostgreSQL parses inline relate rules with `RelationRule::parse_inline` and raises its refusals as `thinkthen::Error` built with `Error::new`; its `Refusal` copy is gone. `LoadedQuestion` implements `DecisionQuestion` and `DetailQuestion` and has `kind`, so no SQL host asks both arms by hand. SQLite and PostgreSQL write `thinkthen_plan` by serializing `PlanEstimate`, pinned byte for byte by their checks. Git history holds the paid items' text.

Owner: a future ticket after 0.1 for items 6 and 7.

Kind: debt

Pay when: items 6 and 7 are features, paid when a user asks after 0.1.

Debt: 018

Severity: low

Each item names a place where the public API (ticket 0084, `crates/thinkthen/src/public/`) falls short of what the command, a spec, or a ruling promises. Ian's ruling of 2026-09-25 applies throughout: no setting that does nothing.

## 6. The engine reuse test and a library warm call

`sdlc/planning/quality-plan.md` has the row "Engine reuse: two calls in one process, module-level and object API both, open one connection on a counting stub; each database extension does the same after a warm call." No such test exists for the library free functions, one engine object, or a database after `thinkthen_warm`. The libraries have no explicit warm call. The product-side reading of 2026-09-22, which Ian can overturn, keeps the test at low priority and says construction never touches the wire.

Done when the quality-plan row passes on every surface, and the library contract says whether a library `warm` exists.

## 7. Typed descriptions and typed annotate forms outside Rust

Rust ships `Description` and `DescriptionBuilder`. TypeScript's `index.d.ts` now declares `Description` as a string or a JSON object per option, label and level. Python still has no documented `Description` dataclass, and its recognize builder takes text descriptions only (`libraries/python/src/asked.rs`, `_build`). No binding has a typed annotate class, and no cross-surface test compares the digests of one description object sent from Rust, Python, TypeScript and a question file. Ian's ruling 7 of 2026-09-29 defers type-review recommendations beyond ADR 0112, which may cover the typed annotate class.

The product-side asks of 2026-09-22, which Ian can overturn: a string stays valid everywhere; the library never reorders or normalizes the object; the same object gives the same digest on every surface. A typed annotate class serializes to the same JSON as the question file, so the cache is shared. Python follows the Pydantic AI mapping: `bool` is decide, `Literal` or `Enum` is choose, an ordered `IntEnum` is score, `list[Literal]` is tag, a nested class is an annotate form. Rust uses builders with no derive macro.

Done when Python accepts a typed description, including for recognize kinds, the typed annotate form is built or ruled out, and the cross-surface digest test passes.
