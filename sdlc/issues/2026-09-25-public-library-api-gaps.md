# Public library API gaps

Status: Open.

Ian's ruling, 2026-09-25: "I don't want any settings that don't do anything. Get rid of it." Item 4 takes the second fix. `cache_bytes` leaves the library, ticket 0084's frozen contract, and every binding before 0.1. The same rule applies to any other setting that has no effect.

This issue merges eight issues, now deleted: `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, `2026-09-25-public-library-api-gaps.md`, and `2026-09-25-public-library-api-gaps.md`. Each one names a place where the public API (ticket 0084, `crates/thinkthen/src/public/`) falls short of what the command, a spec, or a ruling promises. Each gap makes a binding copy engine code, skip a shared case, or drift from the other bindings. One owner of the public API should close them together. Each section below was checked against main on 2026-09-25, after the Python and Python Polars landing. The issue `2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md` stays separate. Its per-process request total shares the root cause of item 1.

## 1. Engine counters count per engine, not per process

What happens today. `Engine::usage()` returns the counts of one engine and its clones (`crates/thinkthen/src/public/engine.rs:140`). Its doc says "This process's totals". `EngineBuilder::build` gives each engine new in-memory counters with `Counters::new(None)` (`crates/thinkthen/src/public/settings.rs:246`). A second engine in the same process starts from zero. The PostgreSQL binding keeps every engine a backend built and adds their counters itself (`databases/postgresql/src/call.rs:179`, `databases/postgresql/NOTES.md:7`). ADR 0047's PostgreSQL section records that workaround.

What the record says. Ticket 0084 calls `Counters` "the broader process total returned by `Engine::usage`" (`sdlc/tickets/0084-freeze-the-public-rust-contract.md:336`). Ian chose a request cap per process for every SQL surface on 2026-09-25 (`sdlc/planning/one-line-plan-2026-09-25.md:22`). That cap reads "the requests its engines in this process have sent", so each SQL surface repeats the sum.

The fix. Count in one process-wide place, as 0084 says, and drop the PostgreSQL sum. Or amend 0084 and the `usage` doc to say the counters belong to each engine. The owner of the public API decides. The status issue's option 1 (persist library counts) should build on whichever place this picks.

Done when: two engines built in one process report one shared total, or 0084 and the doc say per engine and every SQL surface sums through one shared helper.

## 2. No public parser for inline relate rules

What happens today. The command reads inline relate rules as `NAME` or `NAME=SOURCE:TARGET` in the private `inline_rule` (`crates/thinkthen/src/core/relate_file.rs:295`). The PostgreSQL binding's `thinkthen_relate(query, rules text[])` takes the same spellings, so it copies the function (`databases/postgresql/src/relate.rs:34`, with the copy noted at line 59).

What the record says. ADR 0047 makes bindings thin crates over the public API. Two copies of one grammar can drift.

The fix. Export a constructor such as `RelationRule::parse_inline(text, either)` that returns the command's rule and its usage error. PostgreSQL calls it and drops its copy.

Done when: `databases/postgresql/src/relate.rs` has no `inline_rule` of its own and its test calls the public parser.

## 3. `thinkthen::Error` has no public constructor

What happens today. The constructors `Error::of`, `usage`, `local`, `defect`, and `cancelled` are all `pub(crate)` (`crates/thinkthen/src/public/error.rs:114` to `146`). A binding cannot make its own `Error`. The PostgreSQL binding refuses bare text, a spent request total, an over-cap file, and a cancelled wait with its own `Refusal` type (`databases/postgresql/src/call.rs:17` to `40`). Its doc comment says why: "`thinkthen::Error` has no public constructor".

What the record says. ADR 0047 routes every kind through one kind table. Each binding that keeps a second type maps kinds to host errors twice.

The fix. Make `Error::usage(message)` and `Error::local(message)` public, or document the one pattern each binding follows. The owner of the public API decides.

Done when: the PostgreSQL binding raises its own refusals as `thinkthen::Error` and `Refusal` is gone, or a written pattern replaces it.

## 4. `EngineBuilder::cache_bytes` has no effect in the library

What happens today. `cache_bytes` refuses 0 and otherwise returns the builder unchanged (`crates/thinkthen/src/public/settings.rs:208` to `220`). Its doc says the library "keeps no cap and prunes nothing". Python's `tt.Engine(cache_bytes=)`, Ruby's `cache_bytes:`, and PostgreSQL's `thinkthen.cache_bytes` all pass through this setter, so none of them caps the cache. The command's `thinkthen cache prune` reads its own configured cap.

What the record says. Ticket 0084 freezes `cache_bytes(self, value: u64) -> Result<Self, Error>` (`sdlc/tickets/0084-freeze-the-public-rust-contract.md:108`). A frozen setter that does nothing misleads every binding's user.

The fix. Give the library a prune that applies the cap. Or drop the setter through an 0084 amendment before 0.1 ships, and drop it from each binding.

Done when: a test sets a cap, fills the cache past it, and sees the cache folder shrink under the cap, or the setter is gone from 0084 and every binding.

## 5. The library cannot ask find with a none option or annotate over record parts

What happens today. `Engine::find` and the free `find` take a question and units with no switch for the "none of these" candidate (`crates/thinkthen/src/public/bulk.rs:135`, `crates/thinkthen/src/public/mod.rs:211`). The command has `find --none`. `QuestionSet::from_json` refuses any member with a non-root `on` (`crates/thinkthen/src/public/set.rs:37` to `47`). The shared-case runner skips `18-find-second`, `19-find-none`, and `18-annotate-two-groups` for these reasons (`conformance/consumer/consumer/tests/public/cases.rs:7` to `34`). It also skips `25-defect-fault`, which R1-10 already covers.

What the record says. Every shared case should run through the library. Ticket 0084 has no spelling for either feature.

The fix. Amend 0084 with a find option for the none candidate, such as a `FindBuilder` step. Decide whether structured records get per-question parts through `Evidence`. Then remove the three skips.

Done when: the three cases run through the public API and `SKIPPED` holds only `25-defect-fault`.

## 6. The engine reuse test and the library warm call are missing

What happens today. Rule 2 of the old issue is done (see Already fixed). The pinning test is not. The command has a connection-count test (`crates/thinkthen/tests/backend/parallel.rs:302`). No test proves that two module-level library calls, or two calls on one engine object, open one connection. No database test checks it after `thinkthen_warm`. The libraries have no explicit warm call. Only SQLite and PostgreSQL ship `thinkthen_warm` (`databases/sqlite/README.md:23`, `databases/postgresql/README.md:17`).

What the record says. The quality plan has the row "Engine reuse: two calls in one process, module-level and object API both, open one connection on a counting stub (no re-handshake on the second call); each database extension does the same after a warm call" (`sdlc/planning/quality-plan.md:19`). The product-side reading of 2026-09-22, which Ian can overturn, kept that test. It narrowed rule 3: "Warming is explicit only: a `warm` call or `thinkthen_warm`. Construction never touches the wire." It kept rule 5: "The word is the engine." It set priority low. It split out shared connections under width, which need HTTP/2 and a client change, as a separate decision measured first.

The fix. Add the counting-stub test for the Rust free functions and one engine object, then for each library and database extension through its own test harness. Decide whether libraries get an explicit `warm` method, and record the answer in the library contract.

Done when: the quality plan row passes on every surface, and the library contract says whether a library `warm` exists.

## 7. Typed descriptions and typed annotate forms outside Rust

What happens today. Rust ships the typed `Description` with `text` and a `builder()` for `what`, `not_for`, and `examples` (`crates/thinkthen/src/public/question.rs:76` to `171`). Its builders take `Option<Description>` per option, label, and level (`crates/thinkthen/src/public/builders.rs:247`, `296`). Python accepts a mapping for options, labels, and levels and sends it through `json.dumps` (`libraries/python/thinkthen/__init__.pyi:81`, `libraries/python/thinkthen/__init__.py:84`). It has no documented `Description` dataclass and no typed annotate class. Python's recognize builder takes text descriptions only (`libraries/python/src/asked.rs:172` to `183`). TypeScript takes `options`, `levels`, and `labels` as `readonly string[]` only (`libraries/typescript/index.d.ts:19`, `26`, `32`, `55`). It has no `Description` interface and no typed annotate form.

What the record says. The product side filed this on 2026-09-22, and Ian can overturn it. Each language gets one description type, "a string stays valid everywhere", "the library never reorders or normalizes the object", and "the digest across surfaces is the same for the same object". Python gets "a `Description` dataclass, or a plain `dict` accepted where a string is accepted today. Both work; the dataclass is documented." TypeScript gets "an interface `Description` on the options object, string or object per option." Ruby, R, C, and the databases use their plain form.

The second item, a ruling Ian can overturn: "a library user may declare an annotate form as a typed class in languages that have types. The field's type sets the question kind (a boolean is decide, an enumeration is choose, an ordered scale is score), and the field's description carries the instructions, string or JSON. Every field comes back with its value and its probability. The class is sugar over the question set: it serializes to the same compact JSON the question file holds, so the digest is identical to the file form and the cache is shared." Python matches the Pydantic AI mapping: "`bool` is decide, `Literal` or `Enum` is choose, an ordered `IntEnum` is score, `list[Literal]` is tag, a nested class is an annotate form." Ian can overturn the convention match.

Amended 2026-09-23, and Ian can overturn it: Rust ships "typed `Description` and question-set builders, with no derive macro and no second published crate. A derive comes later as an additive feature over the same builder, once a user asks for it."

The fix. Add the documented Python `Description` dataclass and the typed annotate class. Add the TypeScript `Description` interface with string or object per option, and the typed annotate interface. Add Python's object descriptions to recognize kinds. Add one cross-surface test that sends the same description object from Rust, Python, TypeScript, and a question file and compares the digests.

Done when: Python and TypeScript accept a typed description and a typed annotate form, and the cross-surface digest test passes.

## 8. Python Polars does not follow the ADR 0047 column table

What happens today. Ticket 0106 landed the Python Polars door on main with its own frame writer. `widened` in `libraries/python/src/arrow/write.rs:335` to `354` builds each widened cell itself. A score uses Rust's `f64` display, so 1.0 becomes `1`. The failed marker comes from a hand-built string and the binding's own cause table (`libraries/python/src/engine.rs:55` to `64`). An unwidened tag column is a list column (`libraries/python/src/arrow/write.rs:386`), and `annotate` over a frame uses it (`libraries/python/src/frame.rs:159`). The Rust Polars door takes every widened cell from `value_json` (`crates/thinkthen/src/public/frame/column.rs:132` to `155`, ticket 0130) and writes a frame's tag column as JSON array text (`crates/thinkthen/src/public/frame/column.rs:120` to `126`). The two doors write different text for the same frame. The Python conformance check parses widened cells with `json.loads` (`libraries/python/tests/conformance.py:120`), so it cannot see the number text difference.

What the record says. ADR 0047 item 10 sets one table for both doors. Score widens to "the number text from `value_json`". Tag is "`String` holding the JSON array text". A failed cell is "the marker from `value_json`". "A widened cell takes its text unchanged from `AnnotatedRecord::value_json`, the engine's one serializer. No door keeps its own table of failure causes. Ian can overturn the table." A column widens only when the same reply also holds a usable answer, so a single failed answer ends the call with a `Backend` error.

The fix. Make Python's frame writer take widened cells and frame tag cells from `value_json`, as the Rust door does. Remove the Python cause table from the widened path. Add a frame case with a widened score of 1.0 that compares the cell text exactly, not after `json.loads`.

Done when: the same annotate frame gives byte-identical columns through the Python and Rust Polars doors, including a widened score, a failed marker, and a tag column.

## Already fixed

- One engine per process, rule 2. Rust's free functions use one lazily built `default_engine()` (`crates/thinkthen/src/public/mod.rs:51`). Python (`libraries/python/src/engine.rs:296`), TypeScript (`libraries/typescript/src/door.rs:152`), Ruby (`libraries/ruby/src/ffi.rs:304`), and R (`libraries/r/thinkthen/src/rust/src/lib.rs:157`) delegate to it. SQLite holds one engine for the process (`databases/sqlite/README.md:42`). Rules 1 and 4 were already ADR 0017. The rest of that issue is item 6.
- Typed descriptions in Rust. The public API ships `Description` and `DescriptionBuilder` with the builder form the 2026-09-23 amendment chose. The rest of that issue is item 7.
