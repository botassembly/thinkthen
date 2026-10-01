# Windows

Status: stage 0 landed through ticket 0373. Stage 1 waits for 0.2 unless Ian pulls it into 0.1. Ian can overturn every stage, verdict and recommendation on this page.

This page is the one home of the Windows plan. It holds the stages, the surfaces that stay Unix-only, the verdict for each surface, the decision record, the stage 1 difficulty report and the findings stage 0 leaves open.

## Decision record

- 2026-10-01, Ian: start Windows stage 0 now. Stage 0 promises users nothing new and must not change Linux or macOS behavior. Stage 1 is assigned to 0.2. Ian may pull stage 1 into 0.1, so stage 0 also measures how hard stage 1 is. Ian can overturn this ruling.
- [ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md) item 6: if Ian pulls stage 1 into 0.1, it lands on main before the `release/0.1` cut, and the cut moves later.
- ADR 0017 and ticket 0062 chose `%APPDATA%\thinkthen\config.json` for the configuration file and `%LOCALAPPDATA%\thinkthen\cache` for the answer cache. Ticket 0360 chose `%LOCALAPPDATA%\thinkthen\usage` for the usage totals. Stage 0 implements these folders.
- Coordinator default, which Ian can overturn: the target is `x86_64-pc-windows-msvc`. The GNU target and ARM64 wait for stage 3.

## Stages

0. **Groundwork (ticket 0373, milestone 0.2, landed).** The root workspace and `libraries/c` compile, pass Clippy and pass their tests on `x86_64-pc-windows-msvc`. The hand-started `windows` workflow proves it on `windows-2025`. Nothing ships. No Linux or macOS behavior changes.
1. **First shipped surfaces (milestone 0.2).** The command line, the Rust crate, the C DLL, the Python wheel, the Node addon, C# and the JVM binding ship for Windows x86-64. The release workflow builds, tests and publishes them. The specification and the README name the Windows folders. Stage 1 also closes findings W1 to W7: the Ctrl-C exit, the Windows privacy checks, the port of the guarded XDG cases and the rest.
2. **The other bindings and the SQL extensions.** C++, Go, Ruby, R, PHP, Swift, Dart, Zig, the Polars door, and the SQLite and DuckDB extensions. Each loads the stage 1 C DLL or builds the engine as stage 1 does.
3. **Hardening.** Windows ARM64, finding W8, and the PostgreSQL question.

## Surfaces that stay Unix-only

- **COBOL, Objective-C and Ada.** Their release tools come from pinned Ubuntu apt snapshots (ticket 0273). On Windows each toolchain is a separate install with its own setup: GnuCOBOL through MSYS2 or a vendor compiler, Objective-C through GNUstep or Clang with a separate runtime, and GNAT through Alire or AdaCore. Each would need its own runner setup and release path, and few Windows users write these languages. They stay Linux and macOS only until a user asks.
- **PostgreSQL**, by default. The extension builds with `pgrx`. Stage 0 did not test `pgrx` on Windows, and the repo has no Windows PostgreSQL runner. It stays Unix-only until a user asks. Stage 3 checks what `pgrx` supports then.

## Verdict for each surface

"Stage 0 result" says what the runner showed. "Verdict" names the stage that ships it.

| Surface | Stage 0 result | Verdict |
| --- | --- | --- |
| Command line | Builds and passes on Windows. Six small product fixes landed. Findings W1 to W7 remain. | Stage 1 |
| Rust crate | Builds and passes on Windows as part of the root workspace. | Stage 1 |
| C door (`libraries/c`) | Builds, passes Clippy and passes its Rust tests. The C door tests build C with a Unix compiler under ASan, so they stay on Unix. | Stage 1 |
| Python wheel | Not built in stage 0. The wheel script expects a `.so`. | Stage 1 |
| Node addon | Not built in stage 0. The loader refuses `win32`. | Stage 1 |
| C# | Not built in stage 0. It imports `libthinkthen.so.0` by name. | Stage 1 |
| JVM | Not built in stage 0. It loads the library by a path property, so the code is name-agnostic. | Stage 1 |
| C++, Go, Ruby, R, PHP, Swift, Dart, Zig | Not built in stage 0. Each loads the C library. | Stage 2 |
| Polars door | Not built in stage 0. It follows the Python wheel. | Stage 2 |
| SQLite and DuckDB extensions | Not built in stage 0. | Stage 2 |
| PostgreSQL extension | Not built. | Unix-only by default |
| COBOL, Objective-C, Ada | Not built. | Unix-only |
| Windows ARM64 | Not built. | Stage 3 |

## What stage 0 changed

- `config.rs` gains `Platform::Windows`. The configuration file resolves under `APPDATA`, and the cache and usage under `LOCALAPPDATA`. A relative or empty value gives no default folder, as on Linux.
- The usage store opens its folder with `FILE_FLAG_BACKUP_SEMANTICS` and skips the folder sync on Windows, because Windows cannot flush a folder handle. `--input` opens through the same helper, so a folder named by `--input` gets the folder sentence.
- `cache convert` closes the live store before it removes the old file on Windows, because Windows cannot remove an open file.
- On Windows a cancelled run returns 130 without re-raising SIGINT. The C runtime's default SIGINT action would end the process with exit 3, which means "not sure".
- Clap names the program `thinkthen` in usage and error sentences on Windows, in place of `thinkthen.exe`.
- The conformance fixture puts each accepted connection in blocking mode. Windows and macOS hand an accepted socket the listener's nonblocking mode, and Linux does not. On macOS the fixture's accepted connections now read blocking, as they always did on Linux.
- Two test helpers replace `env_clear()` and `.env("HOME", …)`. On Windows they keep `SystemRoot`, `SystemDrive`, `TEMP` and `TMP`, and set `APPDATA` and `LOCALAPPDATA` under the test home. On Linux and macOS each does what the replaced line did.
- Test cases that need a Unix tool, a Unix mode, a Unix signal, the XDG folders or a Linux path carry `#[cfg(unix)]` and a reason.

## Findings stage 0 leaves open

W3, W4, W5 and W8 name the tests that show them. The others come from reading the code. Stage 1 owns W1 to W7. W8 waits for stage 3.

- **W1. Ctrl-C outside an active command exits 3.** `signal-hook`'s conditional default re-raises through the C runtime when no command holds the cancel, and the C runtime ends the process with exit 3. A cancelled command returns 130. The fix needs a console control handler, which the standard library does not expose; it needs `windows-sys` and a `policy.py` and `deny.toml` update. A Windows Ctrl-C end-to-end test needs `GenerateConsoleCtrlEvent` for the same reason.
- **W2. No privacy checks on Windows.** The usage store's mode, owner and identity checks are `cfg(unix)`, and `config::writable_by_another` returns `false` off Unix. The usage refusal sentences say "folder 0700, files 0600", which means nothing on Windows. A Windows check reads the folder's ACL and needs `windows-sys`.
- **W3. A refused loopback port reads as unreachable.** On Windows `exchange` prints "the backend could not be reached" where Linux prints "the backend refused the connection". Tests: `exchange::a_refused_port_fails_before_the_first_default_retry_wait` and `engine::http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry`.
- **W4. A closed output reader goes unnoticed.** On Unix the command polls standard output and stops scheduling when the reader closes. Windows has no such poll, so `filter` keeps working until a write fails. Tests: the two `closed_pipe` cases.
- **W5. The short loopback spelling `127.1` does not resolve on Windows.** The command is right to refuse it without a key, but the tests that prove the refusal cannot build the address. Tests: `address::a_key_that_is_unset_or_empty_is_exit_four_away_from_loopback` and `cli::conformance_tests::command::the_runner_hides_a_key_and_an_address_from_its_children`.
- **W6. The XDG-folder tests do not run on Windows.** About forty integration cases, guarded one by one or by page, set `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` or `XDG_STATE_HOME`, or pin a Linux path under `HOME`. Windows puts the cache and the usage both under `LOCALAPPDATA`, so the XDG names cannot map one to one. Stage 1 adds a test helper that names each default folder per platform and ports the cases.
- **W7. `clippy.toml`'s disallowed Unix paths warn "not reachable" on Windows.** The lint still passes; the warning is noise.
- **W8. The busy-parent fork proof hung on Windows.** `engine::facade::fork_tests::a_child_beside_a_busy_parent_uses_fresh_state` passed on one runner attempt and ran past its 60-second bound on two later ones. Windows has no `fork`, so the proof models a Unix case, and it carries `#[cfg(unix)]`. The cause is not known. Stage 3 looks again if a Windows binding ever builds an engine in a child process.

## Stage 1 difficulty report

Sizes count files, changed lines and tickets. They come from the stage 0 build and a read-only survey of the release path on 2026-10-01. Each surface assumes stage 0 has landed.

### Shared release work

Every stage 1 surface shares one release change. `release.yml` builds four targets, and the number four is written into `draft`, `verify-family`, the RubyGems platform-gem counts, the PyPI job's wheel count, `npm-assemble`'s addon count and the Homebrew tap. `sdlc/scripts/release-workflow` lists the four targets in its `collect` step's `expected_targets`. A fifth target touches each of them. The command line ticket carries this change, and the others build on it.

### Command line

- Files: `release.yml`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-pack` (471 lines of shell; it knows only `darwin` and `linux-gnu` and makes only `.tar.gz`), `release-registry.py`, `release-managed-pair.py`, a new PowerShell installer beside the 206-line `install.sh`, `README.md`, and the checks for each. The findings add `cli/interrupt.rs` (W1), the usage store and `config.rs` privacy checks (W2), and the guarded XDG cases (W6). W1 and W2 need `windows-sys`, so `policy.py` and `deny.toml` change too.
- Lines: 700 to 1,200.
- Specification: `specification/recording.md` (the cache, usage and configuration folders), `specification/settings.md` (the configuration home and the Unix-only warning), `specification/question-file.md` (the `XDG_CONFIG_HOME` example). The site pages that name platforms belong to marketing and get a message in `sdlc/inbox`.
- Release workflow: a fifth matrix target on `windows-2025`, a `.zip` pack, the count of four in each job above, and the installer upload.
- New tests: the W6 helper and the ported XDG cases; a Windows Ctrl-C end-to-end case (W1); the Windows privacy refusals (W2); a release-pack case for the `.zip`; an installer smoke on the runner.
- Linux and macOS risk: **medium**. The release scripts and the count of four are shared, so a mistake there breaks the Unix release.
- Unknowns: code signing for the `.exe` (SmartScreen warns on unsigned downloads), the installer's home (a `.ps1` script, winget or Scoop), and whether `managed-build` must cover Windows.
- Tickets: 3 to 4.

### Rust crate

- Files: the crate's `Cargo.toml` metadata and README, and its line in the release checks.
- Lines: 30 to 80.
- Specification: none beyond the command line's pages.
- Release workflow: none; crates.io takes source.
- New tests: none beyond the `windows` workflow, which already runs the crate's tests.
- Linux and macOS risk: **low**. Nothing shared changes.
- Unknowns: none of note.
- Tickets: folded into the command line ticket.

### C DLL

- Files: `libraries/c/build.rs` (the import library and the `.def` exports), `libraries/c/localize.sh` (it uses `nm`, `ld -r` and `objcopy`, which have no MSVC twins), the `.pc` file's `Libs.private`, `libraries/c/DESIGN.md`, the door tests (they use `cc`, ASan and `readelf`, and `fork.c`), `release-pack` and `release.yml`.
- Lines: 400 to 900.
- Specification: `libraries/c/DESIGN.md` gains the DLL, import library and static library rules.
- Release workflow: build `thinkthen.dll`, `thinkthen.dll.lib` and the static library on the Windows target; pack them with the header.
- New tests: an MSVC build of the door's C tests (`fork.c` stays Unix, and MSVC's ASan needs its own check), an export check with `dumpbin`, a static-link smoke.
- Linux and macOS risk: **low to medium**. `build.rs` and the pack script are shared, but each change sits in a Windows arm.
- Unknowns: how to hide the Rust standard library's symbols in a COFF static library, which `localize.sh` does with `objcopy` on ELF and Mach-O.
- Tickets: 2.

### Python wheel

- Files: `build-wheel.sh` (it expects a `.so`), the package's loader and classifiers, the tests that use `fork`, `SIGINT` and `resource`, `release.yml`'s wheel count.
- Lines: 150 to 350.
- Specification: no page changes. `libraries/python/README.md` names the Windows wheel.
- Release workflow: a `win_amd64` wheel; the PyPI job's count rises to five.
- New tests: wheel install and smoke on the runner; `cfg`-style skips for the Unix-only cases.
- Linux and macOS risk: **low to medium**. The wheel script and the count are shared.
- Unknowns: whether `cibuildwheel` or the present script builds the Windows wheel more simply.
- Tickets: 1 to 2.

### Node addon

- Files: `loader.js` (its `SHIPPED` list), `package.json` (`os`, `files`), the `check.sh` case that pins the `win32` refusal, the Node pin (Linux x64 only today), `npm-assemble`.
- Lines: 100 to 250.
- Specification: no page changes. `libraries/typescript/README.md` names Windows.
- Release workflow: a `win32-x64` addon; `npm-assemble` takes five.
- New tests: the refusal case becomes a load case on the runner.
- Linux and macOS risk: **medium**. One npm package carries every platform's addon, so a packing mistake breaks every platform.
- Unknowns: whether the addon needs the MSVC runtime beside it.
- Tickets: 1.

### C\#

- Files: `ThinkThen.cs` (line 45 imports `libthinkthen.so.0` by name), a native library resolver, the NuGet `runtimes/` layout, the tests that use `bwrap`, `flock` and `killpg`.
- Lines: 150 to 400.
- Specification: no page changes. `libraries/csharp/README.md` names Windows.
- Release workflow: a `runtimes/win-x64/native/thinkthen.dll` entry in the package.
- New tests: a load and smoke case on the runner; skips for the Unix-only cases.
- Linux and macOS risk: **low to medium**. A resolver replaces the fixed name, so it touches the Linux load path.
- Unknowns: whether `NativeLibrary.SetDllImportResolver` or the plain `runtimes/` probing is enough on every .NET version the binding supports.
- Tickets: 1.

### JVM

- Files: `check.sh` (it builds a `:` classpath and expects a `.so`), the jar's native resource layout, the tests that use `bwrap` and `pthread_self`. `Door.java` loads the library from `-Dthinkthen.library`, so it needs no change.
- Lines: 50 to 200.
- Specification: no page changes. `libraries/jvm/README.md` names Windows.
- Release workflow: the DLL in the jar's native resources.
- New tests: a load and smoke case on the runner; skips for the Unix-only cases.
- Linux and macOS risk: **low**. The loader is already name-agnostic.
- Unknowns: none of note.
- Tickets: 1.

### Totals and recommendation

| Surface | Lines | Tickets | Linux and macOS risk |
| --- | --- | --- | --- |
| Command line (with the shared release work and W1 to W7) | 700 to 1,200 | 3 to 4 | medium |
| Rust crate | 30 to 80 | 0 (in the command line ticket) | low |
| C DLL | 400 to 900 | 2 | low to medium |
| Python wheel | 150 to 350 | 1 to 2 | low to medium |
| Node addon | 100 to 250 | 1 | medium |
| C# | 150 to 400 | 1 | low to medium |
| JVM | 50 to 200 | 1 | low |
| **Total** | **1,600 to 3,400** | **9 to 11** | |

Recommendation: **keep stage 1 in 0.2.** Stage 1 touches the release workflow that 0.1 has not yet passed a rehearsal on, and the count of four runs through every publishing job. Pulling it into 0.1 moves the cut later, as ADR 0116 item 6 says, and adds risk to the release that matters most.

The option if Ian wants Windows in 0.1: pull only the command line and the Rust crate (3 to 4 tickets, 730 to 1,280 lines). They give Windows users the command and `cargo add thinkthen` without the binding packages, and they keep the release change to the pack script, the installer and the target count. The bindings follow in 0.2.
