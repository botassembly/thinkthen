# Quick Fix qf-heavy-lock: run the heavy rungs one at a time

Status: landed. On 2026-09-24 several agents ran full ladders and cold builds at once on the Beelink. The load hit 123 and 2 GB spilled into swap. Agents then wrapped heavy commands in `flock /tmp/thinkthen-heavy.lock` by hand. This Quick Fix makes the lock part of the rungs. A fresh read-only Opus reviewer returned ACCEPT with three low notes. Two were fixed, as listed below.

## Result

- `sdlc/scripts/heavy-lock` is the one shared helper. `install`, `test`, and `spec` source it right after `set -eu`, before they change directory. `lint` stays unlocked.
- The helper resolves the lock path from `THINKTHEN_HEAVY_LOCK`, or else `${XDG_RUNTIME_DIR:-/tmp}/thinkthen-heavy.lock`. It probes the lock with `flock -n`. When the lock is busy it prints one line, `<rung>: waiting for the heavy-build lock <path>`. It then re-runs the rung under `flock -o`. The lock is released when the rung exits. `-o` keeps the lock descriptor out of the rung and its children. A compiler cache server a build starts then cannot hold the lock.
- The holder exports `THINKTHEN_HEAVY_LOCK_HELD` set to the lock path. A rung or script called under it sees the same path and skips the lock.
- Without `flock` on `PATH`, as on macOS, the rung prints one line and runs unlocked.
- `sdlc/README.md` gains one sentence. `sdlc/scripts/README.md` gains a table row.

## The default path differs from the hand lock

On the Beelink `XDG_RUNTIME_DIR` is `/run/user/1000`. The default lock is `/run/user/1000/thinkthen-heavy.lock`, and the hand lock was `/tmp/thinkthen-heavy.lock`. The difference is deliberate. A hand `flock /tmp/thinkthen-heavy.lock sdlc/scripts/test` on the same path would wait forever on its own wrapper. On different paths an old hand wrapper is only redundant. Agents should stop wrapping the rungs by hand. Setting `THINKTHEN_HEAVY_LOCK=/tmp/thinkthen-heavy.lock` also works for anyone who drops the hand wrapper. Ian can overturn the default.

## Review notes

- Fixed: three trailing "so" clauses in the helper split into sentences.
- Fixed in a comment: killing only the `flock` process frees the lock while the rung runs on. Ctrl-C reaches both. This is the cost of `-o`.
- Kept: a probe that fails for a reason other than a busy lock, such as an unwritable directory, prints the waiting line before `flock` reports its own error and the rung fails.

## Proof

At `34c13660`, with a stub `cargo` first on `PATH`. The stub logs start and end, sleeps 2 seconds, and exits 7. The failing stub stops each rung after its first `cargo` call. `THINKTHEN_HEAVY_LOCK` pointed at a scratch file.

Two concurrent `sdlc/scripts/test` runs, B started 0.3 seconds after A:

```text
test: waiting for the heavy-build lock <scratch>/heavy/lock
16:20:22.906 start A
16:20:24.908 end   A
16:20:24.914 start B
16:20:26.917 end   B
```

Both exited 7, the stub's code, carried through `flock`. B started 6 ms after A ended.

Nested call: `sdlc/scripts/spec` under `timeout 20`, whose stub `cargo` ran `sdlc/scripts/test` while `spec` held the lock:

```text
16:20:26.938 start outer
16:20:26.945 start inner
16:20:28.948 end   inner
inner rung exit 7
16:20:30.953 end   outer
```

The inner rung started 7 ms after the outer one and did not wait. The outer `spec` exited 7, and `timeout` did not fire.

The default path: with `THINKTHEN_HEAVY_LOCK` unset and `/run/user/1000/thinkthen-heavy.lock` held by a hand `flock`, the stub `test` printed `test: waiting for the heavy-build lock /run/user/1000/thinkthen-heavy.lock`, then ran.

## Checks

With `THINKTHEN_API_KEY`, `THINKTHEN_BASE_URL`, and `THINKTHEN_HEAVY_LOCK` unset, at `34c13660`:

- `lint`: exit 0.
- `test`, once under the new default lock: exit 101. 1 test failed in the `backend` binary, 357 passed there, and every earlier binary passed. `secrecy::no_command_on_any_backend_path_writes_the_key_or_quotes_the_evidence` got "the backend refused the connection" for its relate case. The one-minute load was 120 from another session's processes. This change touches no Rust code. The failure is filed as `sdlc/issues/2026-09-24-the-secrecy-relate-case-fails-under-load.md`.
- `spec` and `install` did not run with real `cargo`. Their only change is the same sourced line.
- `sdlc/scripts/live` did not run.
