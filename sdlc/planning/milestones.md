# Milestones

This is ThinkThen's one plan. It says what each release builds, why, and in what order. Ian can overturn any milestone, criterion or placement. pm holds the milestones and every ticket's status, order and dependencies. The steps that move work through a release live in [release-process.md](release-process.md). Branching follows [ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md).

## Hardest problems

ThinkThen puts decision models in your code from about eighteen languages, three databases, a command line and an MCP server, all over one Rust core. These are the hardest problems in that work.

1. **One answer everywhere.** Each language library has computed a call's final answer and probability from raw results, so the copies drift apart. Rust must compute the answer once and every language must only pass it through.
2. **One native path.** The languages reach Rust through a C session, fixed memory layouts, a JSON session and several direct Rust bridges. Every contract change lands three or four ways until one path remains.
3. **A stable core contract.** The C interface and the result packet changed many times while the bindings were being built. The core has to freeze before language work starts, and stay frozen.
4. **Concurrency and cancellation in every language.** Each language ran its own loop to wait for results and read errors from a per-thread slot. One blocking call with its own error result must work the same way under every language's threading model.
5. **Native packaging for every ecosystem.** Each package manager wants native code its own way: Python wheels, npm, Maven, Swift and Dart binary assets, Go modules, R, Ruby gems, CMake. Each must install on every supported platform without compiling Rust on the user's machine.
6. **A test machinery smaller than the product.** The checks grew to about 70,000 lines and often tested the build environment instead of ThinkThen. They need to run on the real checkout, fail when a tool is missing, and shrink.
7. **One environment on every machine.** Checks run on an arm64 Mac, Linux on two architectures and hosted Linux runners. One pinned container and one declared tool list have to give the same result everywhere.
8. **Secrets never leak.** Provider keys must never appear in any output, error message or log from any language. Provider error text must pass through an allow-list, not a pattern filter.
9. **Release proof without paid calls.** Every check replays saved model responses, so a release qualifies offline and costs nothing. The recordings must stay faithful to what the providers actually return.
10. **Database hosts that move.** DuckDB, PostgreSQL and SQLite each load ThinkThen as an extension, and their versions and build rules change on their own schedules.
11. **Long Linux runs that time out.** The full consumer runs time out on Linux and the cause has not been measured. It must be measured before it is fixed.
12. **An answer store that lasts.** The next release adds one answer database. Its schema must already hold what later curation and evaluation need, and many writers must share one file safely.

## 0.2.1

Outcome: the first release after 0.1, published as 0.2.1, with the 0.2 scope on a rebuilt basis. Rust computes every call's answer. Each language is a thin, idiomatic, properly packaged wrapper over one C entry point or one direct Rust bridge. One pinned environment per architecture runs every check. Adding a core feature mostly touches only Rust. Ruling and root causes: [0.2.1 rebuilds the basis](../decisions/2026-10-18-0-2-1-rebuilds-the-basis.md). Findings: [the 2026-10-18 quality review](../records/2026-10-18-0-2-quality-review.md).

Order. pm dependencies carry the detail. Tickets that do not collide run in parallel.

1. The core contracts: the test machinery, the release workflows, the Rust core, the host crate, the settings parser, the Rust library, the C interface's run call with one input path and one result packet, the value tree, the C library, the command line, the installers, the generator, the binding contract and conformance.
2. The freeze names the contract. [0688](../tickets/0688-name-the-core-freeze-commit.md) names the first main commit after the core contract tickets and 0744 whose configured landing gate and native Rust/C smoke and package pass. The [core-freeze decision](../decisions/2026-10-20-0-2-1-core-freeze.md) names the contract revision and its change rules. The three-platform checkpoint run does not gate the freeze: [the freeze and qualification ruling](../decisions/2026-10-20-freeze-names-the-contract.md).
3. [0740](../tickets/0740-surface-checks-in-container.md) lands alone. It runs one surface's check in the Linux container, and every later container proof uses it.
4. The first language lanes: Objective-C ([0676](../tickets/0676-objective-c-thin-results.md)), Polars ([0679](../tickets/0679-polars-frame-types-from-packet.md)), R ([0682](../tickets/0682-r-thin-binding.md)), Ruby ([0683](../tickets/0683-ruby-native-values.md)) and the SQLite spellings ([0714](../tickets/0714-delete-sqlite-removed-spellings.md)). The routine timing fix ([0605](../tickets/0605-routine-tests-under-timeout.md)) lands before this first batch lands.
5. Every other language and the three database extensions follow as pm dependencies say. Each language ticket deletes its own projection, session loop and fixture consumer. A final ticket deletes the old shared C paths and answer rules.
6. Qualification ([0687](../tickets/0687-qualify-the-0-2-1-candidate.md)) runs the three-platform checkpoint on one frozen main revision. It needs 0605, 0743, 0745, 0746 and 0748 as well as the language work.

Exit criteria:

1. Every 0.2.1 ticket is closed, and no language or template code computes a call's answer.
2. The release qualification ticket passes on one frozen main revision: every language library builds natively and passes its installed tests on every platform it supports, with the rehearsal and the hosted Linux jobs on the same revision. Coverage does not gate it.
3. The artifact QA control job passes on the tag, and Ian gives the go to publish.

## 0.2.2

Outcome: the docs and site sell one idea after 0.2.1 ships: put Jev-class decision models in your code, whatever model, language or database you use. It is a docs change with no release of its own (Ian, 2026-10-12). It starts with a trial: a reader who has never seen ThinkThen installs it and asks one question from the public documents alone, and the docs are fixed where that reader got stuck.

## 0.3

Outcome: ThinkThen owns your answers. One answer database, designed from the start with the full schema that 0.4's curation and evals need. A toolbox and a `run` command. MCP as one tool. Publishing. Spreadsheets join only if the writer prototype passes. Libraries gain only the ten functions and `run`; the toolbox, answers and admin stay in Rust and the command line. Ruling: [three releases after 0.2](../decisions/2026-10-14-three-releases-after-0-2.md).

Experiments come before 0.3 product tickets. None changes product code, spends model money or publishes, and none starts before 0.2.1 is published: [0.3 plans while 0.2 ships](../decisions/2026-10-17-0-3-plans-while-0-2-ships.md). [0583](../tickets/0583-experiment-duckdb-one-build-across-versions.md) runs first. [0584](../tickets/0584-experiment-answer-schema-covers-0-4.md), [0585](../tickets/0585-experiment-many-writers-one-sqlite-file.md), [0586](../tickets/0586-experiment-split-wording-evidence-context.md), [0587](../tickets/0587-experiment-command-inside-python-and-npm.md) and [0588](../tickets/0588-experiment-spreadsheet-writer-round-trip.md) follow in parallel. 0584 and 0586 set the database ticket.

## 0.4

Outcome: ThinkThen grades and improves your answers. Curation fixes data: with labels, it finds where models and labels disagree; without labels, it queues likely-wrong first answers, lowest confidence first, then disagreement between models. Evals and the ranking setting run over the 0.3 answer database. Agents and notebooks curate through the command line; a web UI comes later.

## later

Outcome: no release is promised. The proxy and document structure wait here for 0.5 under the [three-release ruling](../decisions/2026-10-14-three-releases-after-0-2.md). Each item waits on its own trigger, such as a user's request, an upstream fix or Ian's ruling, and moves to a numbered milestone when its trigger fires or Ian places it.
