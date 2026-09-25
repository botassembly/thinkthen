# Surface batch integration

Status: step 3 of `sdlc/planning/one-line-plan-2026-09-25.md`, run 2026-09-25 by Claude on `ticket/surface-batch` from main at `88f41143`. The one surface failure, in Ruby, is fixed on its branch and merged, and every landed surface's check now passes. The coordinator lands the branch after review. Python (0105, 0106) and DuckDB (0110, 0118) arrive later.

## Merges

Each branch head matched the head the coordinator named. Each merge is its own merge commit, in this order.

| Ticket | Branch head | Merge | Conflicts, resolved by keeping both sides |
|---|---|---|---|
| 0094 C | `7c42db3a` | `e3b34e84` | none |
| 0120 Rust Polars | `92b84153` | `a71bab90` | `sdlc/surfaces.txt` |
| 0107 TypeScript | `7cb678f0` | `475b8a22` | none |
| 0109 SQLite | `4b669db4` | `902f6b65` | the ADR 0047 sections |
| 0112 Ruby | `96fa3cf4` | `b0ee47e5` | `sdlc/surfaces.txt` |
| 0108 R | `e3e34f6a` | `f0dbea55` | `sdlc/surfaces.txt`, the ADR 0047 sections |
| 0111 PostgreSQL | `32bc44c7` | `ed621a7f` | `sdlc/surfaces.txt`, the ADR 0047 sections |
| 0112 Ruby tick fix | `2e0ae18b` | `c799af6f` | none |

No branch touched the root workspace members, `Cargo.lock`, `sdlc/ratchet.json`, or the planning pages of another surface. No surface file conflicted. ADR 0047 now holds the Polars amendment and the TypeScript, SQLite, PostgreSQL, Ruby, and R sections. The Ruby and R sections sit after "Consequences", where their branches put them. `sdlc/surfaces.txt` lists eight surfaces as landed. Python and DuckDB stay planned.

## Gate changes

Commit `c10f84c9`, "Teach the ladder every surface's layout". It adds 169 nonblank lines and removes 44, for 125 net. By file: `policy.py` +122 −37, `surfaces` +18 −4, `lint` +21, `deny.toml` +5 −2, `sdlc/surfaces.txt` +3 −1.

- **R's layout.** A `sdlc/surfaces.txt` line may carry a third field that names the crate folder inside the binding. R's line names `thinkthen/src/rust`. `surfaces --registry` refuses a landed surface whose line names no `Cargo.toml` and runs deny on the named manifest. `policy.py` reads the same field. It checks each folder under `libraries/*` and `databases/*`, refuses a folder where it finds no manifest, and computes the `thinkthen` path from the crate folder. R therefore needs `../../../../../crates/thinkthen`, and every other binding needs `../../crates/thinkthen`. The earlier glob already covered `databases/*/Cargo.toml`. The new folder walk keeps that.
- **Binding deny files.** One table, `BINDING_DENY`, replaces `BINDING_DENY_CHECKED` and the Polars-only deny check. Each binding's `deny.toml` must equal the root file plus its named entries, with reasons ignored: Polars' four license exceptions, SQLite's `foldhash` Zlib exception, R's RUSTSEC-2024-0436 ignore, and PostgreSQL's `foldhash` exception and RUSTSEC-2021-0127 ignore. Any other difference fails. A binding may leave out a root entry its tree never uses. A `deny.toml` in a binding the table does not name still fails. The registry already passed the binding's own `deny.toml` (0120).
- **`sources`.** The registry's deny call runs `advisories bans licenses sources` for every binding (R ticket departure item 3, SQLite ticket, 0105 amendment change 1). `lint` builds a one-crate manifest with a `file://` git dependency under a scratch `CARGO_HOME` and requires deny's `sources` check to exit 8 with `source-not-allowed` (0105 amendment change 9).
- **Root `deny.toml`.** `c10f84c9` admitted `target-lexicon` in the root file for pyo3's build. After review the coordinator ruled that it leaves the root, and Ian can overturn that ruling. The follow-up commit restores the root file and its comment. The Python binding carries the entry in its own `deny.toml`, under `BINDING_DENY`. The Polars and R `deny.toml` comments again match the root file's comment.
- **PostgreSQL (ticket 0111 item 15).** `policy.py` accepts that crate's lint table, with `unexpected_cfgs` at `deny` and the root's one `check-cfg` name. It refuses the token `unexpected_cfgs` in any binding's Rust files. For PostgreSQL it refuses a `build.rs`, a `.cargo/config` or `.cargo/config.toml` in `databases/postgresql`, `databases`, or the repository root, and `pg_ctl … restart` outside a comment in the crate's shell scripts. The existing rule keeps `unsafe` in `ffi.rs` only. The embed shim's `allow(unsafe_code)` holds no `unsafe` token, so that rule needs no change.
- **What stays in `check.sh`.** TypeScript and Ruby run their own deny and `file://` plant in `check.sh`. SQLite runs fmt and Clippy there. C asked for nothing beyond the registry.

### Plants

Each plant below ran against the real files. Each file was then restored with `git checkout` and touched. Every plant turned red.

| Plant | Red result |
|---|---|
| R's line without its crate folder, in `policy.py` | `libraries/r holds a Cargo.toml, or its surfaces.txt line names its crate folder` |
| R's line without its crate folder, in `surfaces --registry` | `libraries/r is landed and its line names no Cargo.toml` |
| R's manifest with `../../crates/thinkthen` | `libraries/r depends on thinkthen once, by path, with default features off` |
| A second license exception in SQLite's `deny.toml` | `databases/sqlite/deny.toml is the root file plus its named licenses exceptions` |
| `yanked = "warn"` in SQLite's `deny.toml` | refused as a difference from the root file |
| A second ignore entry in R's `deny.toml` | `libraries/r/deny.toml is the root file plus its named advisories ignore` |
| `wildcards = "allow"` in R's `deny.toml` | refused as a difference from the root file |
| A fifth license exception in Polars' `deny.toml` | `libraries/polars/deny.toml is the root file plus its named licenses exceptions` |
| A second ignore entry in PostgreSQL's `deny.toml` | `databases/postgresql/deny.toml is the root file plus its named advisories ignore` |
| `yanked = "warn"` in PostgreSQL's `deny.toml` | `databases/postgresql/deny.toml is the root file plus its named entries` |
| `cfg_attr(test, allow(unexpected_cfgs))` in `src/warm.rs` | `databases/postgresql/src/warm.rs names unexpected_cfgs outside the lint table` |
| An empty `build.rs` in `databases/postgresql` | `databases/postgresql/build.rs can widen check-cfg` (the follow-up renames it `databases/postgresql has a build script, which can widen check-cfg`) |
| `databases/postgresql/.cargo/config.toml` | `databases/postgresql/.cargo/config.toml can change the lints cargo applies` |
| `"$BIN/pg_ctl" -D "$DATA" restart` in `check.sh` | `databases/postgresql/check.sh runs pg_ctl restart` |
| `unexpected_cfgs` at `allow` in PostgreSQL's manifest | `databases/postgresql uses the root lint table with unsafe_code denied, not forbidden` |
| A `file://` git source under the root `deny.toml` | deny exit 8, `sources FAILED` |

`policy.py` and `surfaces --registry` also carry in-script copies of these plants on every run. The deny rule plants an extra entry in each named table and a changed `bans` setting for each binding. The lint-table rule plants PostgreSQL's `deny` level on `libraries/rust`.

### Review follow-up

A fresh review of `c10f84c9` found no weakening and no dropped merge side. The follow-up commit fixes its findings:

- A crate-folder field that starts with `/` or holds a `..` part fails. `policy.py` requires the joined path to equal its normal form and to stay under the binding folder. The registry's `awk` refuses the same field.
- For PostgreSQL, a `build` key in `[package]` fails like a `build.rs`.
- The `pg_ctl restart` match joins backslash-newline pairs first.
- For R, `policy.py` reads every `.rs` file under `libraries/r`, not only the crate folder.
- ADR 0047 describes the crate-folder field under the surface ticket's item 1 and the `sources` check under item 9. The R section no longer says the registry has yet to learn R's layout.
- `target-lexicon` leaves the root `deny.toml`, as above.

| Plant | Red result |
|---|---|
| R's line naming `../rust`, in `policy.py` | `libraries/r's crate folder libraries/r/../rust leaves the binding folder` |
| R's line naming `/tmp/rust`, in `policy.py` | `libraries/r's crate folder libraries/r//tmp/rust leaves the binding folder` |
| R's line naming `../rust`, in `surfaces --registry` | the line is refused as a crate folder outside the binding |
| `build = "planted.rs"` in PostgreSQL's `[package]` | `databases/postgresql has a build script, which can widen check-cfg` |
| `pg_ctl -D "$DATA" \` then `restart` on the next line, in `check.sh` | `databases/postgresql/check.sh runs pg_ctl restart` |
| `libraries/r/tests/planted.rs` holding `unsafe` | `libraries/r/../../../tests/planted.rs holds unsafe outside the binding's FFI module` |

The in-script copies cover `../rust`, `/tmp/rust`, the `build` key, and the split restart.

## Rungs

Each rung ran once, at a one-minute load between 2 and 9.

| Rung | Result |
|---|---|
| `install` | pass |
| `lint` | pass. The policy, registry, eight binding deny runs with `sources`, the root deny, fmt, Clippy, docs, and the inventory all passed |
| `test` | pass. 886 Rust tests passed and none failed, and the script suites passed |
| `spec` | pass. 21 demos green, 0 red |
| `surfaces` | FAIL on the first run: seven surfaces passed and Ruby failed. After the Ruby fix, Ruby's `check.sh` alone passed |

The root ratchet measures 61,721 and holds 61,721. No merge touched `crates` or `conformance`, so the ceiling stays unchanged.

## Surface checks

| Surface | `check.sh` | Detail |
|---|---|---|
| `libraries/rust` | pass | the examples test |
| `libraries/polars` | pass | the integration tests and the README doctest |
| `libraries/c` | pass | 2 unit and 6 door tests, the sanitizer row included |
| `libraries/typescript` | pass | 23 of 23 node tests; conformance 49 pass, 0 fail, 5 not run, of 54 |
| `libraries/ruby` | pass on the rerun | first run: FAIL in `test_interrupt_single.rb`, below. Rerun at `c799af6f`: all nine test files, 38 runs, 0 failures; conformance 49 pass, 0 fail, 5 not run, of 54 |
| `libraries/r` | pass | conformance 46 pass, 0 fail, 8 not run, of 54; the tarball installed and answered |
| `databases/sqlite` | pass | conformance 38 pass, 0 fail, 16 not run, of 54 |
| `databases/postgresql` | pass | 51 steps passed, `twenty_thousand_warm_rows` among them; conformance 43 pass, 0 fail, 11 not run |

## Failures

1. **Ruby, `TestInterruptSingle#test_a_raising_tick_stops_a_held_decide_at_once`.** Owner: ticket 0112. At `tests/test_interrupt_single.rb:13` the backend counted 0 held sends where the test expects 1. The test's tick raises on its second run, about 0.2 s after the watchdog starts, whatever the send has done. The call therefore stopped before its one request reached the backend. The other four tests in that file passed. A fixed tick count races the engine's first send, and the race is likely under load. The backend did not die: the same run's other held tests counted their sends. Fixed on `ticket/0112-port-ruby-surface` at `2e0ae18b`, at the coordinator's request. The child's tick now raises only after the parent's `backend.wait(1)` has counted the held send and the parent has told the child. No fixed delay remains. The fixed test passed 3 of 3 runs. Its plant made the watchdog swallow a tick's raise without firing the call's token. The test then turned red: the call stayed held, and the parent timed out waiting for the child's report. The restored file was touched. The fix merged into this branch at `c799af6f`. Ruby's `check.sh` then ran once under the heavy lock with the key unset, and it passed. Every test file ran, including the four after `test_interrupt_single.rb`.

No failure came from a merge or from the gate commit.
