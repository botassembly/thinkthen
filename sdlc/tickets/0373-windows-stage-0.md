# 0373: Windows stage 0: the root workspace and the C door compile and pass their tests on Windows

Status: landed. Lane claude-3. Branch `ticket/0373-windows-stage-0`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian on 2026-10-01. It lands in two slices. Slice A lands the `windows` workflow file alone, so GitHub can dispatch it. Slice B lands the rest.

Milestone: 0.2

## Outcome

- The root workspace and `libraries/c` compile on `x86_64-pc-windows-msvc`. `cargo clippy --locked --all-targets -- -D warnings` passes for both, as `sdlc/scripts/lint` and `libraries/c/check.sh` run it on Linux.
- `cargo test --locked --all-targets` passes on Windows for the root workspace, and `cargo test --locked` passes for `libraries/c`. Doctests wait for stage 1. Each test that cannot run on Windows carries `#[cfg(unix)]` and a comment that says why.
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
    - The usage store opens its folder with `FILE_FLAG_BACKUP_SEMANTICS` through `std::os::windows::fs::OpenOptionsExt`, and skips the folder sync on Windows. Windows cannot flush a folder handle. `engine/store/convert.rs` already skips it the same way. Proof: the usage unit tests in `engine/usage/tests.rs`, such as `concurrent_updates_keep_every_count_in_one_monthly_aggregate`, run on the runner and write, read and lock a real usage folder. The usage-totals library cases name the XDG folders, so they wait on finding W6.
    - On Windows the command does not re-raise SIGINT. A cancelled run returns 130, as the Unix path does after its re-raise. Proof: a `#[cfg(windows)]` unit test installs the real emulation, fires the cancel, and gets 130 back in the same process.
  - The runner found more Windows gaps, each fixed behind a Windows guard or in a test:
    - `engine/store/convert.rs` closes the live store before it removes the old file on Windows, because Windows cannot remove an open file. `cache convert` failed on Windows without it.
    - `--input` opens through the usage store's open helper, so on Windows a folder opens and gets the folder sentence in place of "Access is denied".
    - Clap names the program `thinkthen` on Windows. It printed `thinkthen.exe` in every usage and error sentence.
    - The conformance fixture puts each accepted connection in blocking mode. Windows and macOS hand an accepted socket the listener's nonblocking mode, and Linux does not. On Windows the fixture closed connections at random before a reply. On macOS the change makes the fixture read blocking, as it always did on Linux, so the M5 runs the fixture's and the root workspace's tests.
    - `libraries/c/build.rs` passes no `soname` or `install_name` on Windows. The C door tests build C with a Unix compiler under ASan, so they carry `#![cfg(unix)]`.
    - The workflow turns off Git's line-ending conversion, because the fixtures and goldens are byte-exact.
    - Tests that pinned an operating-system error sentence build it from the host's own error. The profile tests name their files without quotes and colons. The fixture checksum list compares `/`-separated names. A recording folder no command can make is `/dev/null/recording` on Unix and a name with `|` on Windows.
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

## What the build taught us

- Windows hands an accepted socket the listener's nonblocking mode. The conformance fixture's listener is nonblocking, so its peek of a request that had not arrived yet failed and the fixture closed the connection. On one runner attempt 174 backend cases failed at random with "the backend closed the connection before a reply". macOS does the same. On the M5 at the branch's code commit `439044dff`, the whole root workspace test run failed 62 cases, and every one of them also failed at the base commit `a7cd6c6a8`, where 126 failed. The 64 cases that pass now are the same random closes. The 62 that fail on both sides are cases that pin Linux folders or Unix tools, which nothing runs on the M5 today.
- A test fixture that answers before it reads the whole request can lose its answer on Windows. The socket closes with unread bytes, Windows sends a reset, and the reset can reach the client before the reply. The HTTP and deadline fixtures now read the whole request first through one helper, `test_deadline::whole_request`.
- Windows refuses quotes, colons and `|` in file names, and it reads `/dev/null/x` as a folder it can make on the current drive. The profile tests and the unmakeable recording folder needed Windows spellings.
- The runner's Git converts LF to CRLF on checkout, which broke byte-exact fixtures. The workflow turns the conversion off.
- GitHub dispatches only workflows whose file is on the default branch, so the ticket landed in two slices. Slice A landed the workflow file alone. A dispatch on main would have failed until slice B landed, because main's code did not yet build on Windows.
- Slice A's first version set `APPDATA` from the `runner` context in a job-level `env`, which GitHub refuses. GitHub recorded one zero-second failed `push` run for that file version; the file never had a `push` trigger. The fix writes both variables through `$GITHUB_ENV`.
- The first cache never saved, because the job failed and `actions/cache` saves only after success. Restore and save are now separate steps, and save runs after a failure too. `--keep-going` makes Clippy report every crate in one run.
- About forty integration cases name the XDG folders or a Linux path under `HOME`. Windows keeps the cache and the usage totals both under `LOCALAPPDATA`, so the XDG names do not map one to one. They carry `#[cfg(unix)]`, and finding W6 ports them in stage 1. The configuration file maps one to one, so the named-backend harness also sets `APPDATA` and runs on Windows.
- The M5 call: the change reaches macOS through the conformance fixture's accept, so the M5 ran the whole root workspace tests at the branch's code commit and at the base, as the first item records. Every other change is either Windows-only or the same code on macOS and Linux.
- The Windows-target type check on Linux caught most compile errors before a dispatch. It cannot see dead code that only the Windows test build leaves behind, so the runner found the unused helpers that the guards created.
- Proof run: the `windows` workflow passed at `d85e5e961` (https://github.com/botassembly/thinkthen/actions/runs/36880023738) after the rebase onto main at `4ec07dc2d` (https://github.com/botassembly/thinkthen/actions/runs/36884481940), and on the final code after the last code rebase at `8c3459f00` (https://github.com/botassembly/thinkthen/actions/runs/36891982772). Commits after that change only records. On Linux, after the two lint fixes below, `sdlc/scripts/lint` and `policy.py` passed at the final code. The `thinkthen` unit tests (398), `backend` (751), `library` (66), `public_controls` (23), the conformance loopback tests (22) and the `libraries/c` tests (37) passed. One dispatch on main after landing confirms it there.
- Full lint found two things the review could not see. `sdlc/scripts/children` read `.clear_environment()` as a child that inherits the environment, so it now accepts the helper and has a self-test case for it. `libraries/c/ratchet.json` rises by 8, to 4953 after the rebase, for the Windows arm in `build.rs` and the guard on the door's C tests.
- Reviews: the ticket review returned findings, then ACCEPT. Slice A's code review found the job-level `runner` context. Slice B's code review returned five findings: the macOS accept note, the usage proof line, the stage of each finding, two counts in `windows.md`, and the release files that hold the count of four. All five are fixed.

