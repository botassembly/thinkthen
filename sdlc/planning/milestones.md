# Milestones

Status: accepted 2026-10-01 on Ian's direction. Ian can overturn any milestone, criterion or placement. The steps that move work through a release live in [release-process.md](release-process.md). Branching follows [ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md).

Every open ticket and open issue carries a `Milestone:` line with `0.1`, `0.2` or `later`. The lists below come from those lines. `grep -rn '^Milestone:' sdlc/tickets sdlc/issues` reads them until `pm` supports milestones; the pm team was asked on 2026-10-01. Whoever changes a `Milestone:` line updates the list here in the same commit.

## 0.1

Outcome: the first public release, on every registry, with every surface and binding (ruling 10 of [cleanup-2026-09-30.md](cleanup-2026-09-30.md)).

Exit criteria, all met on 2026-10-03. 0.1.1 is published on every registry, and GitHub release v0.1.1 is public. The record is [0128-release-0-1.md](../records/0128-release-0-1.md).

1. Done: a clean release rehearsal. Every job passed through `draft` in runs 36945370940, 36998358908 and 37010060315.
2. Done: release QA's round 5 was clean on `checkpoint/surfaces/2026-10-02-1` (`4e880cdf6`).
3. Done: registry setup for every registry the release workflow publishes to. crates.io, PyPI, npm and RubyGems are set up. NuGet publishes by trusted publishing. pub.dev holds `thinkthen_dart` 0.0.1 under its publisher, with automated publishing enabled. Maven Central has its namespace verified, its secrets set and its signing key on the public key servers. The Homebrew tap repo `botassembly/homebrew-thinkthen` exists with its deploy key. The `release` environment has required reviewers and a `v*` tag policy. The `v-tags` ruleset is in place, and `RELEASE_ARMED` is `true`.
4. Done: tickets 0149 and 0157 are closed.
5. Done: Ian gave the go. Release run 37035814818 from `v0.1.0` published only Maven Central, pub.dev and the Homebrew tap. Release run 37059415069 from `v0.1.1` published every registry, after Ian allowed direct publishing on npm for the rerun.

R-universe and Packagist are set up. The registry repo `botassembly/botassembly.r-universe.dev` exists, and Ian installed the R-universe GitHub app on `botassembly`. Packagist lists `botassembly/thinkthen`, and its GitHub webhook works; its last delivery returned 202. The release workflow has no job for either one. Packagist reads the `v0.1.0` tag through its webhook. R-universe builds from the published GitHub release. Ticket 0128's Phase 4 step 8 includes the R-universe and Packagist install checks.

Blockers, refreshed 2026-10-03: none for the release. Ticket 0128 waits on its public install checks, Phase 4 step 8, and the Pages deploy that follows. Two items wait on Ian: confirming the local registry tokens are deleted, and deciding the retained history step of ticket 0128. The fresh one-commit history did not happen before the repository went public, and the published tags and the Go module proxy now name existing commits.

The 2026-10-02 sweep closed twelve 0.1 tickets that waited only on release-target proof. Ticket 0374's installed-file cases ran on all four release runners in run 37010060315. The sweep also closed the README issue. Its overhead sentence landed in `248bc6aa4`.

Tickets 0149 and 0157 closed on 2026-10-02 on the coordinator's ruling. Ticket 0231's macOS 26 proof, with Intel under Rosetta, is enough for 0.1. Rehearsal runs 36998358908 and 37010060315 also built and smoked on the `macos-15` and `macos-15-intel` runners.

Path to 0.1:

1. Done: ticket 0374 landed the panic and token-cap cases in the installed-file checks.
2. Done: a clean rehearsal passed on all four targets (ticket 0128 phase 3b). Run 36945370940 passed on main `94d0500c0` at version 0.0.1. Run 36998358908 passed on main `f65faea4e` at version 0.1.0.
3. Done: the checkpoint `checkpoint/surfaces/2026-10-02-1` was tagged on `4e880cdf6`. Release QA round 5 ran on it and was clean.
4. Done: ticket 0387 landed the release commit at `ff7120f89`. The coordinator tagged `rc/0.1.0-rc.1` on `4e880cdf6` and cut `release/0.1` there. Run 37010060315 passed from `release/0.1`.
5. Done: ticket 0389 landed on main and on `release/0.1`. Registry setup and the arming switch are done. Tickets 0149 and 0157 closed.
6. Done: ticket 0391 fixed the PyPI, npm and NuGet steps that failed in the 0.1.0 run. Checkpoint `checkpoint/surfaces/2026-10-02-2` and rehearsal 37048376945 passed on `release/0.1`. Ian tagged `v0.1.1` at `9463cef05`, and run 37059415069 published it.
7. Done: the public install checks of ticket 0128 phase 4 step 8 ran on 2026-10-03. Left: the Pages deploy of ticket 0392.
8. Left: ticket 0394 and a 0.1.2 release. On macOS 26, `gem install thinkthen` installs the 0.0.1 placeholder.

Open items:

- [0128: Release and install for 0.1](../tickets/0128-release-and-install.md)
- [Release and install for 0.1](../issues/2026-09-25-release-and-install-for-0-1.md)
- [0394: macOS gems install on every macOS version](../tickets/0394-darwin-gem-platform.md), for 0.1.2

Closed 2026-10-02 by the release rehearsals: 0222, 0224, 0226, 0227, 0231, 0268, 0269, 0270, 0271, 0272, 0273 and 0299, and the issue [README: where to get a key, how to change the backend, and the overhead line](../issues/closed/2026-09-29-readme-key-backend-and-overhead-lines.md). Closed 2026-10-02 on the coordinator's ruling: [0149](../tickets/0149-sql-settings.md) and [0157](../tickets/0157-library-size-and-retry-settings.md).

## 0.2

Ship the working core and the files/local-runtime additions approved by Ian on 2026-10-05. [The current plan](team-0-2-2026-10-04.md) owns order, completion status, delegated run approvals and exit criteria.

Core items through the default-recognize check have landed. Finish the current Windows corrections, then build files and folders for all ten functions and built-in llama.cpp and MLX routes alongside Ollama. One shared folder fixture covers CLI, Python and database examples; DuckDB and SQLite readers ship, while Postgres server-file access waits. Existing text, record and column calls retain their behavior.

Run the final authorized rehearsal and real Windows test over the expanded reviewed commit, then release QA and Ian's go. Missing required Windows behavior blocks release. [0415](../tickets/0415-signed-duckdb-community-listing.md) proceeds beside this work without holding release: offline community rehearsal, then the approved submission under imaurer. Listing publication remains DuckDB's decision.

Main carries 0.2.0; release/0.1 is frozen, and public install text stays 0.1.2 until 0.2 ships. dbt v1 remains the documented route; dbt v2 waits for a live signed listing. No grep alias or new semantic function is approved. Size the data-talk gaps before choosing any additional work; sizing does not admit every gap into 0.2.

## 0.3

Outcome: add the remaining Windows bindings after the 0.2 core release. These tickets stay open under Ian's 2026-10-05 ruling.

Future outcomes: support additional decision-provider endpoints and API versions, then images and other modalities when their contracts are known. These are outcomes only.

Open items:

- [0383: Windows stage 1: the Node addon ships for Windows x86-64](../tickets/0383-windows-node-addon.md)
- [0384: Windows stage 1: the C# package loads the Windows DLL](../tickets/0384-windows-csharp.md)
- [0385: Windows stage 1: the JVM binding loads the Windows DLL](../tickets/0385-windows-jvm.md)

## later

Outcome: no release is promised. Each item waits on its own trigger, such as a user's request, an upstream fix or Ian's ruling.

Exit criteria: none. An item moves to a numbered milestone when its trigger fires or Ian places it.

Blockers: none.

Open items:

- [Windows static-library distribution waits for stage 3](../issues/2026-10-05-windows-static-library-waits-for-stage-3.md)

- [0406: Rank questions preserve criteria and score ordering across core surfaces](../tickets/0406-rank-question-equivalence.md)
- [0407: Supply independent context for each record](../tickets/0407-per-record-context.md)
- [0408: Expose complete question probabilities through C JSON and SQL details](../tickets/0408-j1-full-probabilities.md)
- [0409: Match TypeScript rank and find declarations to runtime](../tickets/0409-typescript-rank-find-contract.md)
- [0410: Complete dataframe function coverage](../tickets/0410-column-function-equivalence.md)
- [0413: Supply candidate options independently for each record](../tickets/0413-options-per-record-equivalence.md)
- [0414: Define separate context on aggregate functions](../tickets/0414-separate-shared-context.md)
- [0417: SQL rank accepts a question set](../tickets/0417-sql-rank-question-sets.md)

- [0411: Prove changed reading rules on every binding route](../tickets/0411-binding-reading-replay-proof.md)
- [0412: Document R index adapters in the binding contract](../tickets/0412-r-result-contract.md)
- [0418: Language rank accepts a question set](../tickets/0418-binding-rank-question-sets.md)

- [0119: Functional audit of the engine tests](../tickets/0119-mutation-audit-of-the-engine-tests.md)
- [0295 — Polars namespace and SIGINT (F4)](../tickets/0295-polars-namespace-and-sigint.md)
- [0296 — pandas Series accessor (F5)](../tickets/0296-pandas-series-accessor.md)
- [0300: Caller-priced cost in call and run facts](../tickets/0300-caller-priced-call-cost.md)
- [The new-user stumble register](../issues/2026-09-20-new-user-stumble-register.md)
- [Annotate options from a file or a record](../issues/2026-09-23-annotate-options-from-a-file-or-a-record.md)
- [Rank by graded relevance: custom weights and rank fusion](../issues/2026-09-24-rank-by-graded-relevance-for-search-reranking.md)
- [Docs and how-tos owed](../issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md)
- [Public library API gaps](../issues/2026-09-25-public-library-api-gaps.md)
- [recognize and relate: scale](../issues/2026-09-25-recognize-and-relate-scale-and-shape.md)
- [Every surface should give back what Jev tells us about each run](../issues/2026-09-26-every-surface-should-give-back-run-facts.md)
- [Relation pairs span the whole text](../issues/2026-09-26-relation-pairs-span-every-mention-and-the-whole-text.md)
- [Tuning loop asks: cases to label, repeated runs, and cost beside the score](../issues/2026-09-27-nothing-lists-the-uncertain-hard-or-flip-flopping-cases.md)
- [A lone record over the size setting is sent, and one backend refusal stops the file](../issues/2026-09-30-a-lone-oversized-record-is-sent-anyway.md)
- [A usage write that fails after a good start is silent on the libraries](../issues/2026-09-30-a-usage-write-that-fails-after-a-good-start-is-silent-on-the-libraries.md)
- [A batch command that runs many questions in one process, after 0.1](../issues/2026-09-30-batch-command-runs-many-questions-in-one-process.md)
- [OpenTelemetry traces for backend calls, after 0.1](../issues/2026-09-30-opentelemetry-traces-after-0-1.md)
- [The Polars door cannot test lazy streaming](../issues/2026-09-30-polars-door-cannot-test-lazy-streaming.md)
- [The proxy: a second program that serves the functions](../issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md)
- [Three status phrases pass the site word check](../issues/2026-09-30-site-replay-folders-have-no-fixture.md)
- [About a third of the spec's "no calls" edges show only after a real send, and the spec lags the build in places](../issues/2026-09-30-spec-no-calls-edges-need-a-real-send.md)
- [The systemone adapter sends criteria descriptions as JSON objects that Ollama refuses](../issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md)
- [Zig 0.15.2's linker drops constant alignment](../issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md)
- [`tag` takes a separate cutoff for each label](../issues/2026-10-01-tag-cutoff-per-label.md)
- [The command's own time: the usage fsync and a new connection per run](../issues/2026-10-03-command-overhead-fsync-and-connection.md)
- [Two site paths point at bench runs that left the bench's main branch](../issues/closed/2026-10-03-site-bench-paths-point-at-deleted-runs.md)
