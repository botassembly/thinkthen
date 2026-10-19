# 0706: Question, recognize and relate files carry no version

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0704

## Outcome

- Changes: `build-tools/results/templates/go.py`, `build-tools/results/templates/swift.py`, `conformance/calibration.json`, `conformance/cases.json`, `conformance/consumer/consumer/tests/public/parts.rs`, `conformance/consumer/consumer/tests/public/settings.rs`, `conformance/named-inputs.json`, `conformance/recognition-context.json`, `crates/thinkthen/src/cli/check/functions.rs`, `crates/thinkthen/src/cli/edge/deadline_tests.rs`, `crates/thinkthen/src/cli/schedule/width_tests.rs`, `crates/thinkthen/src/core/check.rs`, `crates/thinkthen/src/core/declaration/parsing.rs`, `crates/thinkthen/src/core/question_set/`, `crates/thinkthen/src/core/recognize_file.rs`, `crates/thinkthen/src/core/relate_file.rs`, `crates/thinkthen/src/core/relate_file/tests.rs`, `crates/thinkthen/src/core/result/complete/tests/aggregate.rs`, `crates/thinkthen/src/engine/facade_tests.rs`, `crates/thinkthen/src/mcp/tests/execution.rs`, `crates/thinkthen/src/mcp/tests/loading.rs`, `crates/thinkthen/src/ownership_tests/bulk_details_tests.rs`, `crates/thinkthen/src/public/request/question_parts.rs`, `crates/thinkthen/src/public/request/request.schema.json`, `crates/thinkthen/src/public/request/tests.rs`, `crates/thinkthen/tests/`, `databases/duckdb/cpp/verify_nested.py`, `databases/duckdb/tools/`, `databases/postgresql/check.sh`, `databases/postgresql/examples.json`, `databases/postgresql/fixtures/`, `databases/postgresql/tests/native_records.py`, `databases/postgresql/tests/rank_sets_cases.py`, `databases/postgresql/tests/settings_cases.py`, `databases/sqlite/examples.json`, `databases/sqlite/src/tables.rs`, `databases/sqlite/tests/rank_backend.py`, `databases/sqlite/tests/test_files.py`, `databases/sqlite/tests/test_rank_sets.py`, `databases/sqlite/tests/test_values.py`, `demos/14-grade-a-batch/checks.json`, `demos/16-triage-pipeline/questions.json`, `demos/45-map-relationships/relations.json`, `libraries/c/tests/c/session.c`, `libraries/c/tests/door/session.rs`, `libraries/c/tests/sql_saved_reading.rs`, `libraries/cpp/fixtures/owned_calls.py`, `libraries/cpp/include/thinkthen/inputs_generated.hpp`, `libraries/csharp/src/RequestInputs.g.cs`, `libraries/csharp/tests/source/FixtureQuestions.g.cs`, `libraries/csharp/tests/source/Program.cs`, `libraries/csharp/tests/source/SessionChecks.cs`, `libraries/csharp/tests/source/TypeCase.cs`, `libraries/dart/lib/src/session/inputs_generated.dart`, `libraries/php/src/session/inputs_generated.php`, `libraries/python/examples.json`, `libraries/python/tests/test_pydantic.py`, `libraries/python/tests/test_series_accessors.py`, `libraries/python/thinkthen/pydantic.py`, `libraries/r/examples/examples.json`, `libraries/r/tests/verbs.R`, `libraries/ruby/examples.json`, `libraries/ruby/tests/fixture/form.json`, `libraries/rust/README.md`, `libraries/rust/examples/annotate.rs`, `libraries/swift/Sources/ThinkThen/InputsGenerated.swift`, `probes/annotate-0015/`, `probes/annotate-batching/questions.json`, `probes/speed/workloads/`, `site/examples/beatles/annotate/files/questions.json`, `site/examples/beatles/relate/files/rules.json`, `site/examples/functions/annotate/files/form.json`, `site/examples/functions/question-file/files/records/refund.json`, `site/examples/functions/recognize/files/kinds.json`, `site/examples/functions/recognize/files/names.json`, `site/examples/functions/relate/files/makes.json`, `site/examples/functions/relate/more/duckdb.sql`, `site/examples/how-tos/bash/fill-a-form-by-selection/files/form.json`, `site/examples/how-tos/bash/label-a-json-file/files/triage.json`, `site/examples/how-tos/triage-a-support-inbox/files/inbox-form.json`, `site/examples/install/polars/files/form.json`, `site/examples/install/python/files/form.json`, `site/examples/reference/annotate/files/form.json`, `site/examples/reference/annotate/files/on-body.json`, `site/examples/reference/question-sets/files/triage.json`, `spec/annotate.md`, `spec/rank-sets.md`, `spec/result.md`, `specification/annotate.md`, `specification/fixtures/files/questions.json`, `specification/fixtures/question-file/corpus.json`, `specification/fixtures/recognize/`, `specification/fixtures/relate/`, `specification/fixtures/types/corpus.json`, `specification/question-file.md`, `specification/question-file.schema.json`, `specification/rank.md`, `specification/recognize.md`, `specification/relate.md`, `specification/request.schema.json`.
- Question sets, recognize files and relate files have no `version` member, and question files and SQL declarations have no `wording_version` member. A file that still has either is refused with one clear sentence naming the member to delete. The question-set hash keeps its current constant, so `questions_sha256` values do not change.
- The resolved question set that `QuestionSet` serializes no longer writes `version`.
- The question-set definition in the request schema drops `version`, and the generated input readers in each wrapper regenerate from it.
- Docs, site examples, demos, probe inputs, spec fixtures and SQL examples drop these members. `sdlc/scripts/spec` and `sdlc/scripts/demos` replay every changed page and demo.
- Configuration files, backend profiles, the request contract and the `wording_version` carriers in the public API, the C interface, results and wrappers are 0720, 0721 and 0719.

### Proof

- On the M5 through `pm job run --machine m5 0706 SHA --memory 8 -- cargo nextest run -p thinkthen --lib core::question_set core::recognize_file core::relate_file core::declaration`, one table per input parses the file without the member and refuses a file that carries `version` or `wording_version` with the exact sentence. Before the change, the files without `version` are refused and the refusal cases parse.
- On the M5 through pm job, `sdlc/scripts/spec`, `sdlc/scripts/demos` and `sdlc/scripts/surfaces` pass for the Rust, C, Python, Ruby, R and SQLite fixtures this ticket edits.
- The DuckDB and PostgreSQL suites under `databases/duckdb/tools/` and `databases/postgresql/` run at the 0619 container checkpoint; the M5 cannot run them.

## Evidence

Ruling: [no version numbers before 1.0](../decisions/2026-10-19-no-version-numbers-before-1-0.md). Each of these files accepts exactly one value today, so the marker guards nothing and users must write it. 0704 (running) makes the question-set `version` optional; this ticket removes it.

Starts from:

- `QuestionSet::parse` in `crates/thinkthen/src/core/question_set/parsing.rs` lists `version` among its keys and checks `value.member("version")`; `crates/thinkthen/src/core/question_set/resolved.rs` writes `version` when it serializes a resolved set.
- `RecognizeSpec::parse` in `crates/thinkthen/src/core/recognize_file.rs` checks `version` is 1.
- The relate file struct in `crates/thinkthen/src/core/relate_file.rs` holds `version: u8` and an ignored `_wording_version`.
- `KEYS` in `crates/thinkthen/src/core/declaration/parsing.rs` admits `wording_version`.
- Fixtures: `git grep -lE '"version" ?: ?1'` outside `sdlc/` lists about 200 files; most live under `crates/thinkthen/tests/`, `site/examples/`, `specification/fixtures/`, `conformance/` and the database suites.

The 2026-10-19 amendment below put every `wording_version` removal in this ticket. This ticket keeps the file and declaration side. The API, C interface, result and wrapper side moves to 0719, which depends on this ticket, so the carriers go after no file can fill them.

Collides with 0719, 0707 and 0709 on generated wrapper files and `conformance/cases.json`. Paths under `crates/thinkthen/src/cli/` and `crates/thinkthen/src/mcp/` move when 0639 lands; the claim follows the move.

## Amendment, 2026-10-19

Ian ruled that `wording_version` does not belong in the SDK: the SDK asks the question directly, and tracking wordings of a question belongs to the proxy, where question optimization will live. This ticket also deletes `wording_version` from question files, the public Rust API (`with_wording_version`), the C interface (`thinkthen_complete_wording_version_v1`), results, generated wrappers, the specification and fixtures. A question file that still carries it is refused with the same one-sentence rule as `version`.

## Amendment, 2026-10-19

The question-set, recognize-file and relate-file `version` removal moved to 0704, which finishes it once. 0706 keeps the configuration `schema`, the merged backend profile, `thinkthen.request/1` and `wording_version`.
