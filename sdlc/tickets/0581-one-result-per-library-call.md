# 0581: Return one result per library call

Status: OPEN.

Milestone: 0.2

Reviews: revision 3f0c285, reject

Reviews: revision 0043787, accept

Reviews: revision 544c42d, accept

Reviews: revision 41f439a, reject

Reviews: revision bca9842, accept
Reviews: revision 46a710a, accept
Reviews: revision 7ed1e4b, reject

Reviews: revision 993bd65, accept
Reviews: revision 566177f, accept

Reviews: revision a42fd11, accept

Reviews: revision 6f5b7fb, accept

## Outcome

In every library except C, each of the ten functions returns one result with `value`, `probability`, `results` and `facts`, and no details switch or separate details call exists.

## Evidence

- Starts from `ticket/0543-final`, surveyed 2026-10-14:
  - Python's `Result` already has `.value`, `.probability`, `.results` and `.facts`, plus a `details=True` keyword on all ten calls, `Result.details`, and `__bool__` and `__eq__` that read `.value`.
  - Rust's `Call<T>` holds only `value()` and `facts()`, with no probability and no records for any function. `Engine::details` and `details_with` return `Call<Details>` for decide, choose, score and tag only. `filter` and `annotate` return `Batch` streams.
  - TypeScript, Go, C#, the JVM languages, Swift, PHP and Dart return the records with no `value` or `probability`; the value comes from a second reader such as `decideAnswer`. Zig's answer has `value()` and `facts()` but no probability; C++ reads it through `Call::answer()`. Ruby has `.value` and no probability.
  - The request schema's `details` option is refused except for four functions and changes no output; `crates/thinkthen/src/mcp/tools.rs` filters it.
- Ian, 2026-10-14, agreed the design: one result per call, value and probability on it, the rest on the same object.

## Decisions

- PM decision, 2026-10-14: `value` is the plain answer in each language's idiom. Unresolved is null or none, never false. One input gives one `value`; a list input gives a list in input order. Rank's value is a list of original indexes, filter's the selected inputs, find's the found unit or null. Rust `filter` and `annotate` return one `Call` collected from the stream.
- PM decision, 2026-10-14: `probability` is the yes probability for decide and filter and the picked item's probability for choose and find, aligned with `value`. It is null for score, tag and annotate, whose maps stay in `results`. Rank, recognize and relate carry a probability on each item. This extends ADR 0105 ruling 5A to find and filter.
- PM decision, 2026-10-14: C stays the shared packet layer and is exempt. Ada, COBOL and Objective-C follow the same shape.
- PM decision, 2026-10-14: delete Python's `details=True`, `Result.details`, `__bool__` and `__eq__`, Rust's `Engine::details` and `details_with`, the `decideAnswer`-style readers, and the request `details` option with its admission and MCP filter. Guides teach `result.value`.
- Supersedes: ADR 0105's `Call.details`, ADR 0089's `Engine::details` reuse, the details text in `specification/result.md` and `specification/rust-api-0.2.md`, 0574's answer-accessor change, and 0576's plain-list accessor line. 0574 stops adding accessors; its C++ safety, Zig wait, builders and examples stay.

## Scope

- Changes: `libraries/` except `libraries/polars/`, `crates/thinkthen/src/public/engine.rs`, `crates/thinkthen/src/public/results/`, `crates/thinkthen/src/public/request/`, `crates/thinkthen/src/public/native_batch.rs`, `crates/thinkthen/src/mcp/tools.rs`, `crates/thinkthen/tests/library/`, `sdlc/generators/results/`, `conformance/cases.json`, `specification/result.md`, `specification/rust-api-0.2.md`. Python's pandas and Polars `.tt` results take the same shape in the Python lane: `.probability` a nullable Float64 Series on original rows.
- Keeps: the packet session API, the CLI `--details` flag, SQL (0582), Rust Polars (0580), MCP's envelope, and every record field.
- Size: follow the standing size ruling; the deleted switch, details call and readers offset added lines.

## Proof

One new case in `conformance/cases.json`, replayed by every binding's installed driver: decide on the refund case reads `value` true, `probability` 0.99, one record in `results` and `facts.requests_sent`; choose reads the pick's probability. It fails today in every library lacking `probability` or `results`. Tests of `details=True`, `Engine::details`, the request `details` option and the deleted readers go. `sdlc/scripts/test` and `lint` pass.

## Defers

Registry pages (0578), the website (0575) and docs examples follow after this lands.

PM amendment, 2026-10-14. In pandas and Polars `.tt` results, `.probability` for recognize and relate is a typed list of Float64 per original row, one per item. Decide, filter, choose, find and rank stay nullable Float64; score, tag and annotate stay null.

PM amendment, 2026-10-14. A recognize item's probability is its kept proposal's `span_probability`, matched by the selected bounds and kind; boundary-only recognition uses `proposal.probability`. Entity strength is not a probability and is not exposed as one. No wire or diagnostic change.

PM amendment, 2026-10-14. In C#, a function whose authored values may be any JSON types `Result.Value` and its list variants with the existing generated `JsonElement` authored-value leaf. The result envelope JSON stays private. No new recursive union type.

PM amendment, 2026-10-14. Probability belongs to the settled value. When a call leaves its value unresolved (null), its probability is null in every host, as Rust reports today.
