# 0379: The root workspace tests pass on macOS

Status: in progress. Lane claude-3. Branch `ticket/0379-macos-root-suite`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian's coordinator on 2026-10-01. It changes tests only, so it may land before the `release/0.1` cut (ADR 0116). A product bug found on macOS stops that case and goes to the coordinator; this ticket does not fix product code.

Milestone: 0.2

## Outcome

- `cargo test --locked --offline --workspace --all-targets` at the repository root passes on the M5 (macOS 26.4, arm64).
- Every case that failed on the M5 at the start is listed below with one of three verdicts:
  - (a) test assumption: the case now uses macOS's real folders or tools.
  - (b) Linux only: the case carries a `cfg` guard and a comment that says why.
  - (c) product bug: the case is left failing, the evidence is recorded here, and the coordinator gets a report.
- No case is skipped without a stated reason.
- The root tests still pass on Linux, and the `windows` workflow still passes on the branch.

## Evidence

- Starts from: the following evidence.
  - Ticket 0373 ran the root tests on the M5. 126 cases failed at its base and 62 at its branch. It said every remaining failure pinned Linux folders or Unix tools. No release check runs the root tests on macOS, so a real macOS bug could hide among them.
  - On 2026-10-01 the M5 ran `cargo test --locked --offline --workspace --all-targets --no-fail-fast` at main `55db6f59f`, under `sdlc/scripts/scratch.sh`'s `usage_guard`, with the pinned Rust 1.95.0. 62 cases failed: 1 unit test, 54 in `tests/backend` and 7 in `tests/library`. Two more `child_case` lines are the library harness's child runs of two of those 7.
  - The engine already has macOS folders. `config.rs` puts the cache at `~/Library/Caches/thinkthen`, the configuration file at `~/Library/Application Support/thinkthen/config.json`, and the usage totals at `~/Library/Application Support/thinkthen/usage`. macOS reads all three from `HOME` and ignores the XDG variables. The backend harness gives every spawn its own fresh `HOME`.
  - The failures fall into these groups. Each group's cause was checked on the M5.
    - 57 cases set `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` or `XDG_STATE_HOME`, or pin a Linux path under `HOME`, then read the folder back or run `status` in a second spawn. On macOS the command ignores the variable, and each spawn's fresh `HOME` holds its own totals, so the second spawn reads zero.
    - 2 cases connect to port 0 to get a refused connection: `engine::http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry` and `exchange::a_refused_port_fails_before_the_first_default_retry_wait`. Linux refuses port 0. macOS answers `EADDRNOTAVAIL`, which the engine rightly reads as unreachable. A port that was bound and then freed is refused on both, and `public_batches.rs` already gets its refused port that way.
    - `terminal::a_terminal_is_told_what_the_command_waits_for_and_a_pipe_is_not`: macOS `script` sends its Ctrl-D ahead of the evidence still queued when its standard input closes. The command then reads an empty terminal. Typing the evidence and a Ctrl-D, and holding the pipe open until `script` exits, gives the same bytes on both. On the M5 and on Linux, `script` exits as soon as its child does.
    - `dry_run_terminal::the_role_hint_uses_stderr_only_when_stdout_is_a_terminal`: macOS discards a pseudo-terminal's unread output when its last child side closes. Linux keeps it. The script reads nothing after it closes its copy. Keeping the parent's copy open until the child exits, then reading what is waiting, works on both.
    - `demo_runner::a_page_whose_assertion_is_wrong_fails_the_run`: the run's `PATH` lacked `mustmatch`, which `sdlc/scripts/install` requires. The M5 has it in `~/.local/bin`. This needs no change; the after run puts it on `PATH`.
- Keeps: every Linux test's checks, sentences, counts and exit codes. Every product file. The `cfg(unix)` guards and comments 0373 added for Windows; the Windows port of these cases stays finding W6.
- Changes: tests and one test helper.
  - `src/test_deadline/child.rs`, which every harness includes, gains `Folder`: `Config`, `Cache` or `Usage`. `Folder::X.variable(root)` gives the variable and value that put that folder under a case's root. On Linux that is the XDG variable naming `root/config`, `root/cache` or `root/state`. On macOS it is `HOME` naming the root. `Folder::X.under(root)` gives where the command then keeps it. Each XDG case replaces its variable and its `root.join("thinkthen")` with these two calls. One root then holds every folder on both platforms.
  - The two port-0 cases take a freed port.
  - `terminal.rs` types a Ctrl-D and holds the pipe open. `dry_run_terminal.rs` keeps the parent's terminal side open until the child exits.
  - A case that cannot mean the same thing on macOS gets `#[cfg(target_os = "linux")]` and a reason. One known example sets `HOME` empty and reads the configuration from `XDG_CONFIG_HOME`. macOS has no configuration folder without `HOME`.
  - The list of every case and its verdict goes in "What the build taught us".
- Proof: the following runs.
  - M5: the same root test run at the branch's final code fails no case. The before and after counts go in the record.
  - Linux: the touched test binaries (`thinkthen` unit tests, `backend`, `library`), `sdlc/scripts/lint` and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`.
  - Windows: the `windows` workflow dispatched on this branch passes. Its run URL goes in the record.
- Defers: the following.
  - The Windows port of the XDG cases (finding W6, stage 1). `Folder` has no Windows arm until then.
  - The doctests, the library-only run and the consumer run of `sdlc/scripts/test` on macOS. This ticket covers `--all-targets` at the root, as 0373 measured.
  - A macOS release check that runs these tests. Nothing in the release path runs them today.
