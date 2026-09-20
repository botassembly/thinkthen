# Prospective plan for the command line and libraries

Date: 2026-09-20. Status: temporary, prospective. Character budget: 4,000.

Tickets are written as work begins. The [main plan](plan.md) records current work. The [repair-tranche review](../issues/2026-09-20-the-live-guard-grew-past-its-job.md) explains this revised order.

## Destination

Version one adds `annotate` and `find` to the five built commands. Bash uses the command-line tool. Rust, Python, JavaScript, and Ruby use libraries named `thinkthen`. All interfaces share questions, rules, request bytes, results, and recordings. Ian ruled the libraries and public name in.

The core keeps judgment rules. Hosts own files, credentials, retries, and concurrency. Policy stays in the caller or a `jq` transform.

## Current state

Tickets 0024 through 0027 fixed an interactive hang, unbounded paid dispatch, invalid answers, replay corruption, and false comparisons. Ticket 0035 replaced ticket 0034's supervisor with one append-only ledger and deleted its Rust harness. Eighteen how-tos are green.

Ticket 0015 built `annotate`, turned how-tos 39 and 14 green, and deleted absorbed page 08. Its mixed-question measurement kept the same values while reducing billed input from 915 tokens to 371. Its borderline measurement moved one of six answers from `false` to unresolved, which confirms the reference warning to keep a comparison's question group fixed.

## Next work

1. **Build `tag`.** Add the accepted label-appending command before the remaining input and repair work.
2. **Add CSV and DSV input.** These are input framings only. Every record the tool prints remains JSONL; no CSV or DSV writer or output transform enters the plan.
3. **Make one small correction pass.** Pin the hostless-address sentence, make every negative threshold spelling reach the same parser, correct `493 recorded distributions plus four standalone fixtures`, and correct stale pages.
4. **Coalesce duplicate cache misses.** One bounded per-digest lock prevents repeated rows under `--jobs` and separate writers from paying twice or failing on divergent replies.
5. **Build `find`.** Repeat the `rank --top 1` comparison on documents of 100 to 250 lines, then build the command and how-to 15 if the evidence holds.
6. **Write page 16 and finish the transforms.** Land the flagship triage workflow and its policy before library restructuring.
7. **Merge to one crate.** Make the command-line tool the first caller of the public Rust library shape.
8. **Prepare the release.** Complete help, the manual, installation, the agent skill, and how-to 18.

Document the cache limitation until step 4 lands. Keep the strict probability-total rule and collect rounding evidence during an authorized product probe. Drop the commit-message checker and separate response-stability experiment.

## After version one

The libraries are committed work. Before implementation, rewrite ADR 0017 whole around Ian's rulings: Rust, Python, JavaScript, and Ruby; one public name; bare answers by default and details on request; one `thinkthen` crate preferred, with a `thinkthen-cli` crate as the accepted fallback. Then:

1. Expose the pure rules through the `thinkthen` Rust library and make the binary its first caller. Shared cases prove the public behavior and a second test adapter proves the seam.
2. Spike one Python `decide` binding and install its wheel on a machine without Rust, then build Python fully.
3. Build JavaScript and TypeScript for Node first, then Ruby.
4. Verify all five interfaces against the same questions, recordings, failures, and installed artifacts.

R remains open. Claiming registry names remains Ian's outward action.
