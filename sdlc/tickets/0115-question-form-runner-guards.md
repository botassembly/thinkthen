---
flow: quick-fix
priority: 115
opens: test
---

# 0115: Question-form runner guards

Status: open. Owner: Claude.

## Outcome and authority

The 0091 re-review left two gaps in the conformance command runner (`sdlc/issues/2026-09-24-the-question-form-runner-lacks-two-guards.md`). A mutated question-form case could reach the real backend when a key sits in the environment, and no test guards the rule that a question form needs evidence. This quick fix closes both.

## Work

1. The runner dispatches each question-form case in a child test process. The child environment drops every `THINKTHEN_` variable that names a key or a URL. The child also passes `--url` for a loopback listener that counts requests, and it requires zero.
2. A test starts the runner with `THINKTHEN_API_KEY=test-key-not-real` and `THINKTHEN_BASE_URL` naming a counting loopback listener. The child asks a valid question. It must see neither variable, fail for lack of a key, and send nothing. The test fails before step 1.
3. A ported mutation drops `evidence` from case 29. It passes validation when the rule is removed.

The command runs in process, and the crate forbids `unsafe`, so a test cannot clear its own environment. A child process is the only place the runner can remove the key.

Touches only `crates/thinkthen/src/cli/conformance_tests/`, `sdlc/ratchet.json`, the issue, this ticket, and its record.
