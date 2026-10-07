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

Outcome: every SDK supports the same ten functions with the same admitted inputs, complete typed results, errors and cache/record/replay behavior, including admitted image decide/choose/score, the local MCP surface and additive named question/input declarations. The SDK uses one configured route; business policy belongs to the proxy. Ian’s 2026-10-06 direction supersedes the earlier core-only deferrals. [The current plan](team-0-2-2026-10-04.md) owns the retained seventeen asks and accepted 0456 intake, dependencies, lane ownership and acceptance.

Already landed: version/release safety, backends/provider setups/default-eight, rank search, audit/docs/recipes, Windows CLI/Rust/C/Python, backend selection, DuckDB 1.5.4/1.5.5, agent skill, native files and local runtimes. These are retained behavior. Raw JSON compatibility methods and older qualification runs do not establish the newly required typed SDK parity.

Owners below include landed prerequisites and remaining work; the current plan’s remaining-work table gives the execution order.

The reviewed 2026-10-07 intake adds [0457: source-size warnings](../tickets/0457-warn-on-large-handwritten-source.md), [0459: binding C-header agreement](../tickets/0459-enforce-binding-c-header-layouts.md), and [0458: bounded file-seam cleanup](../tickets/0458-audit-file-seams-after-core-freeze.md). Implement 0457 before cleanup. C-header agreement is required for 0.2. Run 0458 only after a named reviewed core-freeze commit; it is the first candidate for 0.2.1 deferral if Ian changes priorities.

- [0400 slice D: Reconcile provider setup measured rows](../tickets/0400-provider-setups-and-concurrency.md), using landed 0421 runtime evidence without new paid calls.
- [0296: Add the pandas Series accessor for all ten functions](../tickets/0296-pandas-series-accessor.md)
- [0300: ---](../tickets/0300-caller-priced-call-cost.md)
- [0393: npm publishes through staged publishing](../tickets/0393-npm-staged-publishing.md)
- [0406: Preserve rank criteria and score ordering on every surface](../tickets/0406-rank-question-equivalence.md)
- [0407: Supply separate per-record context everywhere](../tickets/0407-per-record-context.md)
- [0408: Expose complete probabilities and details everywhere](../tickets/0408-j1-full-probabilities.md)
- [0409: Match TypeScript rank and find types to runtime](../tickets/0409-typescript-rank-find-contract.md)
- [0410: Complete all ten dataframe functions and located files](../tickets/0410-column-function-equivalence.md)
- [0411: Reread stored answers under changed rules on every surface](../tickets/0411-binding-reading-replay-proof.md)
- [0412: Document and test R index conventions](../tickets/0412-r-result-contract.md)
- [0413: Supply per-record candidate options everywhere](../tickets/0413-options-per-record-equivalence.md)
- [0414: Separate context from aggregate evidence everywhere](../tickets/0414-separate-shared-context.md)
- [0417: Rank question sets on every SQL surface](../tickets/0417-sql-rank-question-sets.md)
- [0418: Rank question sets through typed language and frame APIs](../tickets/0418-binding-rank-question-sets.md)
- [0425: Put the complete SDK outcome into the 0.2 plan](../tickets/0425-sdk-consistency-0-2-plan.md)
- [0426: Expose typed C calls and complete result carriers](../tickets/0426-typed-c-ten-function-carriers.md)
- [0427: Complete Go, C# and JVM typed parity](../tickets/0427-go-csharp-jvm-typed-parity.md)
- [0428: Complete C++, Swift, Zig and Objective-C typed parity](../tickets/0428-cpp-swift-zig-objc-typed-parity.md)
- [0429: Complete PHP, Dart and Flutter typed parity](../tickets/0429-php-dart-flutter-typed-parity.md)
- [0430: Complete Ada and COBOL typed parity](../tickets/0430-ada-cobol-typed-parity.md)
- [0431: Complete typed located results in existing named SDKs](../tickets/0431-named-sdk-located-result-parity.md)
- [0432: Enforce shared behavior and generate the current parity table](../tickets/0432-canonical-binding-parity-matrix.md)
- [0433: Keep SQLite find’s selected model](../tickets/0433-sqlite-find-keeps-selected-model.md)
- [0434: Match SQL question inputs and descriptions](../tickets/0434-sql-question-and-option-parity.md)
- [0435: Return isolated SQL call facts and caller-priced cost](../tickets/0435-sql-call-facts-and-prices.md)
- [0436: Return rank position in detailed rank values](../tickets/0436-rank-details-return-rank-position.md)
- [0437: Honor the configured native build output folder](../tickets/0437-native-install-honors-target-dir.md)
- [0438: Replace Ruby’s unsupported-platform placeholder route](../tickets/0438-ruby-platform-fallback-diagnostic.md)
- [0439: Document Linux R installation](../tickets/0439-r-linux-install-guide.md)
- [0440: Move audit and diff under runs](../tickets/0440-runs-audit-diff-command-tree.md)
- [0441: Prepare the Decisions backend for preview access](../tickets/0441-openai-decisions-backend-preview.md)
- [0442: Settle SDK identity, provenance and cache compatibility](../tickets/0442-proxy-ready-sdk-contract.md)
- [0454: Store actual batch size and preserve per-question caching](../tickets/0454-group-questions-only-with-explicit-route-opt-in.md)
- [0443: Carry SDK call identity and cache instructions](../tickets/0443-sdk-call-identity-and-cache-policy.md)
- [0444: Version cache keys and preserve offline replay](../tickets/0444-versioned-cache-identity-and-replay.md)
- [0445: Complete attempt observations and command facts](../tickets/0445-complete-attempt-and-command-facts.md)
- [Every surface gives back run facts](../issues/2026-09-26-every-surface-should-give-back-run-facts.md), through the explicit owners in the plan.
- [0415: Signed DuckDB community listing](../tickets/0415-signed-duckdb-community-listing.md), submitted upstream; publication remains DuckDB’s decision.

- [0446: Add vision and the SDK boundary to the 0.2 plan](../tickets/0446-vision-and-sdk-boundary-0-2-plan.md)

- [0447: Execute typed image questions and read image files](../tickets/0447-typed-image-questions-and-file-items.md)

- [0448: Enforce documented image limits and refuse dropped images](../tickets/0448-image-route-limits-and-explicit-refusals.md)

- [0449: Keep each SDK engine on one configured route](../tickets/0449-one-route-sdk-boundary.md)

- [0450: Give each answer a stable identifier and reserve proxy policy fields](../tickets/0450-stable-answer-identifiers-and-proxy-reservations.md)

- [0451: Create production Guides and a reusable lesson template](../tickets/0451-production-guides-section-and-template.md)

- [0452: Carry typed image values through SQL](../tickets/0452-sql-image-values-and-queries.md)

- 0455: Local ten-function MCP server; accepted ticket and partial implementation remain on its parked branch, with native integration pending.

- [0456: Add named questions and declared item inputs](../tickets/0456-named-questions-and-input-declarations.md), shared design accepted after fresh High review; native/C/MCP/family implementation remains required. Existing0407/0414 are must-land 0.2 dependencies.

OpenAI Decisions text support (0441) is required in 0.2 under the later PM ruling; access, documentation and recorded probe replies are now available. DuckDB community acceptance/signing (0415) remains external and does not block 0.2; it can arrive in a point release when available. Vendor documentation starts image work now; experiment 0036 can adjust limits later and never blocks.

Support work: [0453: Extend authorized live token admission](../tickets/0453-extend-authorized-live-token-admission.md) preserves charges for separately approved runs; experiments remain outside the release gate.

SQLite find’s silent model override is first. Shared C/metadata/semantic contracts precede wide host edits; SQL, dataframe and host families proceed in noncolliding slices. 0432 generates current parity from the complete executed suite with no skips. 0441 is built from admitted documentation and recorded replies, with no further paid calls needed for implementation checks.

The 0456 amendment is a reviewed implementation plan, not release preparation. [0425 completion criteria](../tickets/0425-sdk-consistency-0-2-plan.md) require final executed parity including MCP, full Linux/hosted macOS gates on existing macos-15/15-intel runners, real Windows qualification, fresh docs-only image/MCP use, 0.1 file/cache compatibility and complete docs/changelog/release notes/known gaps. After Ian’s publication go, every public package installs/runs cleanly and selective 0035 runs on public 0.2. Account prerequisites remain pending owner confirmation, not live-verified; existing packaging owners handle them. M5 is bounded-check only and Yellow excluded. Final hosted rehearsal, real Windows qualification and QA run only after required implementation lands on the final reviewed commit; publishing still requires Ian’s go. Main is 0.2.0 and release/0.1 stays frozen. Public installation text stays 0.1.2 until 0.2 ships. No grep alias or new semantic function. 0447/0448/0452 add vision to 0.2; the plan records unsupported function/route refusals. 0449 keeps one endpoint/key/API type; 0450 reserves proxy overrides now; execution remains in 0.3 under an admitted proxy protocol. PostgreSQL evidence paths use the reviewed client-reader workaround in 0434. Windows Node/C#/JVM packaging remains 0.3; SDK parity on supported platforms is required now.

[0460](../tickets/0460-conservative-repeated-input-request-plan.md) passed review, full tests and lint at 607c3688b. Native and command-line plans conservatively bound initial sends while retaining runtime coalescing, caches and the C header. Byte/token previews describe uninterrupted packing. 0444 passed fresh review, all four required library campaigns and integrated full test/lint/spec at 829c4b2c4. Its landing establishes the reviewed core checkpoint; final installed parity remains under 0432.

## 0.3

Outcome: add the remaining Windows bindings after the 0.2 core release. These tickets stay open under Ian's 2026-10-05 ruling.

Future outcomes: support additional decision-provider endpoints and API versions, then other modalities when their contracts are known; admitted image questions are already required in 0.2. These are outcomes only.

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



- [0119: Functional audit of the engine tests](../tickets/0119-mutation-audit-of-the-engine-tests.md)
- [0295 — Polars namespace and SIGINT (F4)](../tickets/0295-polars-namespace-and-sigint.md)
- [The new-user stumble register](../issues/2026-09-20-new-user-stumble-register.md)
- [Annotate options from a file or a record](../issues/2026-09-23-annotate-options-from-a-file-or-a-record.md)
- [Rank by graded relevance: custom weights and rank fusion](../issues/2026-09-24-rank-by-graded-relevance-for-search-reranking.md)
- [Docs and how-tos owed](../issues/2026-09-25-docs-how-tos-and-spec-claims-owed.md)
- [Public library API gaps](../issues/2026-09-25-public-library-api-gaps.md)
- [recognize and relate: scale](../issues/2026-09-25-recognize-and-relate-scale-and-shape.md)
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
