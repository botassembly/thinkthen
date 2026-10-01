# Milestones

Status: accepted 2026-10-01 on Ian's direction. Ian can overturn any milestone, criterion or placement. The steps that move work through a release live in [release-process.md](release-process.md). Branching follows [ADR 0116](adr/0116-release-branches-cut-at-the-release-candidate.md).

Every open ticket and open issue carries a `Milestone:` line with `0.1`, `0.2` or `later`. The lists below come from those lines. `grep -rn '^Milestone:' sdlc/tickets sdlc/issues` reads them until `pm` supports milestones; the pm team was asked on 2026-10-01. Whoever changes a `Milestone:` line updates the list here in the same commit.

## 0.1

Outcome: the first public release, on every registry, with every surface and binding (ruling 10 of [cleanup-2026-09-30.md](cleanup-2026-09-30.md)).

Exit criteria:

1. A clean release rehearsal: every job passes through `draft`.
2. Release QA's latest round is clean on the latest checkpoint.
3. The docs team has finished registry setup: PyPI, npm, crates.io, RubyGems, NuGet, Maven, pub.dev and the Homebrew tap.
4. Ian gives the go to dispatch the real release.

Blockers:

- The rehearsal has not yet passed. Ticket 0128 phase 3b records each attempt.
- Registry setup belongs to the docs team, and Ian approves each outward step.
- The README overhead line waits on marketing's overhead benchmark.

Most DuckDB, language workflow and runner tool tickets below keep open only proofs on the release targets. A clean rehearsal on all four targets supplies those proofs. Ticket 0374 added panic-isolation and token-cap cases to the installed-file checks the rehearsal runs. Tickets 0226, 0227 and 0299 stay in 0.1. Their proof comes from ticket 0374 plus the rehearsal.

Path to 0.1:

1. Done: ticket 0374 landed the panic and token-cap cases in the installed-file checks.
2. A clean rehearsal passes on all four targets (ticket 0128 phase 3b).
3. A fresh checkpoint, the fourth, follows the clean rehearsal. Release QA runs its final round on it.
4. The release candidate: the coordinator tags it and cuts `release/0.1` (ADR 0116, release-process.md section 5). The release commit lands at that cut, as ticket 0128 phase 4 step 3: `versions --set 0.1.0`, the publish flags dropped, the `CHANGELOG.md` date, the README "Install" section and the site's install lines.
5. Ian gives the go, and ticket 0128 phase 4 runs the release.

Open items:

- [0128: Release and install for 0.1](../tickets/0128-release-and-install.md)
- [0149: Give SQL the engine settings and charge every send](../tickets/0149-sql-settings.md)
- [0157: Expose request size and retry counts through the libraries](../tickets/0157-library-size-and-retry-settings.md)
- [0222: Batch DuckDB record vectors and warm groups](../tickets/0222-duckdb-record-batching.md)
- [0224: Find one best unit from an ordered SQL group](../tickets/0224-sql-group-find.md)
- [0226: Keep caught native panic payloads out of diagnostics](../tickets/0226-native-panic-diagnostics.md)
- [0227: Keep caught language-binding panic payloads out of diagnostics](../tickets/0227-language-panic-diagnostics.md)
- [0231: Ship the DuckDB C++ extension on the other release platforms](../tickets/0231-duckdb-release-platforms.md)
- [0268: Start the reviewed language archives in the manual release workflow](../tickets/0268-release-workflow-language-packages.md)
- [0269: Gate Swift and Zig source files in the Linux x86 release workflow](../tickets/0269-swift-zig-linux-workflow.md)
- [0270: Gate PHP and Dart source packages in the Linux x86 release workflow](../tickets/0270-php-dart-linux-workflow.md)
- [0271: Prepare Ada, GNU Objective-C and COBOL for the Linux x86 release workflow](../tickets/0271-ada-objc-cobol-linux-workflow.md)
- [0272: Prepare C# and JVM managed archives for the Linux x86 release workflow](../tickets/0272-csharp-jvm-linux-workflow.md)
- [0273: Select and verify Linux x86 language runner tools](../tickets/0273-linux-language-runner-tools.md)
- [0299 — Estimated input admission total](../tickets/0299-token-cap-contract.md)
- [Release and install for 0.1](../issues/2026-09-25-release-and-install-for-0-1.md)
- [README: where to get a key, how to change the backend, and the overhead line](../issues/2026-09-29-readme-key-backend-and-overhead-lines.md)

## 0.2

Outcome: Windows support and the first features after 0.1. 0.2 work lands on main after the `release/0.1` cut (ADR 0116).

Exit criteria, a coordinator default Ian can overturn:

1. Every item below lands or moves to `later`.
2. A clean rehearsal and a clean release QA round on the latest checkpoint.
3. Ian gives the go.

Blockers:

- The `release/0.1` cut. Only work that cannot change 0.1 behavior lands before it.

Windows work follows [windows.md](windows.md):

- [0373: Windows stage 0](../tickets/0373-windows-stage-0.md), landed before the cut. The root workspace and the C door build and pass their tests on Windows, and nothing ships. It changes no 0.1 behavior.
- Windows stage 1: the command line, the Rust crate, the C DLL, the Python wheel, the Node addon, C# and the JVM ship for Windows x86-64, and findings W1 to W7 close. The stage 1 report in [windows.md](windows.md#stage-1-difficulty-report) sizes it at 9 to 11 tickets' worth of work and 1,600 to 3,400 lines. Ian ruled on 2026-10-01 to keep all Windows work in 0.2. Tickets 0380 to 0385 carry it, in slices. Each waits for the `release/0.1` cut. 0380 lands first, and 0384 and 0385 also wait for 0381.

Open items:

- [0377: Each language binding and SQL extension names a backend in code](../tickets/0377-binding-backends.md)
- [0380: Windows stage 1: the command line and the Rust crate ship for Windows x86-64](../tickets/0380-windows-command-line-and-rust-crate.md)
- [0381: Windows stage 1: the C library ships as a DLL for Windows x86-64](../tickets/0381-windows-c-dll.md)
- [0382: Windows stage 1: the Python wheel ships for Windows x86-64](../tickets/0382-windows-python-wheel.md)
- [0383: Windows stage 1: the Node addon ships for Windows x86-64](../tickets/0383-windows-node-addon.md)
- [0384: Windows stage 1: the C# package loads the Windows DLL](../tickets/0384-windows-csharp.md)
- [0385: Windows stage 1: the JVM binding loads the Windows DLL](../tickets/0385-windows-jvm.md)
- [A Flutter app file in the release bundle](../issues/2026-10-01-a-flutter-app-file-in-the-release-bundle.md)
- [The bindings and SQL extensions cannot name a backend](../issues/2026-10-01-bindings-and-sql-extensions-name-no-backend.md)
- [`rank --threshold P` keeps only records at or above a probability](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md)
- [`score --level NAME=MEANING` describes a level on the command line](../issues/2026-10-01-score-levels-described-on-the-command-line.md)
- [SQL named forms `thinkthen_rank` and `thinkthen_filter`](../issues/2026-10-01-sql-names-for-rank-and-filter.md)

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
- [A proxy service in front of the backends, after 0.1](../issues/2026-09-30-proxy-service-for-shared-limits-and-traces.md)
- [Three status phrases pass the site word check](../issues/2026-09-30-site-replay-folders-have-no-fixture.md)
- [About a third of the spec's "no calls" edges show only after a real send, and the spec lags the build in places](../issues/2026-09-30-spec-no-calls-edges-need-a-real-send.md)
- [The systemone adapter sends criteria descriptions as JSON objects that Ollama refuses](../issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md)
- [Zig 0.15.2's linker drops constant alignment](../issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md)
- [`tag` takes a separate cutoff for each label](../issues/2026-10-01-tag-cutoff-per-label.md)
