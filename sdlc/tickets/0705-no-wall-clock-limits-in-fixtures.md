# 0705 — Fixtures have no tight wall-clock limits

Status: OPEN.

Milestone: 0.2.1

Reviews: revision 8d51878, accept

Reviews: revision 83a707f, accept

## Outcome

- Changes: `.config/nextest.toml`, `conformance/c_parity.py`, `conformance/children/check.py`, `conformance/children/children.py`, `conformance/children/complete_parity.py`, `conformance/children/provider_diagnostics.py`, `conformance/children/signal_observer.c`, `conformance/children/signal_observer_probe.c`, `conformance/children/signals.py`, `conformance/children/test_privacy.py`, `conformance/children/usage.py`, `conformance/children/usage_installed.py`, `conformance/cli_parity.py`, `conformance/test_campaign_env.py`, `conformance/test_candidate_campaign.py`, `conformance/test_parity.py`, `libraries/cpp/fixtures/owned_calls.py`, `libraries/go/fixtures/owned_installed.py`, `libraries/zig/Tests/installed-session.py`, `sdlc/scripts/build-admission-test.py`, `sdlc/scripts/coverage-checks.py`, `sdlc/scripts/hosted-workflow-self-test.py`, `sdlc/scripts/install-check-self-test.py`, `sdlc/scripts/installer-test`, `sdlc/scripts/registry-readmes.py`, `sdlc/scripts/release-bounded.py`, `sdlc/scripts/release-process-cleanup-test.py`, `sdlc/scripts/release-python-self-test.py`, `sdlc/scripts/windows-installer-integration-self-test.py`, `sdlc/scripts/windows-installer-test.py`.
- No test fixture fails because a fake tool or child script took more than a few seconds on a loaded machine. Fixture subprocesses that run fake tools use one shared generous limit (60 seconds) that exists only to stop a true hang, set in one place, and print the stalled command and its output when it fires.
- Fixtures that only restate workflow or file layout, such as the hosted-workflow Apple and DuckDB cache cases, are deleted rather than updated, per 0614.
- The Rust tests that timed out under nextest on a busy M5 either get a named slow-test override in the nextest profile or are made faster; the same one test never fails a gate twice.

### Proof

- `git grep -nE 'timeout=[0-9]\b'` over `sdlc/scripts` and `conformance` finds no per-call limit under the shared one.
- One M5 gate run while two other ThinkThen jobs run passes.

## Evidence

Of the 21 failed M5 gate runs between 2026-10-18 and 2026-10-19, about two thirds failed in the test machinery, not the product. Five-second subprocess limits in release-workflow and PostgreSQL package fixtures expired four times while the M5 ran other jobs. Two single Rust tests timed out under nextest. Each failure cost a 5 to 25 minute gate and a diagnosis, and one held three accepted tickets. The `timeout=5` pattern appears 41 times across `sdlc/scripts` and `conformance`.

This removes a cost every lane pays, so it lands before other work that waits on the gate.
