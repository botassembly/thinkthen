# 0379: The root workspace tests pass on macOS

Status: COMPLETE.

Opened as: 2026-10-11. Lane claude-3. Branch `ticket/0379-macos-root-suite`. Plan: `sdlc/planning/cleanup-2026-09-30.md`. Asked by Ian's coordinator on 2026-10-01. It changes tests only, so it may land before the `release/0.1` cut (ADR 0116). A product bug found on macOS stops that case and goes to the coordinator; this ticket does not fix product code.

Milestone: 0.2

## Outcome

- `cargo test --locked --offline --workspace --all-targets` at the repository root passes on the M5 (macOS 26.4, arm64).
- Every case that failed on the M5 at the start is listed below with one of three verdicts:
  - (a) test assumption: the case now uses macOS's real folders or tools.
  - (b) Linux only: the case carries a `cfg` guard and a comment that says why.
  - (c) product bug: the case is left failing, the evidence is recorded here, and the coordinator gets a report.
  - (d) host setup: the run lacked a tool the gate requires. The case needs no change; the run supplies the tool.
- No case is skipped without a stated reason.
- The root tests still pass on Linux, and the `windows` workflow still passes on the branch.

## Evidence

- Starts from: the following evidence.
  - Ticket 0373 ran the root tests on the M5. 126 cases failed at its base and 62 at its branch. It said every remaining failure pinned Linux folders or Unix tools. No release check runs the root tests on macOS, so a real macOS bug could hide among them.
  - On 2026-10-01 the M5 ran `cargo test --locked --offline --workspace --all-targets --no-fail-fast` at main `55db6f59f`, under `sdlc/scripts/scratch.sh`'s `usage_guard`, with the pinned Rust 1.95.0. 62 cases failed: 1 unit test, 54 in `tests/backend` and 7 in `tests/library`. Two more `child_case` lines are the library harness's child runs of two of those 7.
  - The engine already has macOS folders. `config.rs` puts the cache at `~/Library/Caches/thinkthen`, the configuration file at `~/Library/Application Support/thinkthen/config.json`, and the usage totals at `~/Library/Application Support/thinkthen/usage`. macOS reads all three from `HOME` and ignores the XDG variables. The backend harness gives every spawn its own fresh `HOME`.
  - The failures fall into these groups. Each group's cause was checked on the M5.
    - 57 cases set `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` or `XDG_STATE_HOME`, or pin a Linux path under `HOME`, then read the folder back or run `status` in a second spawn. On macOS the command ignores the variable, and each spawn's fresh `HOME` holds its own totals, so the second spawn reads zero.
    - 2 cases connect to port 0 to get a refused connection: `engine::http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry` and `exchange::a_refused_port_fails_before_the_first_default_retry_wait`. Linux refuses port 0. macOS answers `EADDRNOTAVAIL`, which the engine reads as unreachable. On the M5 the command printed `thinkthen: the backend could not be reached; check --url and the network` and exited 4. It finished inside the case's 10-second bound, so it did not retry. No user connects to port 0, so this is a test assumption. A port that was bound and then freed is refused on both, and `public_batches.rs` already gets its refused port that way.
    - `terminal::a_terminal_is_told_what_the_command_waits_for_and_a_pipe_is_not`: macOS `script` sends its Ctrl-D ahead of the evidence still queued when its standard input closes. The command then reads an empty terminal. Typing the evidence and a Ctrl-D, and holding the pipe open until `script` exits, gives the same bytes on both. On the M5 and on Linux, `script` exits as soon as its child does.
    - `dry_run_terminal::the_role_hint_uses_stderr_only_when_stdout_is_a_terminal`: macOS discards a pseudo-terminal's unread output when its last child side closes. Linux keeps it. The script reads nothing after it closes its copy. Keeping the parent's copy open until the child exits, then reading what is waiting, works on both. With the parent's copy open the read never ends in `EIO`, so the script stops reading when `select` finds nothing waiting. The child has exited by then, so all its output is already waiting. The M5 confirmed this with a small script.
    - `demo_runner::a_page_whose_assertion_is_wrong_fails_the_run`: the run's `PATH` lacked `mustmatch`, which `sdlc/scripts/install` requires. The M5 has it in `~/.local/bin`. Verdict (d). This needs no change; the after run puts `~/.local/bin` on `PATH`.
- Keeps: every Linux test's checks, sentences, counts and exit codes. Every product file. The `cfg(unix)` guards and comments 0373 added for Windows; the Windows port of these cases stays finding W6.
- Changes: tests and one test helper.
  - `src/test_deadline/child.rs`, which every harness includes, gains `Folder`: `Config`, `Cache` or `Usage`. `Folder::X.variable(root)` gives the variable and value that put that folder under a case's root. On Linux that is the XDG variable naming `root/config`, `root/cache` or `root/state`. On macOS it is `HOME` naming the root. `Folder::X.under(root)` gives where the command then keeps it. Each XDG case replaces its variable and its `root.join("thinkthen")` with these two calls. One root then holds every folder on both platforms.
  - On macOS one `HOME` holds all three folders. A case that names only one folder on Linux keeps the other two in each spawn's fresh `HOME`. On macOS they now share the case's root across spawns. A later spawn could then get a cache answer where Linux sent a request. The rule: each case's Linux counts, sentences and exit codes must hold unchanged on macOS. Where sharing would change them, the case passes `--no-cache` only if the Linux run also does not use the cache, or gives the cache its own root, or takes verdict (b).
  - A case that blocks the usage folder blocks only the parent of `Folder::Usage.under(root)`, so the cache and the configuration stay usable and each case still tests one failure.
  - A passing case that names an XDG folder and checks that nothing appears there passes without testing anything on macOS. The builder checks each such case in the touched files and ports it the same way, so macOS tests it too.
  - The two port-0 cases take a freed port.
  - `terminal.rs` types a Ctrl-D and holds the pipe open. `dry_run_terminal.rs` keeps the parent's terminal side open until the child exits.
  - A case that cannot mean the same thing on macOS gets `#[cfg(target_os = "linux")]` and a reason. One known example sets `HOME` empty and reads the configuration from `XDG_CONFIG_HOME`. macOS has no configuration folder without `HOME`.
  - The list of every case and its verdict goes in "What the build taught us".
- Proof: the following runs.
  - M5: the same root test run at the branch's final code fails no case. The run puts the pinned Rust 1.95.0 and `~/.local/bin` on `PATH`, uses its own target folder in the builder's scratch folder, and takes `usage_guard`. The before and after counts go in the record.
  - Linux: the touched test binaries (`thinkthen` unit tests, `backend`, `library`), `sdlc/scripts/lint` and `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`.
  - Windows: the `windows` workflow dispatched on this branch passes. Its run URL goes in the record.
- Defers: the following.
  - The Windows port of the XDG cases (finding W6, stage 1). Their `cfg(unix)` guards stay. `Folder` has a Windows arm only because three helpers that run on Windows (`check.rs`, `secrecy.rs` and the named backends' support) now name their folders through it.
  - The doctests, the library-only run and the consumer run of `sdlc/scripts/test` on macOS. This ticket covers `--all-targets` at the root, as 0373 measured.
  - A macOS release check that runs these tests. Nothing in the release path runs them today.
  - The timing flake in `public_controls`'s throttle-gate case on the M5 (see below).

## What the build taught us

- Counts on the M5, root `cargo test --locked --offline --workspace --all-targets --no-fail-fast`:
  - Before, at main `55db6f59f`: 62 cases failed (1 unit, 54 backend, 7 library).
  - After, at the branch's final code: 0 failed. Unit 398, backend 749, library 66, loopback 21, public_batches 46, public_controls 23, public_cap 1, public_estimated 1 passed.
  - Backend runs 2 fewer cases than on Linux. One is the (b) case below. The other is `interrupt.rs`'s `RLIMIT_NPROC` case, which was already Linux only with a stated reason.
- Linux at the final code: unit 398, backend 751, library 66 passed. `sdlc/scripts/lint` and `policy.py` pass.
- Windows: the `windows` workflow passed at the final code `e41103b9b` (https://github.com/botassembly/thinkthen/actions/runs/36899801088). It also passed at `2693e7a56` and `865910f2d`, before the review answers.
- One M5 run at `e41103b9b` failed `public_controls::a_stop_at_the_throttle_gate_sends_nothing_new_and_sent_work_finishes`: only 2 of the 4 held sends arrived within its wait. This ticket does not touch that file. The case passed in 5 other full M5 runs, including the rerun at `e41103b9b`, and 8 times in 8 alone. It is a timing flake under load, and this ticket defers it.
- Verdicts: (a) 60, (b) 1, (c) 0, (d) 1. No product bug turned up. The engine's macOS folders behaved as `config.rs` states in every case.
  - (a), folder cases (57): every failing case in `default_cache` and its usage cases (12), `status` (11), `named_backends` (10), `facts` and its priced and lock cases (7), `cache_configuration` (3 of 4), `annotate::splitting`, `backoff`, `cache_partial`, `check`, `relate`, `resend`, `secrecy` (1 each), and the 7 library cases in `public_env`, its `batch`, `cache_budget` and `usage_totals` (the 8th library line, `child_case`, is their child half).
  - (a), port 0 (2): `engine::http::tests::a_refused_attempt_is_observed_once_and_returned_without_a_retry` and `exchange::a_refused_port_fails_before_the_first_default_retry_wait`.
  - (a), terminal (2): `terminal::a_terminal_is_told_what_the_command_waits_for_and_a_pipe_is_not` and `dry_run_terminal::the_role_hint_uses_stderr_only_when_stdout_is_a_terminal`.
  - (b) (1): `cache_configuration::configuration_supplies_address_model_and_cache_switch_without_a_home`. macOS reads the configuration file from `HOME` alone, so no configuration exists there without a home.
  - (d) (1): `demo_runner::a_page_whose_assertion_is_wrong_fails_the_run` passes once `~/.local/bin` is on `PATH`.
- Hidden failures: `interrupt::facts_flush::a_signal_during_usage_flush_still_marks_the_final_facts_line_stopped` passed in the before run by timing alone. Run by itself on the M5 it failed 5 times in 5, because the usage lock it holds sat in an XDG folder macOS ignores. It now uses `Folder`.
- Unexplained skip: `default_cache_storage` was guarded `target_os = "linux"` with no reason, so its 4 cases never ran on macOS. They now block the folder that holds the default cache, run on every Unix, and carry the reason (Unix folder modes).
- Vacuous passes ported, so macOS now tests them: `default_cache::...rejected_input`, the library's hand-built engine in `usage_totals`, `shared_host`, `check`'s scan of the cache, and the cache cases in `scheduling` and `annotate::scheduling`.
- `Folder::configure` writes a case's configuration file and returns its variable. Four test files share it (`public_env.rs`, `public_env/batch.rs`, `facts.rs` and `cache_configuration.rs`), and it keeps `public_env.rs` under its 500-line cap.
- The ratchet rises by 146 lines to 109411 for the helper and the ports.
- No case added `--no-cache`. The shared macOS home changed no Linux count.
