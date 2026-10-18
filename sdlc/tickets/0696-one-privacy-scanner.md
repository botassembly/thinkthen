# 0696: Share one privacy scanner across the library checks

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision 79afa73, accept

## Outcome

- Changes: `conformance/children/native_guard.py`, `conformance/children/privacy.py`, `conformance/children/test_privacy.py`, `databases/duckdb/check.sh`, `databases/postgresql/check.sh`, `databases/sqlite/check.sh`, `libraries/ada/check.sh`, `libraries/ada/checks/generated_names.py`, `libraries/ada/checks/privacy.py`, `libraries/c/check.sh`, `libraries/cobol/check.sh`, `libraries/cobol/checks/generated_names.py`, `libraries/cobol/checks/privacy.py`, `libraries/cpp/check.sh`, `libraries/cpp/fixtures/guard.py`, `libraries/cpp/fixtures/installed_headers.py`, `libraries/cpp/fixtures/owned_calls.py`, `libraries/csharp/check.sh`, `libraries/csharp/tests/isolated_consumer.py`, `libraries/csharp/tests/package_check.py`, `libraries/dart/check.sh`, `libraries/dart/checks/installed_native_assets.py`, `libraries/dart/checks/privacy.py`, `libraries/go/check.sh`, `libraries/go/fixtures/guard.py`, `libraries/jvm/check.sh`, `libraries/jvm/tests/package_check.py`, `libraries/mcp/check.sh`, `libraries/objective-c/check.sh`, `libraries/php/check.sh`, `libraries/polars/check.sh`, `libraries/python/check.sh`, `libraries/r/check.sh`, `libraries/ruby/check.sh`, `libraries/rust/check.sh`, `libraries/swift/Tests/fixtures/guard.py`, `libraries/swift/check.sh`, `libraries/typescript/check.sh`, `libraries/zig/check.sh`, `sdlc/scripts/release-archive-self-test.py`, `sdlc/scripts/release-go-cpp-pair`, `sdlc/scripts/release-managed-pair.py`, `sdlc/scripts/test`.
- One privacy scanner lives in the shared test tools, and every library and database check calls it.
- Every copy is deleted. Any rule that only one copy had is kept in the shared scanner if it still guards something real, and dropped otherwise.

### Proof

- On the M5 through pm job, `sdlc/scripts/test` and each library gate that used a copy pass with the shared scanner.
- `git grep` finds one privacy scanner under `libraries/`, `databases/`, `conformance/` and `sdlc/scripts/`.

## Evidence

0564's review counted nine near-identical copies of a privacy scanner that checks public sources and packages for private text. Three are files named `checks/privacy.py` in `libraries/ada`, `libraries/cobol` and `libraries/dart`, and they already differ. The lane confirms the rest, starting with the fixture guards such as `libraries/cpp/fixtures/guard.py` and `libraries/go/fixtures/guard.py`. A fix to one copy does not reach the others. 0564 found the copies and planned one shared scanner with the first language lane; with the 0.2.1 phases, it belongs in the test machinery before any language work.

## Progress

- 2026-10-18 started
