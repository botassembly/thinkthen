# 0357: `rank` returns each record's place and probability at the C door

Status: landed. Lane claude-3. A fresh ticket review found four gaps, all fixed; the build began during that review, so the code review treated all of it as new. The code review found three gaps, all fixed, and then accepted. Branch `ticket/0357-rank-probability-at-the-c-door`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Serves `../issues/closed/2026-09-30-rank-returns-no-probability-at-the-c-door.md`. Mirrors ticket 0354 slice B, which did the same for `find` (landing `11294af2d`).

## Outcome

The C door's `rank` value is an array of `{"index":N,"record":TEXT,"probability":P}` rows, most likely yes first, with a zero-based input index. That is the shape Python, TypeScript and Ruby already return. The 13 languages on the C door read the door's value as host JSON, so they return the place and probability with no binding code. The shared type corpus case `15-rank-records` expects the new rows, so every C-door port proves it.

## Evidence

- Starts from: main `11294af2d`.
  - `specification/rank.md` covers only the command. It prints each record as it arrived and gives the probability under `--details`; it names no library or door value. `specification/types.md` lists the `rank` answer as "Every record, most likely yes first". The header (`libraries/c/include/thinkthen.h`) says "an array of every record, most likely yes first, for rank".
  - The Rust library's `Ranked<T>` (`crates/thinkthen/src/public/results.rs`) holds the record and its yes probability but not its input place. `Engine::rank_with` (`public/bulk.rs`) collects rows in input order, then reorders them by `ranking`, so the place is known there.
  - Python returns `{"index","record","probability"}` rows: `libraries/python/src/engine/operations.rs` wraps each record in an `Indexed(at, text)` to keep its place. TypeScript returns `Ranked {index, record, probability}` (`libraries/typescript/index.d.ts`). Ruby returns `Ranked(index, record, probability)` (`libraries/ruby/lib/thinkthen.rb`). R returns a one-based place, the record and the probability.
  - The C door's `call.rs` `"rank"` arm writes `row.input()` alone, so the door drops the place and probability.
  - The C-door port folders hold no `rank` wrapper; each passes the door value through as host JSON (ADR 0112). Their type-corpus runners check each value against the generated schema's `rank` definition and the corpus response. Their own matrix tests check `rank` as the strings `["rank-one","rank-two"]`: Go, C#, the JVM family (Java, Kotlin, Scala), PHP, Swift, Zig, Ada, Objective-C and COBOL. C++ and Dart with Flutter check `rank` only through the type corpus.
  - `conformance/cases.json` gives `operation.ranking` rows with `index` and `probability` for `15-rank-records` and `16-rank-stable-tie`. The C door's `tests/door/cases.rs` checks the record texts alone.
- Keeps: the order and its tie rule, the command's `rank` output and its `--details` rows, every request body, every other door value, the C ABI and its symbols, and the Python, TypeScript, Ruby and R `rank` shapes. `Ranked::input`, `probability` and `into_input` keep their meaning.
- Changes: the crate type, the door arm, the schema and corpus, the door tests, the port rank checks and the docs.
  - `Ranked<T>` moves to its own file `public/results/ranked.rs` (`results.rs` is at 485 of 500 lines). It gains its zero-based input place, set in `rank_with`, read through a new `Ranked::index`.
  - The crate gains one public serialized type for one ranked row, with the index, the record's text and its probability, and one `Ranked` method that returns it. The type derives the schema under `cfg(test)`. The block below lists them for `sdlc/scripts/inventory`.
  - `libraries/c/src/call.rs` writes those rows for `rank`.
  - `schema_tests.rs` derives the `rank` definition from that type, so `specification/result.schema.json` is rewritten by the test.
  - `specification/fixtures/types/corpus.json` case `15-rank-records` expects the three rows with indexes 1, 2, 0 and probabilities 0.9, 0.5, 0.2. A new invalid case `rank-without-probability` keeps the bare string array refused by the schema.
  - `libraries/c/tests/door/cases.rs` builds each expected row from `operation.ranking`. `tests/door/bytes.rs` compares the door rows' records with the command's `--jsonl` lines. `tests/door/golden.rs` pins the new `rank` reply bytes. `examples/functions.txt` shows the rows.
  - Each port's own matrix test checks the rank rows' records and a numeric probability in place of the bare strings: `libraries/go/thinkthen_test.go`, `libraries/csharp/tests/source/Program.cs`, `libraries/jvm/tests/Matrix.java`, `libraries/php/fixtures/matrix.php`, `libraries/swift/Tests/fixtures/matrix.swift`, `libraries/zig/Tests/matrix.zig`, `libraries/ada/checks/legacy/main.adb`, `libraries/cobol/checks/run_matrix.py` (the COBOL program prints the value; the runner checks it) and `libraries/objective-c/checks/matrix.m`, whose shared filter and rank branch splits in two. Only the rank checks change. The `run_matrix.py` and `expected_requests.json` lines that name `rank-one` pin request bodies, which do not change. No binding source file changes; the build names any exception here with its reason.
  - Line ceilings rise to the measured totals: `sdlc/ratchet.json` by 54 for the new row type, its withheld `Debug` and the moved `Ranked`; `libraries/c/ratchet.json` by 15 for the door test rows; and by 1 to 7 lines each for the port rank checks in `libraries/go/ratchet.go.json`, `libraries/php/ratchet.php.json`, `libraries/swift/ratchet.swift.json`, `libraries/zig/ratchet.zig.json`, `libraries/objective-c/ratchet.m.json` and `libraries/cobol/ratchet.py.json`.
  - `libraries/c/include/thinkthen.h`, `libraries/c/DESIGN.md`, `specification/types.md` and `libraries/BINDING-AUTHOR.md` name the new value. `CHANGELOG.md` gains one line.
  - The issue moves to `sdlc/issues/closed/` with a `Resolution:` line.
- Proof: conformance cases, pinned bytes and every C-door port check.
  - The C door's conformance runner passes `15-rank-records` and `16-rank-stable-tie` with each row's index and probability. `tests/door/bytes.rs` and `golden.rs` pin the rows. `specification/fixtures/types/self-test` passes against the real door. The schema drift test passes on the rewritten schema.
  - Each C-door port surface check passes with its type-corpus runner reading the new `15-rank-records` response and its matrix test reading the new rows, with no binding source change.
  - Checks before landing: `sdlc/scripts/test`, `spec`, workspace clippy with `-D warnings` on all targets, `policy.py`, `tickets`, `inventory`, lint in a clean checkout, the C door tests, and the surface checks of the C-door port folders. Python, TypeScript, Ruby, R, Polars and the SQL extensions read `Ranked` through unchanged methods; the coordinator's sweep covers them.
- Defers: Python and Ruby keep their own `Indexed` place wrappers; `Ranked::index` could replace them, but they work and the change gains nothing for a user. `site/` belongs to the marketing lead; the build checks whether it shows a C-door `rank` value and files a message if so.

### Added public declarations

```text
fn Ranked::index(&self) -> usize
fn Ranked::row(&self) -> RankedRow<'_>
impl Serialize for RankedRow
struct RankedRow<'a>
```

## What the build taught us

- The C door dropped the `rank` place and probability; no port did. The C-door port folders read the door value as host JSON, so no binding source file changed. Their own matrix tests pinned the old strings and needed updating, and the shared type corpus now pins the rows for all of them.
- `Engine::rank_with` already held each row's input place: rows arrive in input order before the sort. The fix kept the place on `Ranked` instead of wrapping records, as Python and Ruby still do.
- A port's rank check that grows by a line moves that port's line ceiling. Seven ceilings moved, and the ticket first named four; the code review measured all of them.
- Two port checks shared one branch between `filter` and `rank` (Zig and Objective-C), and PHP shared one case line. Each split in two so the `filter` check stays as it was.
- Checks on the rebased branch: `sdlc/scripts/test` (131 and 20 passed, 56 type schema cases against the real door), `spec` (24 demos green), workspace clippy with `-D warnings` on all targets, `policy.py`, `tickets`, `inventory`, lint in a clean checkout, the C door tests (32 passed), and the surface checks of C, Go, C++, C#, the JVM family, Dart with Flutter, PHP, Swift, Zig, Ada, Objective-C and COBOL. The JVM check first failed to compile: its rank check reused a pattern name the method already held.
