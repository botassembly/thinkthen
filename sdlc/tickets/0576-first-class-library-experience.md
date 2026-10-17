# 0576: Give every library a first-class experience

Status: OPEN.

Milestone: 0.2

Depends on: 0562, 0568, 0574

Reviews: revision 243002f, reject

Reviews: revision 340f966, accept

Reviews: revision bd54638, accept

Reviews: revision f6f4f90, accept

Reviews: revision d84408e, accept

Reviews: revision 721f505, reject

Reviews: revision 87d2896, accept

Reviews: revision c8805a2, accept

Reviews: revision fac3992, accept

Reviews: revision a82cc47, reject

Reviews: revision 6bc15d6, accept

Reviews: revision e1fd5b9, accept

Reviews: revision 3b3d67b, reject

Reviews: revision 1d312d5, accept

Reviews: revision 43923ee, reject

Reviews: revision fab1322, accept

Reviews: revision 394de24, reject

Reviews: revision ed62acd, accept

Reviews: revision 0301bb6, accept

Reviews: revision b0042d9, accept

Reviews: revision cd5ccdb, accept

Reviews: revision 5f9680d, accept

Reviews: revision c63b1fe, accept

Reviews: revision 6f8962f, reject

Reviews: revision 1edec6e, accept

Reviews: revision 17d3301, reject

Reviews: revision 9d0b261, accept

Reviews: revision 0296829, accept

Reviews: revision d6f05b1, accept

Reviews: revision 60eaaef, reject

Reviews: revision 35f104f, accept

Reviews: revision 09622f6, reject

Reviews: revision 09622f6, accept

Reviews: revision 1890659, accept

Reviews: revision 470ba03, reject

Reviews: revision bb1ce71, accept

Reviews: revision 68866c2, reject

Reviews: revision 72812a2, reject

Reviews: revision c89a5ee, reject

Reviews: revision db407b3, accept

Reviews: revision 4001d49, accept

Reviews: revision 281cba6, accept

Reviews: revision 16ae7ce, accept

Reviews: revision e363ee5, reject

Reviews: revision 10020da, accept

Reviews: revision c451630, accept

Reviews: revision 3dfe085, accept

Reviews: revision dbb86bc, accept

Reviews: revision f64a780, accept

Reviews: revision b63c597, reject

Reviews: revision 7df92cb, accept

Reviews: revision 7f445b1, accept

Reviews: revision f21ad8a, accept

Reviews: revision a5d7ebf, accept

Reviews: revision e5892dd, accept

Reviews: revision a47be8c, reject

Reviews: revision 1980ac6, accept

Reviews: revision b5ab4ea, reject

Reviews: revision c77da7c, accept

Reviews: revision ec08393, accept

Reviews: revision 02621bc, accept

Reviews: revision 9105d64, accept

Reviews: revision ad2c361, accept

Reviews: revision d705dfc, accept

Reviews: revision 1423089, accept

Reviews: revision 15d416c, reject

Reviews: revision d8d1108, accept

Reviews: revision 2ee2a3e, reject

Reviews: revision da14cdf, reject

Reviews: revision ac54e24, reject

Reviews: revision 47501bc, reject

Reviews: revision cddc589, reject

Reviews: revision af1a102, reject

Reviews: revision f0b0422, reject

Reviews: revision 1a166b0, accept

Reviews: revision 8cfc15a, accept

Reviews: revision 164b2ca, accept

## Outcome

On the 0.2 candidate, a fresh read-only grader gives every language library, database extension and the MCP server B or better on the seven-point library rubric, and no public API exposes transport JSON, schema-path names or raw C layouts.

## Evidence

- Starts from: `ticket/0543-final`, where every 0576 language lane and the PostgreSQL typed family have merged. The 2026-10-13 grading at acf368f06 found Rust A-; Python, C# and MCP B+; Swift B; Java, Go and PostgreSQL B-; Ruby, Kotlin, C++, Ada and DuckDB C+; TypeScript, Scala, C, COBOL and SQLite C; R C-; PHP, Dart and Flutter, and Zig D+; Objective-C D.
- The rubric's seven points: typed requests, one typed result per call, typed question builders, platform idiom, helpers, examples and tests, and thinness.
- The MCP catalog measured 24,753 bytes for `tools/list` at 243002feb, recorded in `sdlc/records/0576-mcp-catalog-measurement.md`. 0581 edits `crates/thinkthen/src/mcp/tools.rs`, so the candidate measures it again.
- 0581 defines what each library call returns: one result with `value`, `probability`, `results` and `facts`. 0582 defines each database's details route. 0580 defines the Rust Polars shape.

## Decisions

- PM decision, 2026-10-13: Swift, C++, Java and Zig read native result views and keep no handwritten JSON parser. C#, Go, TypeScript and R may keep the platform's standard decoder over generated types. The coordinator amends the 2026-10-09 decision and ADR 0129 to say so.
- PM decision, 2026-10-13: the frozen C compatibility sections of ADRs 0125 and 0129 are marked superseded by the no-compatibility ruling.
- PM decision, 2026-10-13: Polars returns typed `List` and `Struct` columns for all ten functions, generated, in the shape 0580 and 0581 define.
- PM decision, 2026-10-13: the Swift and Objective-C lanes prove themselves on a dispatched, nonpublishing GitHub macOS workflow, and the candidate runs it again.
- PM decision, 2026-10-14: the SQLite and PostgreSQL lanes build 0582's `thinkthen_<function>_details` for all ten functions. This follows 0582, which supersedes the 2026-10-13 database family ruling and the 2026-10-14 typed `thinkthen_details` amendment. Answer reading follows 0581.
- PM decision, 2026-10-13: size follows the standing size ruling. The lane budgets are goals: Rust ≤392, C ≤1,905, Python ≤1,043 py and ≤1,175 rs, R ≤487, TypeScript ≤494, Ruby ≤965, PHP ≤420, Java ≤687, Kotlin ≤66, Scala ≤59, C# ≤415, Go ≤628, Dart ≤623, Swift ≤593, Objective-C ≤95 h and ≤292 m, Zig ≤399, C++ ≤466, Ada ≤200 adb and ≤80 ads, COBOL ≤12 c and ≤12 h.

## Scope

- Changes: `databases/sqlite/`, `libraries/*/ratchet*.json`, `databases/*/ratchet*.json` and `sdlc/records/0576*`. A library the fresh grading puts below B reopens its own `libraries/<lang>/` or `databases/<db>/` lane with the grader's findings.
- Remaining work:
  - SQLite lane: on `ticket/0576-sqlite-typed-family`, replace the typed `thinkthen_details`, `thinkthen_try_details` and `thinkthen_details_images` with `thinkthen_<function>_details` for all ten functions as 0582 says, then merge into `ticket/0543-final`.
  - Ceilings: set each library's and database's handwritten ratchet to its measured total on the candidate, and name the cause of each raise above its budget.
  - MCP: measure `tools/list` on the candidate and have a fresh agent load the server and answer one call per tool.
  - Grading: run one fresh read-only grading of every library, database and the MCP server on the candidate, after 0580, 0581 and 0582 merge.
- Keeps: Python's dataframe and pydantic adapters, C as the shared native layer, and one function family per database.

## Proof

- The fresh grading on the candidate gives every library, database and the MCP server B or better on the seven points.
- `tools/list` on the candidate measures under 25 KB in total. A larger catalog blocks 0.2.
- Every language library's `check.sh` passes with its example replayed, and `conformance/library-surface.py --library <lang>` passes for each.
- The SQLite check script passes with one `_details` query per function.

## Defers

The 0581 result shape, the 0582 details routes outside SQLite and the 0580 Polars shape stay with those tickets. Generated names stay with 0562.

## Progress

- 2026-10-13 started
- 2026-10-13 next: Merged the reviewed native helper slice as 8b2a47981a416a324514920e8b72197a89b263e5 after installed C ownership, all 42 shared cases, fourteen ASan door checks and unchanged Ada prerequisite passed; continue the language slices and combined checkpoint.
- 2026-10-14 next: Merged the freshly accepted located-feed file selection repair at 959ed3c7 with 40 request and 39 session checks passing; continue the database and language migrations on the combined branch.
- 2026-10-13 next: Merged reviewed Zig native helpers and strengthened find receipt as 1d9fb9ccccb047b8a3d1717fd959fde8831c05ab with installed ownership and all forty-two routine cases passing; continue remaining language quality work.
- 2026-10-14 next: Merged the accepted PHP typed-library slice at 4d4cab9ae after installed parity, answer ownership and shipped example checks passed; continue remaining library migrations before the candidate.
- 2026-10-14 next: Merged the accepted B+ PostgreSQL typed family at d6145fa84 with obsolete exports removed, unchanged accepted source, 225 retained parity passes and focused rebuilt native checks passing; continue remaining libraries and the candidate ceiling reconciliation.
- 2026-10-14 next: Merged the freshly accepted Objective-C library at 038586e17 with unchanged reviewed source, passing source gates and retained exact-revision Mac installed checks; finish remaining library reviews and shared candidate proof.
- 2026-10-17 next: Integrated on the 0.2 release branch ticket/0543-final; lands with 0543 once the frozen candidate qualifies, so it runs no separate lane.
