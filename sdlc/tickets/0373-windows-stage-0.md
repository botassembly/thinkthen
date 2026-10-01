# 0373: Windows stage 0: the root workspace and the C door compile and pass their tests on Windows

Status: in progress. Lane claude-3. Branch `ticket/0373-windows-stage-0`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian on 2026-10-01.

## Outcome

- The root workspace and `libraries/c` compile on `x86_64-pc-windows-msvc`. `cargo clippy --locked --all-targets -- -D warnings` passes for both, as `sdlc/scripts/lint` and `libraries/c/check.sh` run it on Linux.
- `cargo test --locked` passes on Windows for the root workspace and for `libraries/c`. Each test that cannot run on Windows carries `#[cfg(unix)]` and a comment that says why.
- The test children keep what Windows needs to run. One helper clears a child's environment, then puts back `SystemRoot`, `SystemDrive`, `TEMP` and `TMP` on Windows only. One helper sets the child's home: `HOME` everywhere, and `APPDATA` and `LOCALAPPDATA` under it on Windows only. On Linux and macOS each helper does exactly what the replaced line did.
- A new hand-started workflow, `.github/workflows/windows.yml` (`windows`), runs on `windows-2025`. It runs Clippy and the tests for the root workspace and for `libraries/c`. It starts only on `workflow_dispatch`, so it can run on any branch by `gh workflow run windows --ref BRANCH`. `release.yml` is unchanged.
- `sdlc/planning/windows.md` holds the staged plan, the per-surface verdict table, the decision record and the stage 1 difficulty report.
- Linux and macOS behavior is unchanged. Every product change sits behind `cfg(windows)` or `cfg(not(unix))`, or adds a Windows arm to a match that Unix never reaches.

## Evidence

- Starts from: main `91c3acec2`. No Windows build has ever been compiled. A read-only survey on 2026-10-01 found these gaps. The builder confirms each on the runner or by a Windows-target type check.
  - `config.rs` `current_platform` returns `Linux` on Windows, so the default folders read `HOME` and `XDG_*`. ADR 0017 and ticket 0062 chose `%APPDATA%\thinkthen\config.json` and `%LOCALAPPDATA%\thinkthen\cache`. Ticket 0360 chose `%LOCALAPPDATA%\thinkthen\usage`.
  - `engine/usage/storage.rs` opens the usage folder with `File::open` and syncs it. Windows refuses to open a folder that way.
  - `cli/interrupt.rs` re-raises SIGINT through `signal_hook::low_level::emulate_default_handler`. On Windows the C runtime's default SIGINT action ends the process with exit code 3, and 3 means "not sure".
  - `engine/facade_tests.rs` imports `nix` without a guard. Several tests assume `sh`, `script`, `/dev/zero`, `mkfifo`, `ulimit` or Unix modes. The harnesses call `env_clear()` in 57 places in the crate and 9 more in `libraries/c` and `conformance`. Windows sockets fail to start without `SystemRoot`.
  - `policy.py` and `deny.toml` know only the `cfg(unix)` dependency tables.
- Keeps: every Linux and macOS behavior, path, exit code and test. `release.yml`, the ladder scripts and their network rule. The dependency graph: stage 0 adds no crate, so `policy.py`, `deny.toml`'s license list and both locks stay as they are.
- Changes: product fixes behind Windows guards, test guards, two test helpers, one workflow and one plan page.
  - The three known behavior bugs are fixed if each fix is small and uses the standard library only:
    - `config.rs` gains `Platform::Windows`. On Windows the configuration file is `%APPDATA%\thinkthen\config.json`, the cache is `%LOCALAPPDATA%\thinkthen\cache` and usage is `%LOCALAPPDATA%\thinkthen\usage`. A relative or empty value gives no default folder, as on Linux. The resolver table test gains the Windows rows and runs on every platform.
    - The usage store opens its folder with `FILE_FLAG_BACKUP_SEMANTICS` through `std::os::windows::fs::OpenOptionsExt`, and skips the folder sync on Windows. Windows cannot flush a folder handle. `engine/store/convert.rs` already skips it the same way.
    - On Windows the command does not re-raise SIGINT. It returns 130, as the Unix path's exit status reads.
  - A fix that needs a new crate, unsafe code, or a change Unix can reach is left as a finding in `windows.md`, with the test that shows it.
  - Tests that need a Unix tool or a Unix-only interface get `#[cfg(unix)]` with a reason. The `nix` import in `facade_tests.rs` moves under the same guard as its one test.
  - `src/test_deadline/child.rs`, which the unit tests and both test binaries share, gains the environment and home helpers. The harnesses use them in place of `env_clear()` and `.env("HOME", …)`.
  - `deny.toml`'s comment that Windows crates are never built is corrected.
  - `sdlc/planning/windows.md` is new.
- Proof: the Windows workflow and the Linux checks.
  - The `windows` workflow passes on this branch. The run URL goes in the record.
  - On Linux: the focused unit and integration tests for each touched file, `sdlc/scripts/lint`, and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`.
  - A Windows-target `cargo clippy` type check on Linux (`--target x86_64-pc-windows-msvc`, with a stub C compiler, since check never links) finds the compile errors before each runner push.
  - The M5 runs only if the change reaches code macOS compiles in a way Linux does not. The builder records the call.
- Defers: everything that ships on Windows.
  - Stage 1 and later: shipping anything on Windows. The command, the Rust crate, the C DLL, the Python wheel, the Node addon, C# and the JVM bindings are assigned to 0.2. Ian may pull stage 1 into 0.1. `windows.md` sizes it.
  - The other bindings and the three SQL extensions on Windows.
  - Specification and site pages: stage 0 promises users nothing, so `specification/recording.md` keeps its Linux and macOS folders only. Stage 1 adds the Windows lines.
  - A Windows Ctrl-C end-to-end test. Sending a console Ctrl-C to a child needs `GenerateConsoleCtrlEvent`, which the standard library does not expose.

## Design notes

- The workflow uses the same pinned `actions/checkout` and `actions/cache` SHAs as `gate.yml`. It downloads crates with `cargo fetch --locked`, then builds `--offline`, as the release workflow does. It sets `APPDATA` and `LOCALAPPDATA` to a folder under the runner's temp folder, so no test writes the runner user's real folders.
- `sdlc/scripts/workflows` allows only `workflow_dispatch`. GitHub dispatches only a workflow whose file is on the default branch. Before landing, the branch's runs use a temporary `push` trigger limited to `ticket/0373-windows-stage-0`. The landed file has `workflow_dispatch` only, and the record says which commit each run tested. After landing, one dispatch on main confirms the landed file.
