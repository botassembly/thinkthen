# Bounded test retirement inventory

The coordinator commissioned a read-only inventory at source `4cbf8a08` in local experiment `2034-thinkthen-test-retirement-inventory`. Pi inspected five candidate groups near recent cache, usage, rank and file-form work. A fresh independent Sol Medium reviewer checked its claims against source, and the coordinator independently inspected both proposed removal groups. No build, test run or deletion was needed. This record does not close the scaffold-test issue.

## Verified result

None of these five groups supports retirement. Similar assertions occur in different behavior sequences, and removing a cheap assertion from an otherwise necessary test saves no execution.

| Group | What the inspection established | Disposition |
| --- | --- | --- |
| Missing-key and line-break cache recovery | `tests/backend/cache_identity.rs` proves recovery after two different refusals. The missing-key case binds to a second address. The line-break case additionally hits that bound cache with the invalid key. Ordinary successful cache-hit cases prove neither complete sequence. | Retain both tests and their tails. |
| Rank top-N unit | `cli/schedule/top_tests.rs` checks the retained-row bound after every callback, then exact winners and counts for all five judged rows. Public rank cases cover order and truncation but cannot observe the private storage bound. | Retain the combined test. Removing its stdout assertion saves no execution. |
| C, TypeScript and Ruby question-file refusals | Each host has its own path conversion, native boundary and error carrier. Repeated input categories protect distinct host secrecy and zero-send behavior. | Retain each host boundary table. |
| Usage sidecar failures | The optimization test proves an unchanged sidecar was not rewritten. The durability test proves a failed changed-sidecar write preserves the durable base and warns once. | Retain both outcomes. |
| Host SIGXFSZ pair | Shared child setup exercises different terminal outcomes: preserving the host handler and respecting the host's default termination action. | Shared setup is not duplicate proof. Retain both. |

The inventory incorrectly described the missing-key case as a cached-hit test and treated both cache tails as partial duplicates. Review corrected that description and rejected deletion. It also rejected the proposed rank assertion removal. The next inventory must compare the triggering state, intervening operations and final guarantees, not just matching assertion text. It must identify an execution or meaningful maintenance cost eliminated by a proposed consolidation.

## Gate selection and limits

The routine test rung selects 31 IDs from the 54-case conformance corpus. It uses exact backend and secrecy filters, the listener target, public settings/control/member targets, selected library tests, public consumer tests, doctests and fixture/transform/demo checks. The inspected cache, rank, usage and host-signal cases do not sit in its unfiltered targets. Host binding checks remain in the surfaces rung. The full functional and stress runners remain explicit opt-in paths.

`test-stress --check` verifies seven ignored Rust campaign names and also executes one nonignored Polars functional case. It does not run the ignored campaigns or port stress selection. This qualification matters when describing a check as inspection-only.

The inventory counted 1,192 Rust `#[test]` declarations under `crates/`. That is a static declaration count, not collected or executed tests and not a new runtime measurement. The 31 routine IDs and 54 corpus cases count different things. No timing improvement, complete audit or whole-suite coverage claim follows from this pass.

Ticket 0205 previously removed two plan-document units backed by executable decide proofs. Ticket 0119's landed listener cleanup removed no test. The remaining scaffold issue stays open for a bounded candidate with stronger equivalent proof. Broader mutation, saturation and churn campaigns remain outside routine validation under Ian's ruling.

## Experiment handling

The first detached inventory wrapper ended without a completion marker or report. Its original folder and launch note remain preserved. A second invocation kept the waiting parent alive, completed with an `ok` marker and produced the reviewed report. This establishes which invocation produced evidence; it does not establish the first wrapper's cause of exit or that it made no API request. Both artifacts stay local and unpushed.
