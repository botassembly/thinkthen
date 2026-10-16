# 0605 — routine-tests-under-timeout

Status: OPEN.

Milestone: 0.3

## Outcome

No routine test comes near the 5-second nextest limit, and a retry reruns only the tests that failed.

## Evidence

- Source: the read-only build-efficiency audit of 2026-10-16, at Ian's request. Timings come from the 0543 release campaign's phase records, 22 revisions from 2026-10-14 to 2026-10-16. Re-measure on main before acting.
- Timeouts struck in 6 of 22 campaign runs: `secrecy`, `recognize::stores`, `recognize::custom`, `default_cache::prune` and `refused`.
- Five whole-suite reruns cost 33.4 minutes, and two of them failed again.
- At 583079a on 2026-10-16, four aggregate-command tests timed out across two routine attempts while another team's disk-heavy job ran. They launch 44, 33, 8 and 30 real command processes with answer-store fsyncs. A bounded trace found 898 fsync calls taking 4.637 seconds in total, and no lock contention or product defect.
- The slowest tests take 3.0 to 3.4 seconds, and 89 tests take over 1 second.
- Cost before: 33.4 minutes of reruns over 22 revisions, plus failed revisions. Expected after: no timeout reruns, and a failed-only retry when one is needed.

## Scope

- Changes: `.config/nextest.toml` and the test files named above.
- Keeps: every check's coverage.

## Proof

Five consecutive routine runs with no timeout, and the slowest test under 2.5 seconds. After landing, measure again and record the result here.
