# 0605 — routine-tests-under-timeout

Status: OPEN.

Milestone: 0.2.1

Phase: 2

## Outcome

No routine test comes near the 5-second nextest limit, and a retry reruns only the tests that failed.

### Scope

- Changes: `.config/nextest.toml`, `crates/thinkthen/src/engine/facade_tests/contract_tests.rs`, `crates/thinkthen/src/engine/store/tests.rs`, `crates/thinkthen/tests/images/replay.rs`, `crates/thinkthen/tests/native_store.rs`, `crates/thinkthen-cli/tests/backend/relate/ceiling.rs`, `crates/thinkthen-cli/tests/backend/speed.rs`, `crates/thinkthen-cli/tests/backend/default_cache/prune.rs`, `crates/thinkthen-cli/tests/backend/refusals.rs`, `crates/thinkthen-cli/tests/backend/refused.rs`, `crates/thinkthen-cli/tests/backend/secrecy.rs`, `crates/thinkthen-cli/tests/backend/recognize/stores.rs`, `crates/thinkthen-cli/tests/backend/recognize/custom.rs`, and the existing test files explicitly named in Evidence if the repeated-case inventory identifies a required repair.
- Keeps: every check's coverage.

### Proof

Five consecutive routine runs with no timeout, and the slowest test under 2.5 seconds. After landing, measure again and record the result here.

## Amendment, 2026-10-20

Ian approved moving this ticket to 0.2.1 after [the five-delays analysis](../records/2026-10-20-five-delays-root-cause.md), rule 4 of [the freeze decision](../decisions/2026-10-20-freeze-names-the-contract.md). It gates qualification (0687), not the freeze, and runs alongside language lanes.

- Scope adds the nine cases that timed out on Yellow in checkpoint 0691/1792518201049-861514 (`sdlc/records/0688-yellow-checkpoint-routine-deadlines.md`): seven SQLite fixture cases, the relate ceiling case and the speed fixture. Use that record's recommendations: build legacy fixtures in one transaction and copy valid seeds; keep the large planning regression in the long suite with a smaller routine boundary case; run one complete speed table and restrict each planted fault to its row.
- No test uses a sleep or a deadline as synchronization. Fixture setup stays outside the timed part. Every assertion and production durability check stays.
- Changes widen to the test files of those cases and `.config/nextest.toml`.
- Proof: `pm job run --machine gator 0605 SHA --disk 20 --memory 7 -- sdlc/scripts/linux-check lint test` passes five times in a row with no timeout, and the slowest routine test stays under 2.5 seconds there; Yellow serves when Gator is held. The configured landing gate passes.
- Focused store proof: `pm job run --machine m5 0605 SHA --disk 20 --memory 8 -- sdlc/scripts/job cargo test --locked --offline -p thinkthen --lib engine::store::tests`.
- Focused refusal proof: `pm job run --machine m5 0605 SHA --disk 20 --memory 8 -- sdlc/scripts/job cargo test --locked --offline -p thinkthen-cli --test backend refusals::` compiles the changed command test files and runs the independently registered refusal cases.
- The retained large regression runs once through `pm job run --machine m5 0605 SHA --disk 20 --memory 8 -- sdlc/scripts/job cargo test --locked --offline -p thinkthen-cli --test backend relate::ceiling::release_only_full_line_set_plans_inside_the_child_deadline -- --ignored --exact`.
- Before fresh review, run the configured routine gate with `pm job run --machine m5 0605 SHA --disk 20 --memory 8 -- sdlc/scripts/gate` after coordinator integration of accepted current main.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- Timeouts struck in 6 of 22 campaign runs: `secrecy`, `recognize::stores`, `recognize::custom`, `default_cache::prune` and `refused`.
- Five whole-suite reruns cost 33.4 minutes, and two of them failed again.
- At 583079a on 2026-10-16, four aggregate-command tests timed out across two routine attempts while another team's disk-heavy job ran. They launch 44, 33, 8 and 30 real command processes with answer-store fsyncs. A bounded trace found 898 fsync calls taking 4.637 seconds in total, and no lock contention or product defect.
- The slowest tests take 3.0 to 3.4 seconds, and 89 tests take over 1 second.
- Cost before: 33.4 minutes of reruns over 22 revisions, plus failed revisions. Expected after: no timeout reruns, and a failed-only retry when one is needed.

## Progress

- 2026-10-20 started
- 2026-10-20 next: Focused M5 store proof 1792527735707-23126 passed ten cases and retained long-plan proof 1792527874037-80419 passed the exact release-only regression on 4664be2d1; final dda9185be also fixes the compiler warning and retains failed store data, and is held for accepted current-main integration before the configured gate and five Gator runs without retries.
- 2026-10-20 next: Clarification of the prior progress line: dda9185be keeps and names failed store fixtures only until the outer scratch.sh cleanup and linux-check tmpfs or container cleanup. It does not guarantee durable failed-fixture artifacts; 0747 owns that shared gap for 0.3. Preserve both prior M5 receipts. Await accepted current-main integration before the configured gate and five Gator runs without retries.
- 2026-10-20 next: setup: 0744 landing recheck 1792532702783-21422 returned the historical refusal matrix timeout, and 8ce06e571 now splits all remaining historical aggregate classes while preserving their assertions; current-main integration and the focused command are pushed at 3266d36d3, with focused proof next and complete Gator proof waiting for the packaging prerequisite repair.
