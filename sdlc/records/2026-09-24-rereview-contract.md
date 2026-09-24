# Confirmation, 2026-09-24

Read-only. Diffed against the rejected commits, fetched from origin: 0084 `cd0a78aa`, 0095 `837263e5`, 0097 `d1e3f92a`, 0093 with ADR 0047 `6467d4e7`, and main `fdb7c535`. All four branches match origin. Nothing was built or run.

- 0084: ACCEPT. C1: 0096 owns fork recovery in both the outcome and the ADR 0017 amendment, and it is gone from 0085's list. C2: the line now reads "rustfmt parses". `wc -m` is 29,989, under 30,000.
- 0095: ACCEPT. C3 uses the rustfmt sentence. The C4 panic text now cancels, then joins, then resumes. F1 states the reason for `Sync` and the SQLite wrapper.
- 0097: ACCEPT. C4 is applied in the design and acceptance, with a planted bug for "join without cancel". F2 adds the recording and lock waits and the "runs once" bug. F3 routes the check through the one poll function and adds no call site in `annotate_schedule.rs`.
- ADR 0047: ACCEPT. Item 5 awaits Ian. C5: the reader takes an optional config path, the no-argument call is unchanged, a per-binding ratchet exists, and each host language gets its own file. F4: the release profile is copied, the lint table is equal except `deny`, and the lock is pinned, all checked by `policy.py` with planted failures in 0093. F5: option (b)'s cost is now stated fairly.
- Main plan (F6): resolved on `fdb7c535`. 0086 exposes `interrupt` and the two deadline methods. The header now says no other vendor reviews this queue.

New follow-ups (non-blocking):
- N1 (ADR 0047 item 1, 0093): Word the lock check to compare only the version that `thinkthen`'s own tree resolves to, because a lock may hold a second major version. An exact pin in a host crate, such as pgrx, may force a root lock bump or conflict. The surface ticket records that case.
- N2 (0093): 150 script lines must now carry seven lint checks, the ratchet argument, the registry check, and the `surfaces` rung. Re-measure when it starts. Name the new fifth rung in the repo `CLAUDE.md` ladder line when 0093 lands.
- N3 (0084): 11 characters of headroom remain, so the next edit must cut first. 0095 and 0097 rely on `CallOptions` being `Send + Sync`, but 0084's trait list names only `Copy + Clone + Default`. Add it to 0084's trait list when there is room, or leave it to the 0086 fixture.

---

# Contract re-review: 0084, 0095, 0097, ADR 0047

Reviewer: fresh read-only Claude session, 2026-09-24. Read: workspace `CLAUDE.md` (Tickets), repo `CLAUDE.md`, main `860086d2` (one-line plan, port guide, ADR 0017, ADR 0037, `Cargo.toml`, `sdlc/ratchet.json`, `sdlc/scripts/ratchet.mjs`, `sdlc/scripts/policy.py`, `engine/*.rs`), 0084 `de9087ce`, 0095 `a68b9fee`, 0097 `c83d4c3c`, 0093 with ADR 0047 `1e911ae1`, 0085 `950d6cbb` and 0096 as input, `notes/todos/2026-09-24-thinkthen-width-cap-with-two-library-copies.md`, and the two prior reviews. All four branches match origin. Nothing was built or tested. One observation ran `rustfmt` on scratch copies of the two contract blocks.

- 0084: REJECT. Two small fixes (C1, C2). Both shorten the ticket.
- 0095: REJECT. One small fix (C3), shared with C4.
- 0097: REJECT. One small fix (C4).
- ADR 0047: REJECT. One fix (C5). Item 5 states its options fairly (see F5).

## Prior findings

All resolved. B1: `rust.md` shows the infallible `Evidence` and the join-on-drop sentence. B2: the 2026-09-24 amendment lists every 0095 member, and 0095 `opens` ADR 0017. B3: item 1 gives each binding its own workspace and lock. B4: item 3 keeps `forbid` in `lib.rs` and the root table, and uses `deny` plus one allowed FFI module per binding. B5: item 6 names the lib `thinkthen_c` with no `rlib`. B6: the todo exists with options, costs, and a recommendation. F1, F3, F4, F6 (now 0096), F7, F8 (0099 and last-wins), F9 (Polars row on main), and F10 are applied. Engine review F1 (split), X2 (planted panic), F9 to F12, and FU3 are applied, except the ratchet (C5).

## Blocking findings

**C1. 0084 still gives fork recovery to 0078 and 0085.** The outcome paragraph says 0078 must land and reconcile "fork recovery" before 0085. "Runtime ownership" lists "fork recovery inherited from accepted 0078" among 0085's tests. The plan and 0085 moved fork recovery to 0096, which lands after 0085 and excludes it from 0085. The 2026-09-23 ADR 0017 amendment on the same branch repeats the old list. Smallest change: drop "fork recovery" from both reconcile lists and from 0085's list, and add "0096 owns fork recovery before 0086." The net change frees characters.

**C2. 0084's acceptance line `rustfmt --check passes on the extracted blocks` fails as written.** Observed: `rustfmt --edition 2024 --check` exits 1 on both extracted blocks, because the one-line signatures exceed the width. `rustfmt --emit stdout` exits 0, so both blocks parse. Smallest change: "rustfmt parses the extracted blocks of 0084 and 0095."

**C3. 0095's acceptance line "the added block parses as the 0086 fixture generator's input" cannot be checked at design time.** The generator does not exist yet, and 0098 now builds most members. Smallest change: use C2's sentence.

**C4. A panicking interrupt check joins workers but does not stop dispatch (0097, and 0095 Meanings).** Both texts say the panic "joins every engine worker, then resumes." Neither says new work stops first. Jobs already in the queue window can still send, and joining can wait on the rest of a batch. The acceptance test checks only that the payload resumes after the join. Smallest change: "A panic fires the call's cancel token, joins every worker, then resumes the payload." Add to 0097's acceptance: "the listener sees no new request after the panic." Add one planted bug: the panic path joins without cancelling.

**C5. ADR 0047 item 4 cannot work with the unchanged reader.** `ratchet.mjs` finds its repo from its own file location and always reads `<repo>/sdlc/ratchet.json`. It takes no argument and ignores the working directory. It also counts `.rs` files only. A `ratchet.json` beside a binding's `Cargo.toml` is never read. Smallest change: pick one option. (a) Each binding carries `sdlc/ratchet.json` and a byte-identical copy of `sdlc/scripts/ratchet.mjs`, and `lint` checks that the copy is identical. (b) The shared reader takes an optional config path, and the other repos that use it are noted. Also say whether host-language code (`.py`, `.ts`, `.R`, `.rb`, SQL) is counted. 0093's matching sentence changes with it.

## Answers

1. **Nine surfaces.** Each still binds through the public API alone. DuckDB and the other streaming hosts that resume across callbacks keep a batch on a worker thread, because `Batch` borrows and is not `Send`. 0095 keeps that pattern legal.
2. **ADR 0047 against main.** Workable: separate workspaces, `exclude`, and `default-members` (0092 owns the policy change for its test member). Also workable: `deny` plus one allowed FFI module, with `forbid` staying in `lib.rs` and in `ACCEPTED_RUST_LINTS`, and the `thinkthen_c` name with `cdylib` and `staticlib`. Not workable: the ratchet (C5).
3. **0084 size.** `wc -m` is 29,954, which leaves 46 characters. The contract block carries the weight, and the prose reads cleanly. C1 and C2 both shorten it.
4. **0097 bounds.** Six files, 180 production lines, and 400 test lines, with two planted bugs. The ticket is bounded apart from C4.
5. **Ian's rulings.** No contradiction. Item 5 waits on Ian. Item 8 matches the 2026-09-21 ruling, which left a Rust Polars door open and unruled.

## Follow-ups (non-blocking)

- F1 (0095, 0097): Say why the check is `Sync`. The likely reason is that `CallOptions` must stay `Send + Sync` for workers. Also say that a host whose handle is not `Sync`, such as the SQLite database pointer, wraps it in its FFI module.
- F2 (0097): The design lists recording and lock waits, but the acceptance covers only the width gate, the retry wait, and the bulk reply. Add those two waits or drop them. Add a planted bug where the check runs only once.
- F3 (0097): `annotate_schedule.rs` has exactly 500 nonblank lines, measured on main. The check must enter through 0085's one poll function, or the file splits first.
- F4 (ADR 0047 items 1 and 3): Cargo ignores a path dependency's workspace profile. Each binding copies the root `[profile.release]` (`overflow-checks = true`, `panic = "unwind"`). State that the binding lint table equals the root table except `unsafe_code = "deny"`. `policy.py` checks both.
- F5 (ADR 0047 item 5): The options are fair. Option (b)'s marker cannot be a Rust static, because each copy has its own statics. Its cost belongs to the same class as (c), minus the fork story, so say so. Point the todo at the main path once 0093 lands.
- F6 (main plan): The plan says 0098 builds every 0095 member beyond 0084. 0095 says 0086 exposes `interrupt` and both deadline methods. Align the two. The plan header also names Codex as the reviewing vendor, while the queue routes every review to a fresh Claude session. Say where the Codex review happens.
