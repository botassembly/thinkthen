# 0380: Windows stage 1: the command line and the Rust crate ship for Windows x86-64

Status: in progress. Slice A landed through `115bb95b20111f3f73688157584ff2f4736171ff` after fresh receipt review and all 350 actual site replays. Windows runner proof and slices B–D remain. Slice B design in `sdlc/records/0380-b-installer-design.md` passed fresh independent review at `fe0bf809c98789e33b25caf1fc5d497172b4305f`. The installer implementation and portable boundary proof are prepared in `sdlc/records/0380-b-installer-build.md`; fresh High source review accepted `046ed382b8`; the coordinator checkpoint passed after its source-preserving rebase. Final receipt review and native Windows proof remain pending. The `release/0.1` cut and ticket 0397 are landed. Fresh ticket review accepted the plan on 2026-10-04. Physical lane: claude-1. Plan: `sdlc/planning/windows.md`, stage 1. Shared workflow edits wait for the coordinator's signal; ticket 0398 slice C owns its workflow guard.

Milestone: 0.2

## Outcome

- The release workflow builds, tests, packs and publishes a fifth target, `x86_64-pc-windows-msvc`, on `windows-2025`. The command ships as a `.zip` with `thinkthen.exe`.
- The release jobs know a fifth target where the command ships: the matrix, `draft` and `expected_targets` in `sdlc/scripts/release-workflow`'s `collect` step. The PyPI wheel count and `npm-assemble`'s addon count stay at four; tickets 0382 and 0383 raise them. The RubyGems platform-gem count and the Homebrew tap skip Windows on purpose. The `collect` checks and `draft`'s per-target gate loop for the stage 2 and Unix-only surfaces (the go-cpp, swift-zig, php-dart, ada-objc-cobol and managed gates) skip the Windows target by name. `verify-family` keeps its counts of four.
- A Windows user installs the command with one documented step. The README names the Windows install and the Windows folders.
- `cargo add thinkthen` works on Windows. The crate's metadata and README name Windows.
- Findings W1 to W7 of `sdlc/planning/windows.md` are closed. Ctrl-C outside a command exits 130 (W1). The usage store and the configuration file get Windows privacy checks with Windows sentences (W2). A refused loopback port reads "refused" (W3). A closed output reader stops `filter` (W4). The `127.1` tests use an address Windows resolves (W5). The XDG cases run on Windows through a per-platform folder helper (W6). `clippy.toml` gives no "not reachable" warning on Windows (W7).
- Linux and macOS release files and behavior are unchanged.

## Evidence

- Starts from: ticket 0373 (stage 0), landed at `55db6f59f`. The root workspace builds and passes its tests on Windows under the hand-started `windows` workflow. Findings W1 to W8 and the stage 1 difficulty report live in `sdlc/planning/windows.md`.
  - The report sizes this ticket at 700 to 1,200 lines for the command line, with the shared release work and W1 to W7, plus 30 to 80 lines for the Rust crate. That is 730 to 1,280 lines and 3 to 4 slices. Linux and macOS risk is medium, because the release scripts and the count of four are shared.
  - `sdlc/scripts/release-pack` is 471 lines of shell. It knows only `darwin` and `linux-gnu` and makes only `.tar.gz`. `install.sh` is 206 lines.
  - W1 reuses the safe installed `signal-hook` shutdown API. W2 needs exact `windows-sys` 0.61.2 and confined native leaves enforced by `policy.py`. The current license policy already admits its licenses.
  - About forty integration cases carry XDG guards (W6).
  - No experiment preceded this ticket. Stage 0's runner results are the evidence.
- Keeps: every Linux and macOS release file, name, count and checksum. `release.yml` keeps its four existing jobs unchanged apart from the count. The folders chosen by ADR 0017, ticket 0062 and ticket 0360: `%APPDATA%\thinkthen\config.json`, `%LOCALAPPDATA%\thinkthen\cache` and `%LOCALAPPDATA%\thinkthen\usage`. A cancelled command still returns 130. The Unix privacy checks and sentences stay word for word.
- Changes: per slice, a suggested split the builder may refine.
  - Slice A, the fifth target: `release.yml`'s Windows matrix entry, the `.zip` pack in `release-pack`, the target lists named in the Outcome and the Windows skips, `release-registry.py`, `release-managed-pair.py` and `release-workflow`'s `expected_targets`. The Rust crate's metadata and README.
  - Slice B, the installer: `install.ps1` beside the unchanged Unix installer, its development README and site copy, real PowerShell boundary fixtures and a real packed-command installer smoke on `windows-2025`. The candidate contract, prior evidence, retained behavior, proof table and deferred gaps live in `sdlc/records/0380-b-installer-design.md`. The amended proof covers partial replacement states, verified snapshot retention and manual recovery, plus executable and receipt owner and access checks under a private parent, on PowerShell 5.1 and 7. Fresh independent design acceptance precedes code. Windows runner evidence remains pending an approved coordinator dispatch.
  - Slice C, W1 and W2: safe conditional shutdown registration in `cli/interrupt.rs`, native owner, access-list, reparse and identity checks for usage and configuration, and exact Windows dependency and unsafe-leaf enforcement in `policy.py`. The accepted design is `sdlc/records/0380-c-windows-design.md`. Native proof remains pending. Windows refusal sentences for the usage folder.
  - Slice D, W3 to W7: the refused-port sentence, the closed-reader stop, the `127.1` test addresses, the per-platform folder helper and the ported XDG cases, and the `clippy.toml` warning.
  - Specification: `specification/recording.md`, `specification/settings.md` and `specification/question-file.md` name the Windows folders. The queue owner also updates the site pages that name platforms, under the current ownership ruling.
- Proof: a rehearsal passes on all five targets with zero "not run" for the command. A `release-pack` case for the `.zip`. An installer smoke on `windows-2025`. A Windows Ctrl-C end-to-end case through `GenerateConsoleCtrlEvent` (W1). The Windows privacy refusals (W2). The W3 to W5 tests named in `windows.md` pass on Windows without a guard. The ported XDG cases pass on Windows and Linux (W6). Clippy on Windows prints no warning (W7). Each slice keeps the Linux and macOS release self-tests green.
- Defers: the main unknowns, each decided inside this ticket or escalated.
  - Code signing for `thinkthen.exe`. SmartScreen warns on unsigned downloads. The coordinator presented the signing options and costs. Ian authorized unsigned development implementation; public distribution awaits his signing ruling.
  - The installer's home: a `.ps1` script, winget or Scoop. The builder picks the simplest that works and records why.
  - Whether `managed-build` must cover Windows.
  - W8, the busy-parent fork proof, waits for stage 3.
  - Windows ARM64 and the GNU target wait for stage 3.

## Notes for the builder

- Slice A touches release files every target uses. Run the release self-tests and `workflows --self-test` before each push. A wrong count breaks the Unix release.
- Rehearsals need Ian's approval, as in `sdlc/planning/release-process.md` section 4.

## What the build taught us

- The archived release proof checks every source file against Git HEAD. Commit a changed packer before running this proof; copying an uncommitted packer into an archive correctly fails the source identity check.
- Windows command packaging needs a separate archive format and executable suffix. Keep those changes in the Windows arm, and leave Unix command tar names and payloads unchanged.

- Crate metadata changes the documentation proof identity even without Rust source changes. The full canonical replay refreshed 308 language and 42 SQL samples; strict verification reports all 350 current, and the final site build passes. Windows runner proof remains separate.

- Slice B's actual AST regressions must preserve saved replacement failures when inspection and diagnostics fail. A committed installation must still exit 0 after reporting fails; an uncommitted failure exits 1. Private original snapshots survive uncertain recovery. The complete local checkpoint is in [the slice B build record](../records/0380-b-installer-build.md); portable acceptance does not close native Windows proof.
