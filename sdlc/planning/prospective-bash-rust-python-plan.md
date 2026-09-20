# Prospective plan for Bash, Rust, and Python

Date: 2026-09-20. Status: temporary, prospective. Character budget: 4,000.

Tickets are written as work begins. The [main plan](plan.md) records current work; the [review report](../issues/2026-09-20-full-project-review-and-follow-up.md) holds the evidence. [ADR 0017](adr/0017-libraries-over-one-bound-core.md) remains proposed.

## Destination

Bash users call the command-line tool. Rust applications use a client library over the public pure core. Python applications install a package bound to that core. All three share question files, judgment rules, request bytes, results, and recordings. Version one has seven commands: `decide`, `choose`, `score`, `filter`, `rank`, `annotate`, and `find`.

The core keeps judgment rules. Hosts own I/O, credentials, retries, and concurrency. Policy stays in the caller or a `jq` transform.

## Current state

Tickets 0024 through 0027 closed four defects. Ticket 0034 completed the live-authority repair and migrated every registered worktree to one audited allowance. Six first-pass findings are closed. Seventeen small repairs remain: four first-pass, ten second-pass, two security leftovers, and the hostless-address diagnostic.

Parallel duplicate cache misses can pay twice and fail if replies differ. Probability totals require machine precision without a provider rounding promise. Both contracts need an explicit operating assessment.

## Version-one sequence

1. **Build `annotate`.** Ticket 0015 is the next product work. It groups questions, turns how-tos 39 and 14 green, and measures the accepted question count through the shared live authority.
2. **Build `find`.** First compare it with `rank --top 1` on documents of 100 to 250 lines. Write the ticket from that evidence, then build the command and how-to 15 if the result still supports it.
3. **Finish the workflows.** Write flagship how-to 16 and its tested policy transform. Complete compare and sweep over every value shape, the monitors, grouped sweep, and the human-label check. Decide again whether `report` earns a command.
4. **Prepare version one.** Finish the repair inventory, help, manual page, installation, agent skill, and how-to 18 against another server. Test the installed artifact. Publication timing remains Ian's decision.

Ticket 0034 keeps one durable authority across local worktrees and bounds interruption. Its ADR 0022 amendment replaces the FIFO protocol and permits checked recovery. One initialized installation owns the allowance; other machines need an explicit transfer. Pin the hostless-address sentence, document duplicate cache misses, and check ratchet commit messages on push. Close the 17 small repairs in batches before release. Collect response and rounded-total evidence during authorized probes before changing contracts. Write tickets as work begins; use an ADR for a contract change.

## After version one

No library work begins until Ian accepts or rewrites ADR 0017. It promises a Rust library but omits its host layer from the work order.

1. Expose the pure-core interface and refactor the existing Rust networking, file, retry, recording, and concurrency layer into the `thinkthen` library, with the binary as its first caller. Add shared cases and prove another wire format with a second test adapter. This combines the shared-core and Rust-client outcomes; it does not insert an extra stage before Python.
2. Bind `decide` in a Python spike and install a wheel without Rust. The spike tests the bound-core design.
3. Build Python in full, then verify all three interfaces against shared cases, recordings, local failures, installed artifacts, and independent labels.

JavaScript and TypeScript remain ADR 0017's later stage, outside this plan.
