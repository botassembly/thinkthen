# Prospective plan for Bash, Rust, and Python

Date: 2026-09-20. Status: temporary, prospective. Character budget: 4,000.

Ian requested this plan after the project review and follow-up. Tickets are written as work begins. The [main plan](plan.md) records current work and the [review report](../issues/2026-09-20-full-project-review-and-follow-up.md) records the evidence. [ADR 0017](adr/0017-libraries-over-one-bound-core.md) remains proposed.

## Destination

Bash users call the command-line tool. Rust applications use a client library over the public pure core. Python applications install a package bound to that core. All three share question files, judgment rules, request bytes, results, and recordings. Version one has seven commands: `decide`, `choose`, `score`, `filter`, `rank`, `annotate`, and `find`.

The core keeps judgment rules. Hosts own I/O, credentials, retries, and concurrency. Policy stays in the caller or a `jq` transform.

## Current state

Tickets 0024 through 0028 closed the five foundation defects found at `2c32524`. Tickets 0022 and 0029 through 0033 closed six findings from the first hands-on pass. Seventeen small repairs remain: four first-pass findings, ten second-pass findings, two security leftovers, and one hostless-address diagnostic with no page or exact test.

The follow-up confirmed three risks. The live-spend wrapper has an unbounded lock-held wait and difficult recovery. Parallel duplicate cache misses can pay twice and then fail if raw replies differ. Probability members must total one within machine precision without a recorded provider rounding promise. The last two contracts lack operating evidence.

## Version-one sequence

1. **Build `annotate`.** Execute ticket 0015. It groups several questions about one record, turns how-tos 39 and 14 green, and measures the question count accepted in one request. This is the next product work.
2. **Build `find`.** First compare it with `rank --top 1` on documents of 100 to 250 lines. Write the ticket from that evidence, then build the command and how-to 15 if the result still supports it.
3. **Finish the workflows.** Write flagship how-to 16 and its tested policy transform. Complete compare and sweep over every value shape, the monitors, grouped sweep, and the human-label check. Decide again whether `report` earns a command.
4. **Prepare version one.** Finish the repair inventory, help, manual page, installation, agent skill, and how-to 18 against another server. Test the installed artifact. Publication timing remains Ian's decision.

The review response is a companion track. Add checked live-lock recovery before `annotate` makes its paid measurement, while keeping durable precharge and fail-closed behavior. Pin the hostless-address sentence, warn about duplicate cache misses, use charged-authority language, and check ratchet commit messages on push. Close the 17 small repairs in batches before release. Collect exact-response and rounded-total evidence during authorized probes before changing either contract. Write tickets as coherent work begins and use an ADR only for a contract change.

## After version one

No library work begins until Ian accepts or rewrites ADR 0017. It promises a Rust library but omits its host layer from the work order.

1. Expose the pure-core interface and refactor the existing Rust networking, file, retry, recording, and concurrency layer into the `thinkthen` library, with the binary as its first caller. Add shared cases and prove another wire format with a second test adapter. This combines the shared-core and Rust-client outcomes; it does not insert an extra stage before Python.
2. Bind `decide` in a Python spike and install a wheel on a machine with no Rust. The spike tests the bound-core design.
3. Build Python in full, then verify all three interfaces against shared cases, recordings, local failures, installed artifacts, and independent labels.

JavaScript and TypeScript remain ADR 0017's later stage, outside this plan.
