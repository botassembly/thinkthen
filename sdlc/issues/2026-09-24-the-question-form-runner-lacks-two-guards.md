# The question-form runner lacks two guards

Status: Open

The 0091 re-review (`sdlc/records/0091-code-review-2.md`) left two notes about the question-form cases in `conformance/cases.json`.

1. No test guards the rule that a fault with a `question_form` must carry `evidence`. The rule sits in the schema-only fault check in `crates/thinkthen/src/cli/conformance_tests/support/conformance.rs`. With that line removed, all six conformance tests stayed green. A ported mutation that drops `evidence` from case 29 would pin it.
2. The `form` arm in `crates/thinkthen/src/cli/conformance_tests/command.rs` runs the real command with the process environment. If someone runs the tests with a real `THINKTHEN_API_KEY` set, and a case's question becomes valid, the command would call the real backend. That call costs money. The runner should clear `THINKTHEN_API_KEY` and the base-URL variables for every command it dispatches, or build its environment without them. A test should prove that a valid question sends nothing with a key set.

The gate ladder runs with those variables unset, so neither gap affects a gate run today.
