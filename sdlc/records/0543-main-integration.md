# Integrate the 0.2 release into main

Merge parents: main `f6eb6f30cb4a9e78026d4d574936c39de6111a18` and release `eba03cc0113f6bab13ab9efe1c46f31ab67b1685`. The 2026-10-17 PM ruling moves qualification to main. This merge resolves twenty-four paths.

- `conformance/c_parity.py`: retain the release's installed-session consumer and shared compiler output; discard the retired generated driver.
- `conformance/test_campaign_env.py`: retain the release's exact Python invocation checks and current DuckDB source-host fixture; retain main's active Python interpreter, resolved scratch root and Linux host stand-ins.
- `conformance/test_candidate_campaign.py`: retain the release's actual generated package and archive checks plus main's Linux host stand-in.
- `databases/duckdb/ratchet.py.json`: retain the release's handwritten production scope and community directory.
- `databases/duckdb/tools/build_version_cases.py`: retain release deletion; source-host fixtures replace the retired driver.
- `libraries/ada/check.sh`, `libraries/cpp/check.sh`, `libraries/go/check.sh`, `libraries/php/check.sh`: retain release package checks and main's deletion of binding build locks. Remove associated compiler wrappers. Preserve the same removal in the automatically merged Swift and Zig scripts.
- `sdlc/ratchet.json`: retain the reviewed release ceiling for the accepted 0.2 product changes.
- `sdlc/scripts/README.md`, `sdlc/scripts/heavy-lock`: retain main's ordinary rung sanitization and PM admission. Existing release qualification consumers still use their existing internal compiler holds; no new lock implementation or environment passdown is added.
- `sdlc/scripts/coverage-checks.py`: retain release coverage and C++ package checks with main's unlocked Ada collector and Linux-only host guard.
- `sdlc/scripts/lint`: retain main's portable environment sanitization fixture; include the release's hosted runner markers without restoring retired lock variables.
- `sdlc/scripts/release-process-cleanup-test.py`: retain all release wrapper cleanup cases and main's direct interpreter invocation for the POSIX executable stand-in.
- `sdlc/scripts/surfaces`, `sdlc/scripts/test`, `sdlc/scripts/test-full-cases`: retain main's ordinary admission ownership and macOS filename skip with the release's expanded campaign.
- `sdlc/scripts/workflows`: retain the release's authorized reusable DuckDB call and fixtures, main's checksum tool stand-in and fixture output, and manual-only gate dispatch.
- `sdlc/tickets/0543-qualify-0-2-candidate.md`, `sdlc/tickets/0560-measure-coverage-on-every-surface.md`, `sdlc/tickets/0579-live-backend-error-handling.md`, `sdlc/tickets/0590-typesafe-jev-images.md`: preserve both parents' existing entries; add no ticket status.
- `specification/fixtures/types/self-test`: retain release schema-only validation; installed session cases own C execution.

Main's M5 tickets gate, jsonschema dependency and Linux-only collector guards remain. Application, usage, cache and live-budget locks are unchanged by the resolutions. Checks on the build machine belong to the combined pushed revision.

Repair merge parents: integration `1ef31c293c445e6b9ba37689355ba72be262b25c` and accepted repair batch `6c996f438d99d60d54ac9dd5496d5242d8f8452b`. Resolve `release-language-tools-self-test.py` with the accepted isolated Python package-provider probe; resolve `workflows` with the accepted refusal of checksum execution and owned temporary roots; preserve both existing review lists in ticket 0543. The batch retains Windows Kotlin argument quoting, approved DuckDB source identifiers, prepared-input preflight, PostgreSQL phase diagnostics and macOS archive metadata exclusion.
