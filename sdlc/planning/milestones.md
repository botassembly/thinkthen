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
- The site replay debt waits on site ticket 0047 slice F and a word check.

Most tickets below (DuckDB, panic diagnostics, language workflows and runner tools) keep open only proofs on the release targets. A clean rehearsal on all four targets supplies those proofs.

Open items:

- [0128: Release and install for 0.1](../tickets/0128-release-and-install.md)
- [0149: Give SQL the engine settings and charge every send](../tickets/0149-sql-settings.md)
- [0157: Expose request size and retry counts through the libraries](../tickets/0157-library-size-and-retry-settings.md)
- [0201: Move the DuckDB extension to the C++ API](../tickets/0201-duckdb-cpp-api.md)
- [0222: Batch DuckDB record vectors and warm groups](../tickets/0222-duckdb-record-batching.md)
- [0224: Find one best unit from an ordered SQL group](../tickets/0224-sql-group-find.md)
- [0226: Keep caught native panic payloads out of diagnostics](../tickets/0226-native-panic-diagnostics.md)
- [0227: Keep caught language-binding panic payloads out of diagnostics](../tickets/0227-language-panic-diagnostics.md)
- [0231: Ship the DuckDB C++ extension on the other release platforms](../tickets/0231-duckdb-release-platforms.md)
- [0249: Merge the language bindings](../tickets/0249-merge-the-language-bindings.md)
- [0267: Require the installed language files in one local Linux release bundle](../tickets/0267-local-language-release-bundle.md)
- [0268: Start the reviewed language archives in the manual release workflow](../tickets/0268-release-workflow-language-packages.md)
- [0269: Gate Swift and Zig source files in the Linux x86 release workflow](../tickets/0269-swift-zig-linux-workflow.md)
- [0270: Gate PHP and Dart source packages in the Linux x86 release workflow](../tickets/0270-php-dart-linux-workflow.md)
- [0271: Prepare Ada, GNU Objective-C and COBOL for the Linux x86 release workflow](../tickets/0271-ada-objc-cobol-linux-workflow.md)
- [0272: Prepare C# and JVM managed archives for the Linux x86 release workflow](../tickets/0272-csharp-jvm-linux-workflow.md)
- [0273: Select and verify Linux x86 language runner tools](../tickets/0273-linux-language-runner-tools.md)
- [0299 — Estimated input admission total](../tickets/0299-token-cap-contract.md)
- [The new-user stumble register](../issues/2026-09-20-new-user-stumble-register.md)
- [Release and install for 0.1](../issues/2026-09-25-release-and-install-for-0-1.md)
- [README: where to get a key, how to change the backend, and the overhead line](../issues/2026-09-29-readme-key-backend-and-overhead-lines.md)
- [Checkpoints publish no packages for 16 surfaces](../issues/2026-09-30-checkpoints-publish-no-source-wrapper-packages.md)
- [The PostgreSQL site samples run under no check, and three status phrases pass the word check](../issues/2026-09-30-site-replay-folders-have-no-fixture.md)

## 0.2

Outcome: Windows support and the first features after 0.1. 0.2 work lands on main after the `release/0.1` cut (ADR 0116).

Exit criteria, a coordinator default Ian can overturn:

1. Every item below lands or moves to `later`.
2. A clean rehearsal and a clean release QA round on the latest checkpoint.
3. Ian gives the go.

Blockers:

- The `release/0.1` cut. Only work that cannot change 0.1 behavior lands before it.

Windows work has no file on main yet. Ticket 0373, Windows stage 0, is in progress in a lane. It is safe before the cut, and its milestone is `0.2`. Windows stage 1 is `0.2` and has no ticket yet. ADR 0116 item 6 says what happens if Ian pulls it into 0.1.

Open items:

- [The bindings and SQL extensions cannot name a backend](../issues/2026-10-01-bindings-and-sql-extensions-name-no-backend.md)
- [`rank --threshold P` keeps only records at or above a probability](../issues/2026-10-01-rank-keeps-only-records-over-a-threshold.md)

## later

Outcome: no release is promised. Each item waits on its own trigger, such as a user's request, an upstream fix or Ian's ruling.

Exit criteria: none. An item moves to a numbered milestone when its trigger fires or Ian places it.

Blockers: none.

Open items:

- [0119: Functional audit of the engine tests](../tickets/0119-mutation-audit-of-the-engine-tests.md)
- [0295 — Polars namespace and SIGINT (F4)](../tickets/0295-polars-namespace-and-sigint.md)
- [0296 — pandas Series accessor (F5)](../tickets/0296-pandas-series-accessor.md)
- [0300: Caller-priced cost in call and run facts](../tickets/0300-caller-priced-call-cost.md)
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
- [About a third of the spec's "no calls" edges show only after a real send, and the spec lags the build in places](../issues/2026-09-30-spec-no-calls-edges-need-a-real-send.md)
- [The systemone adapter sends criteria descriptions as JSON objects that Ollama refuses](../issues/2026-09-30-systemone-adapter-sends-criteria-objects-ollama-refuses.md)
- [Zig 0.15.2's linker drops constant alignment](../issues/2026-09-30-zig-0-15-2-linker-drops-constant-alignment.md)
- [The CA bundle library test needs OpenSSL 3 on PATH](../issues/2026-10-01-ca-bundle-test-needs-openssl-3.md)
