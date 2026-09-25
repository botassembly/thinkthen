# 0111: Build the PostgreSQL surface

Status: built on `ticket/0111-port-postgresql-surface`, not yet reviewed or landed. Owner: Claude.

## Result

`databases/postgresql` is the unpublished crate `thinkthen-postgresql`, built with pgrx 0.17.0 over the public `thinkthen` API in its own Cargo workspace. `CREATE EXTENSION thinkthen` keeps the tag's SQL names. `sdlc/surfaces.txt` lists the folder as landed.

`check.sh` passes 49 of its 50 steps on Beelink in the final run on 2026-09-25. The one failure is the 20,000-row warm timing step. That step took 52.6 s against the ticket's 30 s bound. The section below gives what is known of the cause. The conformance runner reports 43 cases passed, 0 failed, and 11 not run, out of 54.

The check ran without Docker, without root, and without a TCP port. The server package came from experiment 254's copy, whose SHA256 matches `runtime.sha256`. Nothing was downloaded.

## Measurements

| What | Result | Bound |
| --- | --- | --- |
| Single cancel after the first send | under 200 ms | 200 ms |
| psql start, measured once per run | 27 ms and 29 ms | none |
| `statement_timeout = '300ms'` on a held call | under 500 ms from after `fresh` | 500 ms |
| `deadline_ms = 200` on a held call | under 1 s from after `fresh` | 1 s |
| `deadline_ms = 1000` on a held 200-row batch | under 1.5 s from after `fresh` | 1.5 s |
| Throttle-8 batch cancel | under 200 ms, count stays 8 | 200 ms |
| Two-record array batch on the generic arm | under 90 ms, inside the server | 90 ms |
| 20,000-row scalar decide over cached answers | 1,029 to 1,073 ms, about 52 µs a row | 100 µs a row |
| 20,000-row warm at throttle 32, first run over an empty cache | 46 to 52.6 s over four runs | 30 s |
| The same warm again over the cached rows | 2.5 s | none |
| One loopback request on a reused connection, from `curl` | about 41 ms | none |
| `check.sh`, warm cargo target | about 2 to 3 minutes | 8 minutes |

Each timed step starts its clock after `fresh` restarts the server and backend. The clock still includes one psql start, which the run prints.

## The failing step

The loopback backend (`conformance/backend`, ticket 0117) answers about 41 ms after each request on a reused connection. `curl` against the generic arm shows the same delay with no engine involved. The delay matches a small write waiting on a delayed acknowledgement. At throttle 32, 20,000 requests take about 50 s.

A second warm over the same 20,000 cached rows takes 2.5 s. The binding's own text handling and the cache path therefore cost little, and the time sits in the sends. The coordinator's `TCP_NODELAY` Quick Fix to the backend is the expected remedy. This record does not promise that the fix alone turns the step green. If the step stays slow after it, the next look is the engine's send path under throttle 32. The step stays as the ticket wrote it.

## Differences from the ticket

Ticket item 14 records these, and the coordinator accepted them on 2026-09-25. Item 15 records the gate changes left to the landing agent.

- `unsafe` lives in `src/ffi.rs`, as ADR 0047 item 5 says. The ticket named a `src/ffi/` folder that also held the pgrx functions. The pgrx functions stay in `lib.rs`, `relate.rs`, and `warm.rs`.
- The settings registration moved into `call.rs` beside the plan it feeds, so `lib.rs` stays under 500 lines.
- `thinkthen_recognize` gained a spec overload, `thinkthen_recognize(body, '@names.json')`, under decision 8's `Recognize::from_json`. Case 41 and the recognize cases need the spec's kind meanings.
- `18-annotate-two-groups` and `25-defect-fault` report "not run". The ticket's list named neither. A SQL call's evidence is one whole text, and no outside boundary reaches a defect. The Rust consumer runner skips the same two.
- The counting setter stand-in and its two unit tests left. They needed a test-only trait. A unit test with no hook now checks `Plan::of` for the registered defaults and for a plan with every setting set. The settings steps of `check.sh` cover the setters through SQL.
- The warm steps run at throttle 32, so the steps finish inside their `timeout` and reach their assertions.

## Findings for others

Filed in `sdlc/issues/`:

- `2026-09-25-engine-counters-count-per-engine-not-per-process.md`. 0084 describes `Counters` as a process total. The binding adds every engine's counters.
- `2026-09-25-no-public-parser-for-inline-relate-rules.md`. The binding copies `inline_rule` from `core/relate_file.rs:295`, and a comment in `relate.rs` names it.
- `2026-09-25-thinkthen-error-has-no-public-constructor.md`. A binding's own refusals need their own type.
- `2026-09-25-the-postgresql-recognize-relate-and-usage-columns-changed.md`.

Not filed, by the coordinator's ruling: streaming `decide_many` sends records before it refuses a batch over `max_requests`, and the loopback backend's 41 ms delay. The binding checks held records itself first.

## The request total

`thinkthen.max_requests_total` follows Ian's ruling of 2026-09-25 (`sdlc/planning/one-line-plan-2026-09-25.md`, main `f78421be`). It is unset by default. The spend is the sum of `requests_sent` over the backend's engines. Each call computes what remains once. A spent total refuses as usage with "thinkthen.max_requests_total allows N requests in this backend, and they are spent" and sends nothing. An array batch longer than what remains sends the first records that fit, then refuses with the same sentence. The public API has no per-call limit, so the binding builds no extra engines. A forked backend starts from zero. Calls in flight at once can each spend what remains, so the total can be passed by one call per backend in flight, plus retries. The README says both.

`the_total_holds_across_rows` sets the total to 3 over 10 scalar rows and gets exactly 3 sends, then the sentence. One more call in that session sends nothing and refuses. A new session answers a cached row with `t`, and a four-record array sends 3, then refuses.

## Gate changes for the landing agent

This build changes no ladder script, by the brief. Ticket item 15 lists exactly what the landing agent adds: the `policy.py` lint table and `unsafe_code` placement, the `unexpected_cfgs`, `build.rs`, `.cargo/config`, and `pg_ctl restart` refusals, the `deny.toml` comparison in `lint`, and `surfaces --registry` reading `databases/postgresql/deny.toml`. Until then, `policy.py` refuses this crate's lint table, and the registry deny fails on `foldhash` and RUSTSEC-2021-0127. The crate's own deny command passes all four checks.

## Planted failures

`scratchpad/plants.py` applied each plant, ran the named steps of a copy of `check.sh` without fmt, clippy, and unit tests, then restored the files with git and touched them. A unit plant ran `cargo test`, and a deny plant ran `cargo deny`. The tree was clean after every run.

Red: R1-10, R1-18, R1-19, R1-22a, R1-22b with R2-24 (three steps), R1-28 (the compile fails), R1-30, R1-31, R2-4a, R2-4b, R2-10, R2-19, R2-31, R2-32a, R2-32b, R3-2, R3-8, R3-10, R3-21, R3-29, R3-30, R3-32, R5-15b, R5-16, R5-17, R5-19, R5-30, R6-8, R6-14, G1a, G1b, and the extra plants for the preload, the signal mask, the key's secrecy, and the total checked only at engine build.

Stayed green:

- R4-6, confined reads through `open_plain`. The descriptor-path check refuses the read first. The test proves the read is refused, but not which layer refused it.
- R5-15, the size checked before confinement. The spelling check refuses the outside path before any size check. The stronger R5-15b puts the size check before every confinement step, and it turns red.

R6-14 first stayed green, because `another_model_sends_again` warmed one model and decided the other in two psql sessions. A per-backend memo never saw both. The step now decides both models in one session, and the plant turns red.

Not run: the `allow(unexpected_cfgs)` plant, which needs the landing agent's `policy.py` step.

Red-first: with `lib.rs` cut to `pg_module_magic!` alone, 41 of 50 steps fail and conformance reports 0 passed and 43 failed. The nine that pass check for absences or structure: `shipped_lacks_probe`, `no_home_in_library`, `no_catch_unwind`, `every_cargo_call_is_locked_and_offline`, `start_refuses_other_hosts`, `darwin_reports_not_run`, `readme_grant_is_the_fixture`, `the_answer_map_is_gone`, and `the_fake_key_stays_in_the_environment`.

## Budgets

- Production Rust: 1,521 nonblank lines in seven files, counting `src/bin/pgrx_embed.rs`. The ticket allowed 1,500. Item 16 raises it to 1,540 for the request total and the review fixes, approved by the coordinator on 2026-09-25, and Ian can overturn it. Each file's production code is under 500 lines. `call.rs` holds 417 production lines and 505 with its unit tests.
- Unit tests: 285 nonblank lines.
- `check.sh` and `runtime.sh`: 723 nonblank lines together. `tests/runner.py`: 211. `tests/examples.py`: 32.
- `ratchet.json` holds 1,806 and `ratchet.py.json` holds 243, the measured totals.
- `serde_json` still earns its place: the members merge of decision 8 and the `jsonb` results use it. No `serde` derive remains.
