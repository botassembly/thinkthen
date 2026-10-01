# 0373: Windows stage 0: the root workspace and the C door compile and pass their tests on Windows

Status: in progress. Lane claude-3. Branch `ticket/0373-windows-stage-0`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian on 2026-10-01. It lands in two slices. Slice A lands the `windows` workflow file alone, so GitHub can dispatch it. Slice B lands the rest.

## Outcome

- The root workspace and `libraries/c` compile on `x86_64-pc-windows-msvc`. `cargo clippy --locked --all-targets -- -D warnings` passes for both, as `sdlc/scripts/lint` and `libraries/c/check.sh` run it on Linux.
- `cargo test --locked` passes on Windows for the root workspace and for `libraries/c`. Each test that cannot run on Windows carries `#[cfg(unix)]` and a comment that says why.
- The test children keep what Windows needs to run. One helper clears a child's environment, then puts back `SystemRoot`, `SystemDrive`, `TEMP` and `TMP` on Windows only. One helper sets the child's home: `HOME` everywhere, and `APPDATA` and `LOCALAPPDATA` under it on Windows only. On Linux and macOS each helper does exactly what the replaced line did.
- A new hand-started workflow, `.github/workflows/windows.yml` (`windows`), runs on `windows-2025`. It runs Clippy and the tests for the root workspace and for `libraries/c`. It starts only on `workflow_dispatch`, so it runs on any branch by `gh workflow run windows --ref BRANCH`. `release.yml` is unchanged.
- `sdlc/planning/windows.md` holds:
  - The staged plan, stages 0 to 3.
  - The surfaces that stay Unix-only, COBOL, Objective-C and Ada, and why.
  - The per-surface verdict table, updated with what stage 0 hit.
  - The decision record: Ian's ruling of 2026-10-01, which Ian can overturn.
  - The stage 1 difficulty report. For each of the seven stage 1 surfaces (command line, Rust crate, C DLL, Python wheel, Node addon, C#, JVM) it gives the files likely touched, a lines-changed range, the specification pages that change, the release-workflow changes, the new tests, the chance of breaking Linux or macOS with the reason, the main unknowns, and a ticket count. It ends with a recommendation: pull stage 1 into 0.1 or keep it for 0.2, with the trade-off.
  - The Windows findings stage 0 leaves open.
- Linux and macOS behavior is unchanged. Every product change sits behind `cfg(windows)` or `cfg(not(unix))`, or adds a Windows arm to a match that Unix never reaches.

## Evidence

- Starts from: main `91c3acec2`. No Windows build has ever been compiled. A read-only survey on 2026-10-01 found these gaps. A Windows-target type check on 2026-10-01 confirmed the compile ones.
  - `config.rs` `current_platform` returns `Linux` on Windows, so the default folders read `HOME` and `XDG_*`. ADR 0017 and ticket 0062 chose `%APPDATA%\thinkthen\config.json` and `%LOCALAPPDATA%\thinkthen\cache`. Ticket 0360 chose `%LOCALAPPDATA%\thinkthen\usage`.
  - `engine/usage.rs:377` `open_verified` opens the usage folder with `File::open`. `engine/usage/storage.rs` calls it and syncs the folder at lines 189 and 241. Windows refuses to open a folder without `FILE_FLAG_BACKUP_SEMANTICS`.
  - `cli/interrupt.rs:362` re-raises SIGINT through `signal_hook::low_level::emulate_default_handler`. On Windows it restores the default action and raises, and the C runtime's default SIGINT action ends the process with exit code 3. Exit 3 means "not sure".
  - The type check failed in the library: unused Unix-only items in `cli/interrupt.rs` and `engine/usage.rs`. It failed in the unit tests: the unguarded `nix` import and `SIGUSR1` in `engine/facade_tests.rs`, and unused imports in `cli/interrupt/tests.rs` and `engine/usage/no_lock_tests.rs`. Several integration tests assume `sh`, `script`, `/dev/zero`, `mkfifo`, `ulimit` or Unix modes.
  - `env_clear()` appears 56 times under `crates`, 8 times in `libraries/c` and once in `conformance/consumer`. Windows sockets fail to start without `SystemRoot`. `conformance/consumer` is outside the root workspace and out of scope.
  - The usage store's mode, owner and identity checks are all `cfg(unix)`, so on Windows it checks none of them. `config::writable_by_another` returns `false` off Unix. The usage refusal sentences say "folder 0700, files 0600", which means nothing on Windows.
  - `policy.py` and `deny.toml` know only the `cfg(unix)` dependency tables.
- Keeps: every Linux and macOS behavior, path, exit code and test. `release.yml`, the ladder scripts and their network rule. The dependency graph: stage 0 adds no crate, so `policy.py`, `deny.toml`'s license list and both locks stay as they are.
- Changes: product fixes behind Windows guards, test guards, two test helpers, one workflow and one plan page.
  - The three known behavior bugs are fixed if each fix is small and uses the standard library only:
    - `config.rs` gains `Platform::Windows`. On Windows the configuration file is `%APPDATA%\thinkthen\config.json`, the cache is `%LOCALAPPDATA%\thinkthen\cache` and usage is `%LOCALAPPDATA%\thinkthen\usage`. A relative or empty value gives no default folder, as on Linux. `absolute()` uses the host's rule, so a `C:\` path is absolute only on Windows. The Windows resolver rows therefore run in a `#[cfg(windows)]` test, and the Linux and macOS rows stay where they are.
    - The usage store opens its folder with `FILE_FLAG_BACKUP_SEMANTICS` through `std::os::windows::fs::OpenOptionsExt`, and skips the folder sync on Windows. Windows cannot flush a folder handle. `engine/store/convert.rs` already skips it the same way. Proof: the existing usage unit tests and the usage-totals library tests run on the runner and write, read and lock a real usage folder.
    - On Windows the command does not re-raise SIGINT. A cancelled run returns 130, as the Unix path does after its re-raise. Proof: a `#[cfg(windows)]` unit test installs the real emulation, fires the cancel, and gets 130 back in the same process.
  - A fix that needs a new crate, unsafe code, or a change Unix can reach is left as a finding in `windows.md`, with the test that shows it.
  - Tests that need a Unix tool or a Unix-only interface get `#[cfg(unix)]` with a reason. The `nix` import in `facade_tests.rs` moves under the same guard as its one test.
  - `src/test_deadline/child.rs` gains the environment and home helpers. The unit tests, `tests/backend`, `tests/library`, `public_controls`, `tests/polars` and the `libraries/c` door tests already include it by `#[path]`. The harnesses use the helpers in place of `env_clear()` and `.env("HOME", …)`.
  - `deny.toml`'s comment that Windows crates are never built is corrected.
  - `sdlc/planning/windows.md` is new.
  - `.github/workflows/windows.yml` is new:
    - Top-level `permissions: contents: read`. Checkout uses `persist-credentials: false`.
    - The same pinned `actions/checkout` and `actions/cache` SHAs as `gate.yml`.
    - The toolchain comes from `rust-toolchain.toml` (1.95.0, Clippy) through the runner's `rustup`.
    - `cargo fetch --locked` runs once for the root lock and once for `libraries/c`'s lock. Every build after it runs `--offline`, as the release workflow does.
    - `APPDATA` and `LOCALAPPDATA` point under the runner's temp folder, so no test writes the runner user's real folders.
- Proof: the Windows workflow and the Linux checks.
  - Slice A lands the workflow file alone, reviewed, with `workflow_dispatch` only, so `sdlc/scripts/workflows` passes. GitHub then dispatches the branch's own copy with `gh workflow run windows --ref ticket/0373-windows-stage-0`. No commit carries a `push` trigger.
  - The `windows` workflow passes on this branch's final code. The run URL and the commit it tested go in the record. After slice B lands, one dispatch on main confirms it there.
  - On Linux: the focused unit and integration tests for each touched file, `sdlc/scripts/lint`, and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`.
  - A Windows-target `cargo clippy` type check on Linux finds compile errors before each runner dispatch. It uses a private copy of the 1.95.0 toolchain in the builder's scratch folder with the Windows standard library added, so the shared toolchain is unchanged. Build scripts still run during a check, so a stub stands in for both the MSVC compiler and its archiver and writes empty outputs. Check never links. The runner stays the only proof that the code builds, links and passes.
  - The M5 runs only if the change reaches code macOS compiles in a way Linux does not. The builder records the call.
- Defers: everything that ships on Windows.
  - Stage 1 and later: shipping anything on Windows. By default it waits for 0.2, unless the report recommends otherwise and Ian pulls it into 0.1. The command, the Rust crate, the C DLL, the Python wheel, the Node addon, C# and the JVM bindings make up stage 1.
  - The other bindings and the three SQL extensions on Windows.
  - Specification and site pages: stage 0 promises users nothing, so `specification/recording.md` keeps its Linux and macOS folders only. Stage 1 adds the Windows lines.
  - A Windows Ctrl-C end-to-end test. Sending a console Ctrl-C to a child needs `GenerateConsoleCtrlEvent`, which the standard library does not expose.
  - Windows privacy checks for the usage store and the configuration file, and refusal sentences that make sense on Windows. `windows.md` carries them into stage 1.
