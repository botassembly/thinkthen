# Engine and first-binding design re-review: 0085, 0096, 0086, 0098, 0093, 0094

Reviewer: fresh read-only Claude session, 2026-09-24. Read: workspace `CLAUDE.md` (Tickets), repo `CLAUDE.md`, main `860086d2` (`one-line-plan-2026-09-24.md`, `surfaces-port-guide.md`, `Cargo.toml`, `sdlc/ratchet.json`, `sdlc/scripts/ratchet.mjs`, `sdlc/scripts/test`), branches 0085 `950d6cbb`, 0096 `943c88d0`, 0086 `d995b6a5`, 0098 `a8ddc49c`, 0093 `1e911ae1` (ADR 0047), 0094 `a07f0d0f`, the prior reviews, and tag `surfaces-wave7-final` (header, `libraries/c/src/lib.rs` envelope, churn issue, `standin/tests/churn_probe.rs`). 0084, 0095, and 0097 were read as input only. Nothing was built, run, or edited.

## Verdicts

- 0085 facade: REJECT. The prior findings are resolved (interrupt check split to 0097, fork to 0096, 0092 ordered, Claude routing, planted bugs named). The smaller scope now fits the unchanged budget. Three one-sentence fixes remain.
- 0096 fork recovery: REJECT. The seam design is right. The proof has no timeout, and the design lives only in another branch's commit.
- 0086 public API: REJECT. F4, F5, F7, and F8 are resolved, and the budget fits after the 0098 split. Two proofs cannot be built as written, and the churn probe compares two different programs.
- 0098 binding members: ACCEPT. It has bounded scope, reuses existing owners, and has planted bugs. The JSON methods are checked byte for byte against the command.
- 0093 Rust examples and ADR 0047: REJECT. The ratchet plan does not work with the shared reader. Separate workspaces drop the root release profile and the root lock.
- 0094 C interface: REJECT. The header table is still deferred, and relate's argument meaning is undecided. The envelope list ends in "and the rest". The churn probe source is not in any repository.

## 0085

1. R5-19's planted bug may not turn its test red. "Returns before the worker join" adds no send if workers check the token before dispatch. The next-call count then stays right. Smallest change: plant "a worker dispatches one more queued item after the cancel". Or state that the test holds one reply across the cancel and counts sends after releasing it.
2. The G4 bullet tests `Row::probability`, a public method that 0098 adds later. Smallest change: say "the private bulk row carries the yes probability that details returns."
3. Stale fork text. "Repairs state after fork" (Boundary), "fork owners" (Allowed), and "fork recovery" (Reasons) name an owner that does not exist until 0096. Smallest change: delete those three words or point them at 0096's accessor.

## 0096

1. The lock proofs cannot fail cleanly. "The watchdog fails it" names nothing. `sdlc/scripts/test` runs plain `cargo test`, which has no per-test timeout, so a lock-first bug hangs the gate. Smallest change: run each held-lock case on a spawned thread and fail on `recv_timeout` (the pattern in `annotate_schedule.rs:465`).
2. The design exists only by reference to `01d82781` on the 0078 branch, which 0078 has since narrowed. The repo rule says a changed proposal is written again whole. Smallest change: copy the two cited sections into this ticket.
3. `opens` omits root `Cargo.lock` and `deny.toml`, which the `arc-swap` allowance changes. Add them.
4. The plan orders 0085, then 0097, then 0096. Both 0097 and 0096 edit call entry. Smallest change: say "after 0097".

## 0086

1. The ticket contradicts itself. Excluded lists "FFI symbols or `unsafe`", but acceptance gives the consumer crate an `unsafe` fork. Smallest change: exempt that one call by name in Excluded. Say where the consumer crate lives and whether it is a root member. A root member inherits `unsafe_code = "forbid"`, which `allow` cannot lift, so it needs its own lint table.
2. The R4-24 test cannot be written in-crate. It changes an environment variable after build, and `std::env::set_var` is `unsafe` in edition 2024 under a forbid. Smallest change: spawn a child process with `THINKTHEN_BASE_URL` set, build the engine there with a different `base_url`, and count sends on each listener.
3. The churn probe compares two programs. The crash reproduced only through the tag's C door, driven from C threads. The tag's Rust `churn_probe.rs` has a different shape and never crashed. Main has no C door at 0086, so "tag C door, then main" does not compare like with like. Smallest change: run one Rust probe through the stand-in's Rust API on the tag, then through the public API on main, with the same load. If the Rust probe does not reproduce the crash on the tag in 300 runs, record that and leave G3 open for 0094. Do not claim a fix.
4. `opens` omits `probes/` (the churn probe) and `Cargo.lock` (the compile-test dev-dependency). Add them.
5. Small edits. The Dependencies paragraph still says "0078 completes fork recovery"; change it to 0096. The `deadline_seconds`/`deadline_millis` bullet appears twice. Counting "the external consumer crate as one file" hides its files. Count its Rust files and lines.

## 0093 and ADR 0047

1. ADR item 4 cannot work "unchanged". `ratchet.mjs` resolves `sdlc/ratchet.json` from its own location and never from the working directory, so it cannot read `libraries/<host>/ratchet.json`. Smallest change: give the shared reader an optional config-path argument (and note the other repos that share it). Or have ADR item 4 say each binding carries `sdlc/scripts/ratchet.mjs` and `sdlc/ratchet.json` under its own folder.
2. A separate workspace drops the root `[profile.release]`. A binding's release build then compiles `thinkthen` with `overflow-checks` off, which changes the engine behavior every surface ships. Smallest change: ADR item 1 requires each binding workspace to copy the root release profile (`overflow-checks = true`, `panic = "unwind"`), and the 0093 manifest check enforces it.
3. Each binding has its own `Cargo.lock`, so `thinkthen`'s normal dependencies can resolve to other versions than the ones the root gates tested. ureq, the G3 suspect, is one of them. Smallest change: a lint check that each binding lock pins the same versions for `thinkthen`'s normal dependency tree as the root lock.
4. The "surface rung" names no script and no ladder step that runs it. Smallest change: name the script, and say which step of `install`/`lint`/`test`/`spec` runs it, or that it runs by hand and where its results are recorded.
5. R2-28 has no planted bug, which ADR 0047's own checklist requires. Add one: a planted skip-only test fails the check.
6. The swap puts Rust examples ahead of C. Judgment: sound, and the plan records it. The workspace, lint, ratchet, and rung pattern needs no FFI to prove. ADR item 7 names 0094 as the FFI reference, and C still precedes every surface that needs FFI.

## 0094

1. The header table is incomplete, and one change is undecided. The known-change list misses these:
   - `thinkthen_relate` takes `texts`/`lengths` with no kind channel, while main's relate needs `{name, kind}` entities. The ticket says "the C caller maps its records to entities" but does not say how: JSON entity strings in `texts`, a new kinds array, or kind `*` as `--lines` does.
   - Recognize's "a text the recordings do not hold is refused" is stand-in replay.
   - `engine_new`'s environment variables and "connector refuses" wording.
   - `thinkthen_error_retryable`'s busy/slow/refused mapping against 0089's rule.
   - The 0.0.1 version macros.

   Smallest change: write the 19-row kept/changed/dropped table into the ticket now and decide the relate input. It is an ABI decision under ADR 0037 and belongs in design review, not in implementation.
2. The JSON door is not yet bounded. "and the rest" leaves the grammar open, and the list omits `find`/`units`. The tag door has exactly six door keys (`evidence`, `records`, `units`, `details`, `usage`, `rank`) plus `annotate`. Smallest change: list the verbs and those seven keys, state that any other door key is `usage`, and give the envelope and serializer a line cap inside the 1,100.
3. The churn probe is not in any repository. `t7a/churn/R4-1-churn.c` exists only in another session's `/tmp` scratchpad. The crash reproduced under load average 300–360, from runs in batches of 8. Smallest change: commit `churn.c` under `probes/` first. Pin NT=32, ITERS=20000, 70 engines, a refused port, and 8 parallel runs. On the tag, allow up to 300 runs and require at least one crash. Then run 300 on the new door with zero crashes.

## Follow-ups (not blocking)

- FU1: The port guide on main still says "0078 carries R5-3 and R6-3" and "no ticket carries R5-4, R6-15, or R7-1". Update it: 0096 now carries R5-3, 0085 carries G2, and 0086 and 0094 carry R7-1.
- FU2: If 0086 finding 3 leaves G3 open, record in the plan that 0094 is the ticket that closes R7-1.
