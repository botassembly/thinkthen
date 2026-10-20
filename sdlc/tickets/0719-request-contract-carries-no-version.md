# 0719: The request contract and results carry no version or wording version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0704

## Outcome

- Changes: `.gitattributes`, `build-tools/results/generate.py`, `build-tools/results/layouts.py`, `build-tools/results/registry.py`, `build-tools/results/templates/objc.py`, `build-tools/results/templates/swift.py`, `conformance/c_parity.py`, `conformance/cases.json`, `conformance/children/native_fixture.py`, `conformance/children/session_cases.py`, `conformance/cli_parity.py`, `conformance/fixtures/complete.json`, `conformance/library-surface-schema.py`, `conformance/library-surface-test.py`, `conformance/named-inputs.json`, `conformance/parity.py`, `conformance/test_parity.py`, `crates/thinkthen-host/src/complete_native/observations.rs`, `crates/thinkthen/src/core/declaration.rs`, `crates/thinkthen/src/core/declaration/parsing.rs`, `crates/thinkthen/src/core/mod.rs`, `crates/thinkthen/src/core/relate_file.rs`, `crates/thinkthen/src/mcp/request.rs`, `crates/thinkthen/src/public/builders.rs`, `crates/thinkthen/src/public/complete/ownership_tests.rs`, `crates/thinkthen/src/public/declarations.rs`, `crates/thinkthen/src/public/mod.rs`, `crates/thinkthen/src/public/question_metadata.rs`, `crates/thinkthen/src/public/request.rs`, `crates/thinkthen/src/public/request/request.schema.json`, `crates/thinkthen/src/public/request/tests.rs`, `crates/thinkthen/src/public/results/aggregate_reading.rs`, `crates/thinkthen/src/public/results/complete.schema.json`, `crates/thinkthen/src/public/results/reading.rs`, `crates/thinkthen/tests/`, `crates/thinkthen/transforms/sweep.jq`, `databases/duckdb/tools/receipt_types.py`, `databases/sqlite/tests/complete/project.py`, `libraries/c/src/current/author.rs`, `libraries/c/tests/c/authored.c`, `libraries/c/tests/c/request_preview.c`, `libraries/c/tests/c/session.c`, `libraries/c/tests/c/session_wait.c`, `libraries/c/tests/door/session.rs`, `libraries/c/tests/installed_cases.py`, `libraries/cobol/checks/blocking.py`, `libraries/csharp/tests/public_types.py`, `libraries/csharp/tests/source/CarrierChecks.cs`, `libraries/csharp/tests/source/FixtureQuestions.g.cs`, `libraries/csharp/tests/source/NativeCases.cs`, `libraries/csharp/tests/source/SessionChecks.cs`, `libraries/csharp/tests/source/TypeCase.cs`, `libraries/jvm/tests/SessionCases.java`, `libraries/mcp/conformance.py`, `libraries/mcp/test_installed.py`, `libraries/objective-c/Sources/Foundation/TTResultPrivate.h`, `libraries/objective-c/checks/FoundationCases.m`, `libraries/php/fixtures/session_consumer.php`, `libraries/rust/consumer/src/fixture.rs`, `libraries/rust/consumer/tests/settled.rs`, `libraries/zig/Tests/session.zig`, `sdlc/scripts/generate-c-header.py`, `site/examples/functions/question-file/files/records/refund.json`, `site/src/data/catalog.mjs`, `specification/fixtures/question-file/corpus.json`, `specification/fixtures/types/corpus.json`, `specification/question-file.md`, `specification/question-file.schema.json`, `specification/request.schema.json`, `specification/result.schema.json`, `specification/rust-api-0.2.md`, `specification/settings.md`, `transforms/sweep/human-label-test.sh`, `transforms/sweep/sweep.jq`.
- This ticket claims the generator sources and the Rust schemas, not the generated header and wrapper files. The lander regenerates the outputs at merge with `python3 build-tools/results/generate.py --all` and `python3 sdlc/scripts/generate-c-header.py`, and `lint`'s currency checks confirm them.
- `thinkthen.request/1`, the request `schema` member and `RequestVersion` are gone from the request contract, the public Rust API, both request schemas, the MCP tool schemas and every generated wrapper, including `libraries/jvm/session/thinkthen/RequestVersion.java`. A request that still carries `schema` is refused with one sentence naming the member to delete.
- Question files and SQL declarations that still carry `wording_version` are refused with one sentence naming the member to delete, and the question-file schema, its corpus and the site question-file example drop it.
- `wording_version` is gone from the public Rust API (`with_wording_version` and the declaration and reading carriers), the C interface (`thinkthen_complete_wording_version_v1` and its presence type), results, the result schemas, the packet oracle, generated wrappers, the database receipt types, the `sweep` transform and the specification.
- The same change updates `specification/rust-api-0.2.md`: an Authority entry for this ticket and the retired declarations, so `sdlc/scripts/inventory` in `lint` passes. It regenerates `libraries/c/include/thinkthen.h` with `sdlc/scripts/generate-c-header.py`, passes `sdlc/scripts/check-c-exports.py`, and regenerates the wrappers from `build-tools/results/`.

### Proof

- On the M5 through `pm job run --machine m5 0719 SHA --memory 8 -- cargo nextest run -p thinkthen request_contract`, a request without `schema` runs and a request with `"schema":"thinkthen.request/1"` is refused with the exact sentence. Before the change, the first is refused.
- On the M5 through pm job, `sdlc/scripts/lint` passes, which runs the API inventory, the header check and the generated-file currency checks; `git grep -nE 'RequestVersion|wording_version|thinkthen\.request/' -- . ':!sdlc'` finds nothing.
- On the M5 through pm job, `sdlc/scripts/surfaces` passes for every library that runs on macOS. The DuckDB and PostgreSQL receipt changes run at the 0619 container checkpoint.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md), and Ian's 2026-10-19 ruling in 0706's amendment that `wording_version` belongs to the proxy, not the SDK. Split from 0706. After 0704 took the question, recognize and relate file `version` removal, this ticket also owns the file side of `wording_version`, so it lands whole after 0704.

Starts from:

- `RequestVersion` and the `schema` field of the request struct in `crates/thinkthen/src/public/request.rs`, and the MCP request builder in `crates/thinkthen/src/mcp/request.rs` that sets `RequestVersion::V1`.
- `with_wording_version` in `crates/thinkthen/src/public/builders.rs`, the declaration carrier in `crates/thinkthen/src/public/declarations.rs` and `crates/thinkthen/src/core/declaration.rs`, and the reading fields in `crates/thinkthen/src/public/results/reading.rs` and `aggregate_reading.rs`.
- The generator registry in `build-tools/results/registry.py`, which lists `RequestVersion.java`, and the layouts in `build-tools/results/layouts.py`.
- The packet oracle `conformance/fixtures/complete.json` and `conformance/cases.json`, which carry `wording_version` in results.

Collides with 0707 and 0709 on generated wrappers and `libraries/c/include/thinkthen.h`, and with 0642 on `conformance/fixtures/complete.json`; land them one after another. Paths under `crates/thinkthen/src/mcp/` move when 0639 lands; the claim follows the move.
