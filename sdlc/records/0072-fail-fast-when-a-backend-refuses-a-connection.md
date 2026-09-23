# 0072: Refused connections fail fast

Status: Landed on main as `47650c1` after independent acceptance and the final sequential local ladder. Main was pushed and ancestry verified. GitHub Actions remains manually disabled; no hosted success is claimed.

## Result

A typed `TransportKind::Refused` now fails after its first observed attempt even when retries remain. The command keeps exit 4 and the exact safe refusal diagnostic. Timeout, name-lookup, premature-close, other transport failures, and the six retryable statuses keep their existing classification. The existing close-before-headers test still proves a retry and `requests_sent:2`.

## Review and verification

Independent design review accepted the bounded level-2 SWE-2 route. SWE-2 observed the unit classifier, attempt-observation, and compiled default-retry timing tests fail under the old three-attempt behavior, then pass after one match arm changed.

Independent code review rejected the first closed-port proof because dropping an ephemeral listener allowed another local process to claim that port. The implementation switched both real-transport tests to loopback destination port zero, which no listener can claim. Parent-observed engine tests prove the HTTP stack returns typed refusal and invokes the attempt observer once. The same reviewer accepted the remediation after the coordinator supplied the complete staged diff and personally ran the focused tests and Clippy.

Final coordinator command: `sdlc/scripts/install && sdlc/scripts/lint && sdlc/scripts/test && sdlc/scripts/spec && git diff --cached --check`. Exit 0: policy and package checks, audit, format, Clippy, docs, exact ratchet `35974/35974`, 625 Rust tests passed with one intentional ignored child-harness test, doctests, all schema/probe/transform/replay checks, and nineteen green how-tos.

Main advanced with site and library-review records after implementation. The combined tree's first full ladder hit `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` at its existing timing assertion. This ticket changes no scheduler or annotate code. The exact test then passed alone, all 296 backend tests passed together, and the complete four-rung ladder passed on the combined tree. This record retains the failure rather than classifying it as harmless.

The Rust ceiling rose by 65 lines. The growth pins every transport and retryable status class, real attempt observation, and compiled default-retry timing/secrecy behavior. The implementation reuses the retry classifier, command harness, existing refusal diagnostic, and existing premature-close test; no released-port helper or new transport abstraction was added. `exchange.rs` remains below its cap at 496 nonblank lines.

No diagnostic, transport taxonomy, counter, cache, recording, cancellation, deadline, width, fork, signal, dependency, workflow, surface, site, provider, or publication behavior changed.