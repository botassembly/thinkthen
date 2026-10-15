# 0576: Give every library a first-class experience

Status: OPEN.

Milestone: 0.2

Depends on: 0562, 0568, 0574

## Outcome

On the 0.2 candidate, a fresh read-only grader gives every language library, database extension and the MCP server B or better on the seven-point library rubric, and no public API exposes transport JSON, schema-path names or raw C layouts.

## Evidence

- Starts from: `ticket/0543-final`, where every 0576 language lane and the PostgreSQL typed family have merged. The 2026-10-13 grading at acf368f06 found Rust A-; Python, C# and MCP B+; Swift B; Java, Go and PostgreSQL B-; Ruby, Kotlin, C++, Ada and DuckDB C+; TypeScript, Scala, C, COBOL and SQLite C; R C-; PHP, Dart and Flutter, and Zig D+; Objective-C D.
- The rubric's seven points: typed requests, one typed result per call, typed question builders, platform idiom, helpers, examples and tests, and thinness.
- The MCP catalog measured 24,753 bytes for `tools/list` at 243002feb, recorded in `sdlc/records/0576-mcp-catalog-measurement.md`. 0581 edits `crates/thinkthen/src/mcp/tools.rs`, so the candidate measures it again under 0543.
- 0581 defines what each library call returns: one result with `value`, `probability`, `results` and `facts`. 0582 defines each database's details route. 0580 defines the Rust Polars shape.

## Decisions

- PM decision, 2026-10-13: Swift, C++, Java and Zig read native result views and keep no handwritten JSON parser. C#, Go, TypeScript and R may keep the platform's standard decoder over generated types. The coordinator amends the 2026-10-09 decision and ADR 0129 to say so.
- PM decision, 2026-10-13: the frozen C compatibility sections of ADRs 0125 and 0129 are marked superseded by the no-compatibility ruling.
- PM decision, 2026-10-13: Polars returns typed `List` and `Struct` columns for all ten functions, generated, in the shape 0580 and 0581 define.
- PM decision, 2026-10-13: the Swift and Objective-C lanes prove themselves on a dispatched, nonpublishing GitHub macOS workflow, and the candidate runs it again.
- PM decision, 2026-10-14: the SQLite and PostgreSQL lanes build 0582's `thinkthen_<function>_details` for all ten functions. This follows 0582, which supersedes the 2026-10-13 database family ruling and the 2026-10-14 typed `thinkthen_details` amendment. Answer reading follows 0581.
- PM decision, 2026-10-13: size follows the standing size ruling. The lane budgets are goals: Rust ≤392, C ≤1,905, Python ≤1,043 py and ≤1,175 rs, R ≤487, TypeScript ≤494, Ruby ≤965, PHP ≤420, Java ≤687, Kotlin ≤66, Scala ≤59, C# ≤415, Go ≤628, Dart ≤623, Swift ≤593, Objective-C ≤95 h and ≤292 m, Zig ≤399, C++ ≤466, Ada ≤200 adb and ≤80 ads, COBOL ≤12 c and ≤12 h.

## Scope

- Changes: `sdlc/records/0576*`. A library the fresh grading puts below B reopens its own `libraries/<lang>/` or `databases/<db>/` lane with the grader's findings.
- Done:
  - SQLite: the typed family with `thinkthen_<function>_details` for all ten functions, as 0582 says, is merged into `ticket/0543-final`.
  - MCP: `tools/list` measured 24,754 bytes with its newline, and a fresh agent called every tool once.
  - Ceilings: each library's and database's handwritten ratchet is set to its measured total, with the cause of each raise named in the 0543 record.
- Remaining work:
  - Grading: one formal fresh read-only grading of every library, database and the MCP server on the candidate. PM decision, 2026-10-14: it runs as part of 0543's area review, and the coordinator owns it.
- Keeps: Python's dataframe and pydantic adapters, C as the shared native layer, and one function family per database.

## Proof

- The fresh grading on the candidate gives every library, database and the MCP server B or better on the seven points.
- `tools/list` on the candidate measures under 25 KB in total. A larger catalog blocks 0.2.
- Every language library's `check.sh` passes with its example replayed, and `conformance/library-surface.py --library <lang>` passes for each.
- The SQLite check script passes with one `_details` query per function.

## Defers

The 0581 result shape, the 0582 details routes outside SQLite and the 0580 Polars shape stay with those tickets. Generated names stay with 0562.

## Progress
