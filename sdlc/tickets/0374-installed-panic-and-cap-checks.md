# 0374: The installed-file checks prove panic isolation and the token cap

Status: in progress. Lane claude-1. Branch `ticket/0374-installed-panic-and-cap-checks`. Plan: `sdlc/planning/cleanup-2026-09-30.md`.

Milestone: 0.1

## Outcome

Every installed-file check that a ticket's open package proof names runs two more cases against the file a user installs.

1. A panic-isolation case. The shipped native library links no Rust standard library and imports no panic symbol, so its panic hook state stays inside that library.
2. A token-cap case. With `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10`, one call is refused before it sends. The case pins the surface's exact Usage sentence or exit code, and a counting loopback backend reads 0 requests.

`release-smoke` runs these installed-file modes on each release target, both in `surfaces` checkpoints and in the release workflow's smoke jobs. A clean rehearsal then carries both cases on all four targets. R has no file in the release workflow, so its cases run only in the `surfaces` sweep (see Defers).

## Why a panic cannot be injected in an installed package

Every panic trigger the source proofs use sits behind `#[cfg(test)]`: the child tests selected by `THINKTHEN_C_PANIC_CHILD`, `THINKTHEN_TEST_RUBY_PANIC_CHILD`, `THINKTHEN_TEST_R_PANIC_CHILD`, `THINKTHEN_TEST_NODE_PANIC_CHILD`, `THINKTHEN_PYTHON_PANIC_CHILD` (behind the `probe` feature), the SQLite worker child and the DuckDB bridge child. PostgreSQL's `panic-probe` feature builds a separate test library. `release-smoke`'s `scan` fails any shipped file that holds `thinkthen_panic_probe`, and DuckDB's ticket 0110 check forbids its `test-hooks` feature in a shipped build. So a release build has no route to a panic by design, and this ticket adds none.

Tickets 0226 and 0227 proved their Linux packages the same way. The 0227 build record says so: "The selected package proof checks that path and the loaded library mapping, not a synthetic panic in a distributed package." The 0226 build record inspected the shipped C and SQLite libraries for local `std::panicking::HOOK` state and no separately linked Rust runtime. Neither ticket ran a macOS or ARM64 package. This ticket turns those manual Linux inspections into a check that runs on each target.

## Evidence

- Starts from: main `a7cd6c6a8`.
  - Ticket 0226 keeps open "macOS C/SQLite and retained DuckDB Linux ARM64/macOS installed-package proof". Its outcome asks to inspect each artifact's "hook symbol/linkage".
  - Ticket 0227 keeps open "installed-package proof for the remaining targets": "an installed wheel, pinned Ruby gem, Node package and R tarball must separately prove actual host loading and later use, with linkage/lifetime checked per shipped target."
  - Ticket 0299 keeps open "installed package and actual release runner qualification". Ticket 0311 added one loopback token-variable case per surface, in source-tree tests: C `libraries/c/tests/door/settings.rs`, Python `tests/test_call.py`, R `tests/facts.R`, Ruby `tests/test_engine_settings.rb`, TypeScript `tests/settings.test.mjs`, SQLite `tests/test_settings.py`, DuckDB `tools/settings_suite.py`, PostgreSQL `check.sh` step `token_variable_refuses_before_sending`, and the command in `crates/thinkthen/tests/backend/limits.rs`.
  - No installed-file mode runs any of those token cases, and none runs a linkage check. `release-smoke` runs the installed-file modes of `libraries/c`, `databases/sqlite`, `databases/duckdb`, `databases/postgresql`, `libraries/python`, `libraries/typescript`, `libraries/ruby` and the command. The release workflow's `smoke` job runs `release-smoke` on all four targets. The build jobs only pack, so no source-tree test runs on ARM64 Linux or either Mac.
  - The brief named C, DuckDB, Ruby and R. Tickets 0226 and 0227 also name SQLite, Python and TypeScript, and 0299 names every host, so those join. The 0226 "retained DuckDB C API" packages no longer ship; the DuckDB archive on every target is now the C++ extension, so its check covers that file.
  - On this host the release-profile C, SQLite, DuckDB, Ruby and R libraries each list only system libraries as `NEEDED`, import only `_Unwind_*` and host symbols, and hold a local `std::panicking::HOOK`.
- Keeps: every installed-file step that runs now, the rung's own backend for the existing steps, `release-smoke`'s scan and its rule that "not run" fails a release, the source-tree panic children and token tests unchanged, and no test hook in any shipped build.
- Changes: one shared helper, two cases in each named installed-file mode, and one command case.
  - `sdlc/scripts/installed.sh` gains `own_panic_hook LIBRARY`. On Linux it reads `NEEDED` with `readelf -d` and the imported symbols with `nm -D --undefined-only`. On macOS it reads `otool -L` and `nm -u`. It fails when a dependency is a Rust standard library (`libstd-`) or when an imported symbol names `panic`. A missing tool reports "not run".
  - `libraries/c/check.sh`, installed mode: `own_panic_hook` on the archive's shared library. The existing driver `tests/c/driver.c` builds against the archive through its own pkg-config file. One `decide` request runs with the variable set, against a backend the check starts with `backend_start`. The reply is code `1` and the sentence `max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request`. The backend counts 0.
  - `databases/sqlite/check.sh`, installed mode: `own_panic_hook` on `libthinkthen0`, and the selected tests gain `test_environment_token_cap_refuses_before_any_send`.
  - `databases/duckdb/check.sh`, installed mode: `own_panic_hook` on the extension, and the `settings_suite.py` selection gains `environment_token_cap_refuses_before_any_send`.
  - `databases/postgresql/check.sh`, installed mode: the step list gains `token_variable_refuses_before_sending`. PostgreSQL gets no linkage case, since no panic ticket names it.
  - `libraries/python/check.sh`, installed mode: `own_panic_hook` on the wheel's extension module, and pytest runs `tests/test_call.py::test_token_cap_variable_refuses_before_any_send` in the fresh venv.
  - `libraries/typescript/check.sh`, installed mode: `own_panic_hook` on the installed addon, and `node --test` runs `tests/settings.test.mjs` filtered to its token-cap test.
  - `libraries/ruby/check.sh`, installed mode: `own_panic_hook` on the gem's extension, and minitest runs `tests/test_engine_settings.rb` filtered to `test_the_token_cap_variable_refuses_before_any_request`.
  - `libraries/r/check.sh`, tarball step: `own_panic_hook` on the tarball install's `libs/thinkthen.so`, and `tests/facts.R` runs against that install as well as the source install. Its token check pins the `usage` kind and the sentence with 0 sends.
  - `sdlc/scripts/release-smoke` `command_check`: the unpacked command runs `decide "Is it?" --no-cache` against the loopback backend with the variable set. It pins exit 2, stderr `thinkthen usage: max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request`, and 0 new requests. This covers both the ordinary run and `release-smoke --command`.
  - `sdlc/scripts/README.md` notes the two cases under `release-smoke`.
  - Tickets 0226, 0227 and 0299 say their installed proof now comes from this ticket plus the next clean rehearsal.
- Proof: real installed runs on freshly packed Linux files, plants, and the repo checks.
  - Pack fresh Linux x86-64 files with `release-pack` from the branch, as `surfaces` does, and run `release-smoke` on them. Each changed installed-file mode passes, and its log shows the two new cases.
  - Run `libraries/r/check.sh` with its tarball step.
  - Plants, each run once and reverted: a planted `NEEDED` of `libstd-x.so` and a planted imported panic symbol each fail `own_panic_hook`; a token sentence changed by one word fails each surface's case; setting the variable to `100000` makes the counted backend read 1 and fails the C case.
  - `sdlc/scripts/lint` in full and `sdlc/scripts/tickets`. No Rust source changes, so `policy.py` is not needed; it runs if Rust changes.
  - No checkpoint is published and no paid call runs.
- Defers: four gaps stay.
  - The payload-secrecy behavior itself on ARM64 Linux and both Macs. Installed release builds cannot inject a panic, and the release workflow runs no source-tree test on those targets. The honest route is to run each binding's existing panic child (`cargo test` of the named diagnostics test) on each target. That needs a source build per binding in the smoke job or a new job, which costs runner time. The coordinator decides whether to add it.
  - R's two cases on the other targets. The release workflow packs no R file; R-universe builds it from source. A target R proof needs an R step in the release workflow.
  - macOS runs of `own_panic_hook` first happen on the rehearsal. This host cannot run `otool`.
  - The source wrappers over C (Go, C++, C#, JVM, Swift, Zig, PHP, Dart, Ada, Objective-C, COBOL). They load the same C library, which the C case checks. Ticket 0299 rules out repeating the case per wrapper.
