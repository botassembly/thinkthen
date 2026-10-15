# 0581: Return one result per library call

Status: OPEN.

Milestone: 0.2

## Outcome

In every library except C, each of the ten functions returns one result with `value`, `probability`, `results` and `facts`, and no details switch or separate details call exists.

## Evidence

- Ian, 2026-10-14, agreed the design: one result per call, value and probability on it, the rest on the same object.
- Before this ticket, Python offered a `details=True` switch and `Result.details`, Rust offered `Engine::details` and `details_with` for four functions, most other libraries returned records with a separate reader such as `decideAnswer` for the value, and the request schema carried a `details` option that changed no output.
- This ticket closes with the passing 0543 candidate.

## Decisions

- PM decision, 2026-10-14: `value` is the plain answer in each language's idiom. Unresolved is null or none, never false. One input gives one `value`; a list input gives a list in input order. Rank's value is a list of original indexes, filter's the selected inputs, find's the found unit or null. Rust `filter` and `annotate` return one `Call` collected from the stream.
- PM decision, 2026-10-14: `probability` is the yes probability for decide and filter and the picked item's probability for choose and find, aligned with `value`. It is null for score, tag and annotate, whose maps stay in `results`. Rank, recognize and relate carry a probability on each item. This extends ADR 0105 ruling 5A to find and filter.
- PM decision, 2026-10-14: probability belongs to the settled value. When a call leaves its value unresolved (null), its probability is null in every host. Probability is never typed as an untyped object or arbitrary JSON.
- PM decision, 2026-10-14: a recognize item's probability is its kept proposal's `span_probability`, matched by the selected bounds and kind; boundary-only recognition uses `proposal.probability`. Entity strength is not a probability and is not exposed as one. No wire or diagnostic change.
- PM decision, 2026-10-14: in pandas and Polars `.tt` results, `.probability` is a nullable Float64 Series on original rows for decide, filter, choose, find and rank, a typed list of Float64 per row for recognize and relate, and null for score, tag and annotate.
- PM decision, 2026-10-14: C stays the shared packet layer and is exempt. Ada, COBOL and Objective-C follow the same shape.
- PM decision, 2026-10-14: delete Python's `details=True`, `Result.details`, `__bool__` and `__eq__`, Rust's `Engine::details` and `details_with`, the `decideAnswer`-style readers, and the request `details` option with its admission and MCP filter. Guides teach `result.value`.
- PM decision, 2026-10-14: each language takes the result shape in this table. Every other name and type stays as it is.

  | Language | Result shape |
  | --- | --- |
  | Every library except C | One result per call, carrying `value`, `probability`, `results` and `facts`. An unresolved value has null probability. Probability is never untyped. |
  | C# | Generated overloads by input shape. A function whose authored values may be any JSON types `Result.Value` and its list variants with the existing generated `JsonElement` leaf. The result envelope JSON stays private. No new recursive union type. |
  | Swift | Generated overloads by input shape: text or one value returns a scalar `CallResult`, and records, sources or feeds return the list form, with optional `Double` or `[Double?]` probability. No `ResultValue` enum for callers to unwrap. |
  | Objective-C | No overloading, so it keeps `TTCall` with typed `TTCallValue` and `TTCallProbability` wrappers over a scalar or `NSArray`. Probability is never `id`. |
  | C++ | `ResultCall::result()` returns a generated `tt::results::CallResult<V, P, R>` whose `P` is concrete per input shape, such as `std::optional<double>` or a vector of them, selected by overloads or templates. It keeps the `Data` authored-value leaf and has no `std::variant` probability. |
  | Dart | No overloading, so it uses `CallResult<V, R>` with the `Object` authored-value leaf and a closed numeric `CallProbability` carrier for scalar, list or nested-list probability. |
  | JVM | A generated `CallResult<V, P, R>` with `Double` or `List<Double>` probability, the `Object` authored-value leaf, and idiomatic Kotlin and Scala wrappers. The base `RequestInput` router is private; callers use the concrete-input overloads, each with a concrete probability type. |
  | Zig | The ten function names stay, with a generated `Arguments(comptime Input: type)` inferred from the concrete Text, Json, Records, Sources or Feed payload, so probability is a concrete optional `f64` or a list of them. No probability union and no second call family. `RequestQuestion` and `Input` serialize through a private generated encoder, with no public `jsonStringify` hooks. |
  | Ada | The existing input overloads return generated scalar or list and item or item-list result records with typed `Number_Field`, list or nested probability, and owned receipt wrappers. Choose, tag, rank and score values use the concrete `Choice`, `Text_List`, `Rank_List` and `Long_Float` types, with generated list types for collection overloads. `Values.Value` remains only where authored or structured values can be any JSON. |
  | COBOL | `Answer*Result` names and the `tt_value` leaf stay, with typed nullable numeric probability carriers (a presence flag plus a double, list or nested list), owned canonical receipt structs under results and `AnswerFacts`, and the 8192-byte scalar contract. |

- Supersedes: ADR 0105's `Call.details`, ADR 0089's `Engine::details` reuse, the details text in `specification/result.md` and `specification/rust-api-0.2.md`, 0574's answer-accessor change, and 0576's plain-list accessor line. 0574 stops adding accessors; its C++ safety, Zig wait, builders and examples stay.

## Scope

- Changes: `libraries/` except `libraries/polars/`, `crates/thinkthen/src/public/engine.rs`, `crates/thinkthen/src/public/results/`, `crates/thinkthen/src/public/request/`, `crates/thinkthen/src/public/native_batch.rs`, `crates/thinkthen/src/mcp/tools.rs`, `crates/thinkthen/tests/library/`, `sdlc/generators/results/`, `conformance/cases.json`, `specification/result.md`, `specification/rust-api-0.2.md`. Python's pandas and Polars `.tt` results take the same shape in the Python lane.
- Keeps: the packet session API, the CLI `--details` flag, SQL (0582), Rust Polars (0580), MCP's envelope, and every record field.
- Size: follow the standing size ruling; the deleted switch, details call and readers offset added lines.

## Proof

One case in `conformance/cases.json`, replayed by every binding's installed driver: decide on the refund case reads `value` true, `probability` 0.99, one record in `results` and `facts.requests_sent`; choose reads the pick's probability. Tests of `details=True`, `Engine::details`, the request `details` option and the deleted readers go. `sdlc/scripts/test` and `lint` pass.

## Defers

Registry pages (0578), the website (0575) and docs examples follow after this lands.
