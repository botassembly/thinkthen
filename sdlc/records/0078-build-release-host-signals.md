# 0078: Build release host signals

Status: built on `ticket/0078-fork-and-host-signals` at `b7bf265e`, merged with main at `9f47bd18`. Code review pending: a fresh read-only Claude session reviews the final diff and the dependency change. Owner: Claude.

## Result

The recorder no longer registers a `SIGXFSZ` handler, and its `OnceLock` is gone. The engine reports write, sync, and install errors as before and never touches a signal disposition.

`cli/file_size.rs` holds the command's process-once `SIGXFSZ` claim. `cli::entry` calls it for every command that reads input, right after `--version` and `transform` return and before the environment, input, key lookup, or transport. Those are the commands that can open a recording or a cache write permit, because the default cache is on. `status` and `cache prune` do not claim it. A failed claim returns the fixed recording storage failure, exit 5, before anything is read or sent.

`engine::workers` blocks every signal on each engine worker for its lifetime, except the ones a thread raises by its own action: `SIGXFSZ`, `SIGPIPE`, `SIGSEGV`, `SIGBUS`, `SIGFPE`, `SIGILL`, `SIGTRAP`, and `SIGSYS`. The host's disposition still governs those. The calling thread keeps the host's mask.

`nix` (feature `signal`) is now a non-optional Unix dependency of the library. `signal-hook` is optional and the `cli` feature selects it. The tests add `signal-hook` and `nix` with its `pthread` feature as development dependencies. `Cargo.lock` and `deny.toml` did not change, because no crate entered or left the tree.

`policy.py` pins the new manifest tables, including the Unix development table. It refuses `signal_hook` in any engine production file, with a planted recorder registration that must fail and two controls that must pass. Its default-features-off graph check now reads `cargo tree -p thinkthen -e normal --no-default-features`. It fails if `clap`, `csv-core`, or `signal-hook` appears, or if `nix` is missing on Unix. No command-only path keeps `signal-hook` in the library.

`specification/recording.md` now says the command installs the handler and the library installs none.

## Planted bugs

- R6-3, worker mask removed: `engine::host_signal_tests::a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole` failed with `the normal answer: Transport(Other)`. It went red the same way before the mask existed.
- `SIGXFSZ`, the recorder registers the handler again: `a_host_with_the_default_sigxfsz_action_is_killed_by_it` failed with `left: None, right: Some(25)`. The child survived. It went red the same way on main's recorder before the move. `policy.py` refused the same plant.
- Command claim removed: `recording_durability::a_file_size_limit_returns_the_fixed_failure_and_removes_the_temporary_entry` failed with `left: None, right: Some(5)`. The command was killed by the signal.

## Acceptance

| Criterion | Proof |
| --- | --- |
| R6-3: a no-op `SIGUSR1` host handler, delivered by `pthread_kill` to an engine worker during a held send, leaves the normal answer and one send | `a_host_signal_during_a_held_send_on_a_worker_leaves_the_call_whole`. The worker publishes its thread ID through the existing `scoped_observed` begin hook. The listener reads the whole request, holds the reply, and afterwards has no second connection waiting |
| The command under `ulimit -f 1` exits 5, prints the fixed line, sends no extra request, and leaves no final or temporary entry | `a_file_size_limit_returns_the_fixed_failure_and_removes_the_temporary_entry`, which now also counts one request |
| A host `SIGXFSZ` action stays installed before, during, and after recording, and the engine reports the storage failure and cleans up | `a_host_sigxfsz_action_stays_installed_through_a_recording`. A child of the all-features test binary runs under `ulimit -f 1` with no command setup. The host flag is set by the real write's signal, then set again by a later `raise` |
| A host with the default action is killed by `SIGXFSZ` | `a_host_with_the_default_sigxfsz_action_is_killed_by_it` |
| `lint` checks the library graph | `policy.py` through `cargo tree`, as above |
| 0074 SIGINT, 0076 deadlines, and recording durability stay green | The full ladder below |

## What the ticket did not foresee

- `pthread_self` and `pthread_kill` need nix's `pthread` feature. The library keeps only `signal`, and the tests add `pthread` through a Unix development dependency. The reviewer checks it under the dependency rule.
- The mask leaves `SIGPIPE` and the fault signals open beside `SIGXFSZ`. Blocking a fault signal is undefined, and a thread raises each of these by its own action. Ian or the queue owner can overturn that list.
- The `opens` path `crates/thinkthen/tests/recording_durability.rs` now lives at `crates/thinkthen/tests/backend/recording_durability.rs`.
- No seam forces the claim to fail, so no test drives that path. The claim runs before `Environment::read`, input, key lookup, and transport by its position in `entry`.
- A failed recording leaves the `.locks` folder in place, as before. The host proof checks files only.

## Ladder

On `073a7e48`, `sdlc/scripts/install`, `lint`, `test`, and `spec` ran in order and each exited 0. `spec` ended with `demos: 21 green, 0 red`. The ratchet reads 48369 of 48369.
