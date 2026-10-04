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

Outcome: Windows support and the first features after 0.1. 0.2 work lands on main after the `release/0.1` cut (ADR 0116).

Ian approved the 0.2 scope and lane order on 2026-10-04. 0.2 ships when release safety, the backends, the search flags on rank, extract, the recipes with their numbers, and Windows stage 1 have landed. The proxy, the hosted screens, the decision store, link, verify and qualify as functions, Markdown repair, tables and the navigate command are not in 0.2. There are no 0.1.x patch releases; ticket 0397 freezes `release/0.1`.

Lanes, in order:

- First, in any free lane: [0397: Main moves to 0.2.0 and release/0.1 freezes](../tickets/0397-main-moves-to-0-2-0.md).
- Lane 1: Windows, tickets 0380 to 0385 below and what follows them.
- Lane 2: [0398: Release safety](../tickets/0398-release-safety.md), then [0399: A backend sets its request path, Perplexity is built in, and OpenRouter gets both decide sides](../tickets/0399-backend-path-perplexity-openrouter.md), then [0400: One setup format per provider and a default throttle of 8](../tickets/0400-provider-setups-and-concurrency.md), then 0377 below. 0400 slice A must be usable before the experiments team's new-model day run in the week of 2026-10-05.
- Lane 3: [0405: Audit the ten functions](../tickets/0405-audit-ten-functions.md) first on Ian's 2026-10-04 update. Answer filter, rank and grep before implementation of [0401: Search flags on rank](../tickets/0401-rank-search-flags.md), then [0402: The docs tell one story](../tickets/0402-docs-tell-one-story.md), then [0403: The DuckDB extension ships a build for DuckDB v1.5.4](../tickets/0403-duckdb-extension-for-dbt-v2.md).
- Gaps in any lane: [0404: Tech debt cut, with tests held to behavior](../tickets/0404-tech-debt-and-tests-held-to-behavior.md).
- Waiting on experiments: extract (its own function or a mode of annotate), the "none" wording for choose, and each recipe's measured numbers.

Completed foundation: [0397](../tickets/0397-main-moves-to-0-2-0.md) moves main to 0.2.0 and freezes `release/0.1`.

Exit criteria, a coordinator default Ian can overturn:

1. Every item below lands or moves to `later`.
2. A clean rehearsal and a clean release QA round on the latest checkpoint.
3. Ian gives the go.

Blockers:

- None. `release/0.1` is frozen under Ian's ruling of 2026-10-04. Main carries 0.2.0 (ticket 0397). Fixes land on main and ship in 0.2.

Windows work follows [windows.md](windows.md):

- [0373: Windows stage 0](../tickets/0373-windows-stage-0.md), landed before the cut. The root workspace and the C door build and pass their tests on Windows, and nothing ships. It changes no 0.1 behavior.
- [0379: The root workspace tests pass on macOS](../tickets/0379-macos-root-suite.md), landed before the cut. It changes tests only.
- Windows stage 1: the command line, the Rust crate, the C DLL, the Python wheel, the Node addon, C# and the JVM ship for Windows x86-64, and findings W1 to W7 close. The stage 1 report in [windows.md](windows.md#stage-1-difficulty-report) sizes it at 9 to 11 tickets' worth of work and 1,600 to 3,400 lines. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. Tickets 0380 to 0385 carry it, in slices. Each waits for the `release/0.1` cut. 0381 to 0385 wait for 0380 slice A, and 0384 and 0385 also wait for 0381 slice A.

Open items:


- [0377: Each language binding and SQL extension names a backend in code](../tickets/0377-binding-backends.md)
- [0380: Windows stage 1: the command line and the Rust crate ship for Windows x86-64](../tickets/0380-windows-command-line-and-rust-crate.md)
- [0381: Windows stage 1: the C library ships as a DLL for Windows x86-64](../tickets/0381-windows-c-dll.md)
- [0382: Windows stage 1: the Python wheel ships for Windows x86-64](../tickets/0382-windows-python-wheel.md)
- [0383: Windows stage 1: the Node addon ships for Windows x86-64](../tickets/0383-windows-node-addon.md)
- [0384: Windows stage 1: the C# package loads the Windows DLL](../tickets/0384-windows-csharp.md)
- [0385: Windows stage 1: the JVM binding loads the Windows DLL](../tickets/0385-windows-jvm.md)
- [0393: npm publishes through staged publishing](../tickets/0393-npm-staged-publishing.md)
- [A Flutter app file in the release bundle](../issues/2026-10-01-a-flutter-app-file-in-the-release-bundle.md)
- [The bindings and SQL extensions cannot name a backend](../issues/2026-10-01-bindings-and-sql-extensions-name-no-backend.md)
- [`rank --threshold P` keeps only records at or above a probability](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md)
- [`score --level NAME=MEANING` describes a level on the command line](../issues/2026-10-01-score-levels-described-on-the-command-line.md)
- [SQL named forms `thinkthen_rank` and `thinkthen_filter`](../issues/2026-10-01-sql-names-for-rank-and-filter.md)
- [SQLite `thinkthen_find` drops its `model` setting](../issues/2026-10-01-sqlite-find-drops-its-model-setting.md)
- [`rank --details` prints `"value": null` on every row](../issues/2026-10-03-rank-details-prints-value-null.md)
- [Search features: positions, several questions, and grep-style output](../issues/2026-10-03-search-features-positions-several-questions-and-grep-style-output.md)
- [Help and warning gaps from the transcript how-to](../issues/2026-10-03-help-gaps-from-the-transcript-how-to.md)
- [A glossary for call, request and decision](../issues/2026-10-03-glossary-call-request-decision.md)
- [AGENTS.md tells outside agents how to report](../issues/2026-10-03-agents-md-tells-outside-agents-how-to-report.md)
- [The doc tests gate each checkpoint and each release](../issues/2026-10-03-doc-tests-gate-checkpoints-and-releases.md)
- [`native_install` loses the C library when `CARGO_TARGET_DIR` is set](../issues/2026-10-03-native-install-ignores-cargo-target-dir.md)
- [The pandas binding refuses a Series on four functions, and DuckDB's recognize takes no kind descriptions](../issues/2026-10-03-pandas-series-and-duckdb-recognize-descriptions.md)
- [The `ruby` platform gem on RubyGems is still the 0.0.1 placeholder](../issues/2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md)
- [The Polars deadline test races its own deadline under load](../issues/2026-10-03-polars-deadline-test-races-its-deadline-under-load.md)
- Release process, from the 0.1 releases: [the rehearsal never runs the publish steps](../issues/2026-10-04-rehearsal-never-runs-the-publish-steps.md), [the public install checks are done by hand](../issues/2026-10-04-public-install-checks-are-done-by-hand.md), [the release tag can name an unrehearsed commit](../issues/2026-10-04-release-tag-can-differ-from-the-rehearsed-commit.md), [each patch release costs two hand passes](../issues/2026-10-04-each-patch-release-costs-two-hand-passes.md), [the R package's Linux install and pre-release proof](../issues/2026-10-04-r-install-on-linux-and-before-release.md), [two release secrets remain](../issues/2026-10-04-two-release-secrets-remain.md), and [a release needs two approvals](../issues/2026-10-04-a-release-needs-two-approvals.md)
- Draft functions, each waiting on its experiment: [link](../issues/2026-10-03-draft-function-link.md), [verify](../issues/2026-10-03-draft-function-verify.md), [extract](../issues/2026-10-03-draft-function-extract.md), [navigate](../issues/2026-10-03-draft-function-navigate.md), and the [Markdown repair tool](../issues/2026-10-03-draft-markdown-repair-tool.md)

## later

Outcome: no release is promised. Each item waits on its own trigger, such as a user's request, an upstream fix or Ian's ruling.

Exit criteria: none. An item moves to a numbered milestone when its trigger fires or Ian places it.

Blockers: none.

Open items:

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
- [Two site paths point at bench runs that left the bench's main branch](../issues/2026-10-03-site-bench-paths-point-at-deleted-runs.md)
