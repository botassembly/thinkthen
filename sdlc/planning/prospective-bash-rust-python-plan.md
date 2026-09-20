# Prospective plan for Bash, Rust, and Python

Date: 2026-09-20. Status: temporary, prospective. Character budget: 4,000.

Ian requested this high-level plan after the project review. Tickets will be written as work begins. The [main plan](plan.md) records current work. [ADR 0017](adr/0017-libraries-over-one-bound-core.md) remains proposed. Ian can revise any recommendation here. Retire this note when the main plan and ADRs absorb it.

## Destination

Bash users call the command-line tool. Rust applications import a client library. Python applications install a package bound to the same pure Rust core. All three share question files, judgment rules, request bytes, results, and recordings. The intended commands are `decide`, `choose`, `score`, `filter`, `rank`, `annotate`, and `find`.

The core keeps parsing and judgment rules. Each host owns networking, files, credentials, retries, and concurrency. Policy stays in the caller or a tested `jq` transform. A Python binding experiment tests the proposed design.

## Proposed sequence

1. **Repair the foundation.** Address the review findings against `2c32524`: unblock interactive input, bound waiting rows, reject invalid distributions, reconcile recording and cache guarantees, correct evaluation handling of labels and question changes, and enforce the live spending limit. Identify the evidence and model behind each result.
2. **Complete the Bash tool.** Finish the existing message and documentation repairs and `annotate`. Measure long-document selection before implementing `find`. Complete the evaluation transforms, flagship triage pipeline, and executable examples. Prepare installation, help, the manual page, and the agent skill.
3. **Establish shared behavior.** Reconcile specifications and the library proposal. Define outcomes, errors, precedence, saved formats, and compatibility with existing recordings. Add shared cases for thresholds, ties, malformed responses, Unicode, request bytes, and digests. Expose the pure core through a small public interface. Use a second test adapter to prove that another wire format fits.
4. **Build the Rust client.** Make the command-line tool its first caller. Expose single judgments, question files, recordings, and collection operations over Rust data. Define ownership, ordering, partial failure, cancellation, and cleanup. Keep command-line dependencies optional for library users.
5. **Prove and build Python.** Bind `decide` first and install its package on a clean machine without Rust. Then add the remaining judgments, question files, typed results, errors, recording support, and collection operations. Preserve original objects when filtering. Make boolean conversion of a decision fail explicitly.
6. **Verify the installed interfaces.** Run shared cases through all three interfaces. Test retries, cancellation, secrecy, and batch failures against local servers. Evaluate one real workflow against independent labels. Check installation, supported platforms, versions, and executable examples separately for the binary, Rust crates, and Python package.

## Decisions to settle as work begins

- Distinguish exact run replay from a cache that keeps one response per request. Define model changes and compatibility with saved data.
- Resolve async support and cancellation. The study proposes async Rust and both Python clients; the Rust standards require evidence before adding an async runtime.
- Keep partial results compatible with bounded memory. The proposed default yields completed rows and reports the failure position and completion count. The caller retains any rows it needs.
- Choose supported platforms and Python versions before completing package checks.

One question file and recorded exchange must produce matching judgments and request identities through all three installed interfaces, with no key and no network. Local-server tests cover behavior that replay cannot exercise.
