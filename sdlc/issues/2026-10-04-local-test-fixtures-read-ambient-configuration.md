# Local test fixtures read ambient configuration

Status: open only for remaining child-environment isolation under [0487](../tickets/0487-consolidate-test-child-environments.md). The spec/demo configuration-home defect is fixed by 0471 at `ed7f5d9`.
Milestone: 0.2
Kind: debt
Debt: 038
Severity: medium
Pay when: the next reviewed fixture-isolation slice, related to [0404](../tickets/0404-tech-debt-and-tests-held-to-behavior.md).

Ambient configuration makes local fixture results depend on the runner's setup and can conceal the actual regression a gate is meant to catch.

At source `4349b91b3`, the ambient checkpoint refused a custom definition that now shares a built-in name. The type fixture previously failed its first C constructor; ticket 0399 fixed its owning isolation after fresh Medium acceptance. The next ambient run reached `demos/16-triage/README.md` and its `--plan` refused with the same value-free built-in configuration sentence. No user configuration contents or credential files were read.

The coordinator's full test and specification gates pass with an owned empty HOME and explicit existing build-cache homes. This proves the isolated fixture environment, not the default HOME. The ignored receipts are `target/0399-checkpoint-final/test-isolated.log`, `spec-isolated.log` and `failed-ambient-after-types.log`.

The next slice should identify fixtures that read ambient configuration, then isolate each owning runner through existing owned scratch helpers after any environment filtering. Preserve usage isolation, lock reexecution, source identity checks and cleanup ownership. Prove the real fixtures against owned conflicting and malformed configuration plants. Do not change runtime refusal behavior or broaden this ticket's fixture fix.

## Reconciliation, 2026-10-08

Both `sdlc/scripts/spec` and `sdlc/scripts/demos` now call the existing `config_home` helper. The checkpoint failure below is historical evidence. Ticket 0487 owns the remaining shared child-environment intake; fixture defects remain 0.2 and do not change runtime configuration refusal.
