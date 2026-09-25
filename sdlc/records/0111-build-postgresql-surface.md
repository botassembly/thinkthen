# 0111: Build the PostgreSQL surface

Status: built on `ticket/0111-port-postgresql-surface`, not yet reviewed or landed. Owner: Claude.

## Result

`databases/postgresql` is the unpublished crate `thinkthen-postgresql`, built with pgrx 0.17.0 over the public `thinkthen` API in its own Cargo workspace. `CREATE EXTENSION thinkthen` keeps the tag's SQL names. `sdlc/surfaces.txt` lists the folder as landed.

`check.sh` passes 47 of its 48 steps on Beelink. The one failure is the 20,000-row warm timing step. That step took 48.7 s against the ticket's 30 s bound. The cause sits in the loopback backend, and the section below gives it. The conformance runner reports 43 cases passed, 0 failed, and 11 not run, out of 54.

The check ran without Docker, without root, and without a TCP port. The server package came from experiment 254's copy, whose SHA256 matches `runtime.sha256`. Nothing was downloaded.

## Measurements

| What | Result | Bound |
| --- | --- | --- |
| Single cancel after the first send | under 200 ms | 200 ms |
| `statement_timeout = '300ms'` on a held call | under 1.5 s wall, including psql start | 500 ms |
| Throttle-8 batch cancel | under 200 ms, count stays 8 | 200 ms |
| Two-record array batch on the generic arm | under 90 ms, inside the server | 90 ms |
| 20,000-row scalar decide over cached answers | 1,029 to 1,073 ms, about 52 µs a row | 100 µs a row |
| 20,000-row warm at throttle 32 | 48.7 s and 52.1 s in two runs | 30 s |
| One loopback request on a reused connection, from `curl` | about 41 ms | none |
| `check.sh`, warm cargo target | about 2 to 3 minutes | 8 minutes |

The `statement_timeout` step measures from the shell, so it bounds the stop at 1.5 s, not the ticket's 500 ms. The in-server figure was not taken.

## The failing step

The loopback backend (`conformance/backend`, ticket 0117) answers about 41 ms after each request on a reused connection. `curl` against the generic arm shows the same delay with no engine involved. The delay matches a small write waiting on a delayed acknowledgement. At throttle 32, 20,000 requests take about 50 s. The surface cannot change `conformance/`, so the step stays red and names the cause.

## Differences from the ticket

- `unsafe` lives in `src/ffi.rs`, as ADR 0047 item 5 says. The ticket named a `src/ffi/` folder that also held the pgrx functions. The pgrx functions stay in `lib.rs`, `relate.rs`, and `warm.rs`.
- The settings registration moved into `call.rs` beside the plan it feeds, so `lib.rs` stays under 500 lines.
- `thinkthen_recognize` gained a spec overload, `thinkthen_recognize(body, '@names.json')`, under decision 8's `Recognize::from_json`. Case 41 and the recognize cases need the spec's kind meanings.
- `18-annotate-two-groups` and `25-defect-fault` report "not run". The ticket's list named neither. A SQL call's evidence is one whole text, and no outside boundary reaches a defect. The Rust consumer runner skips the same two.
- The counting setter stand-in and its two unit tests left. They needed a test-only trait, and the settings steps of `check.sh` cover the same behavior through SQL.
- The warm steps run at throttle 32, so the steps finish inside their `timeout` and reach their assertions.

## Findings for others

- The engine counts sends per `Engine`, not per process. 0084 describes `Counters` as a process total. The binding adds every engine's counters.
- Streaming `decide_many` sends records before it refuses a batch over `max_requests`. The binding checks held records itself first.
- The public API has no parser for the command's inline relate rules, so the binding repeats the grammar.
- `thinkthen::Error` has no public constructor, so a binding's own refusals need their own type.
- `policy.py` refuses this crate's lint table, because it wants `unexpected_cfgs` at forbid. The ticket's `policy.py` additions did not land here, because this build does not change the ladder scripts: accept `deny` for this crate, refuse the token `unexpected_cfgs` in `src`, refuse `build.rs` and `.cargo/config*`, refuse `pg_ctl restart`, and compare `deny.toml` with the root file.
- `surfaces --registry` runs deny with the root `deny.toml`. It needs to read `databases/postgresql/deny.toml`, or it fails on `foldhash` and RUSTSEC-2021-0127. The ticket's deny command passes: advisories, bans, licenses, and sources are all ok.
- The loopback backend's 41 ms reply delay, above.

## Planted failures

Two plants ran on scratch copies. Dropping `--locked` from the unit-test call turned the cargo-flag scan red (R3-29). Replacing the README grant block turned the byte comparison red (R3-9). The other planted bugs in the ticket's table have not run yet.

## Budgets

- Production Rust: 1,499 nonblank lines in seven files, each under 500. Unit tests: 257.
- `check.sh` and `runtime.sh`: 681 nonblank lines together. `tests/runner.py`: 211. `tests/examples.py`: 32.
- `ratchet.json` holds 1,756 and `ratchet.py.json` holds 243, the measured totals.
- `serde_json` still earns its place: the members merge of decision 8 and the `jsonb` results use it. No `serde` derive remains.
