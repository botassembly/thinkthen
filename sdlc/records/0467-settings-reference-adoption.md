# Settings reference adoption

Fresh read-only review accepted the development upgrade documentation at `ae72e2568b5c99e6011dc49bc15fa4f7dc37f467`. It compared all six changed files with the owning guides and API sources, confirming migration mappings, MCP limits and the published/development distinction. JVM and Foundation implementation gaps remain explicit; no package qualification is claimed.

This bounded 0467 slice starts from `0d1f76bf84c7787558e6fae00c60e8cd2e2e3d1f`. It repairs the existing settings inventory without changing a product API or certifying the remaining cross-surface documentation work.

`sdlc/scripts/settings` excludes build helpers from the runtime environment scan. The JVM session builder's JDK, Kotlin and Scala homes select build tools; they are not engine settings. The C constructor delegates to `EngineBuilder::from_settings_json`. Its canonical `Document::apply` applies the Rust builder setters, and its admitted fields generate `EngineSettings` in `specification/request.schema.json`. The inventory follows that existing call instead of requiring removed C-local setters. Python still applies timeout through its wrapped builder chain.

`specification/settings.md` links the shared reader and schema, distinguishes published 0.1.2 from development targets, and describes the check's actual coverage. Frozen C symbols retain their deprecated compatibility role under `libraries/BINDING-AUTHOR.md`.

## Evidence

The existing settings self-test passes all 15 cases. Missing flag, environment name, question key, configuration field, Python engine cell and SQL cell remain distinct failures. Source negatives remove Python's timeout setter, Rust's shared C timeout setter, and C's connection to the shared settings reader. Each reports its expected source failure.

The real settings page passes against the warm main command with 84 rows, 85 flags, 9 product environment names and 25 question-file keys. These checks use a cleared environment and require no build or provider call. Offline policy passes, with existing file-size warnings in unchanged files. The diff and private-name checks pass.

The script changes add 32 handwritten lines and remove 6. The page changes add 3 and remove 1. This record adds 19. No runtime code, dependency, source ceiling or package version changes. Full suites, installed-package validation, parity, load and release qualification remain outside this slice.

## What the build taught us

An inventory must follow shared ownership when a binding sheds local code. Build-time tool homes belong outside the runtime settings inventory. A source negative must still detect a disconnected caller and a removed shared setter.

Fresh read-only review accepts `dc62d9d270aec8d1b337249a2b70f7fb405f779b`. The reviewer checked the C constructor and shared Rust reader, Python timeout chain and schema ownership, and reproduced all fifteen self-tests and the real page with the warm CLI in a cleared environment. The repair adds 54 and removes seven handwritten lines. No runtime source or build changes enter this slice.

## Current CLI examples

Fresh read-only review accepted `2882bbf527363ef96ab9d65e4a9646168ea87226`. The reviewer checked current contracts and the complete passing site build, reproduced the prepared recognition question, and found no blocking defect. This acceptance covers the bounded site corrections, not installed binding parity or release qualification.

This bounded documentation repair starts from `d05def2bb44345c230e76f392f165f6d98d7fde1`. The described-kind recognition example uses `--plan` and shows its actual first question. `core/recognize/questions.rs` gives caller descriptions to every recognition step and uses entity wording for its boundary choices. The saved site answers describe the older name-only questions. Cache conversion preserves stored question identity; it cannot answer the changed wording. The example makes no new model claim. The existing bare-kind recognition example and all recordings remain unchanged.

The located relation output follows `cli/relate/source.rs::Occurrence`, whose serialized ordinal precedes its original record. Its values and physical positions remain unchanged. The answer-cache examples now show the current `PlanSummary` and `cli/facts.rs` request-size fields. Replay facts retain zero actual sends and zero actual sent-body maxima; planned bodies retain their estimated size.

The four affected examples pass against the current development command supplied through `THINKTHEN_BIN`. The complete local site build passes, including every CLI example and the retained bare-kind recognition replay. The build uses an owned scope capped at 4 GiB memory and 1 GiB swap; it rebuilds no Rust artifact. Logs are ticket-owned build output at `target/0467-affected-replays.log` and `target/0467-site-build-current.log`. Sample formatting, ticket records and count-only private-name checks pass. Public install instructions remain at 0.1.2. No provider call, host binding replay, release build, workflow or publication runs.

A saved answer can become incompatible when a caller description changes the generated question. Show the prepared question or retain a compatible recorded case instead of reusing a different question's reply. Compare native serialization and facts before updating example output.

The same current-contract repair moves `--image` from global flags to the shared flags used by commands whose help exposes it. `find` no longer advertises it. Recognition lists its current mode, literal stage contexts, examples, span proposal pointer and snippet width, and reuses the shared record-context flag. Defaults and ranges follow the existing settings table. The positional kind description reads that table's allowed rule instead of asking it for a removed numeric range.

Recipe metadata and its existing scope contract now point to the three owning issues under `sdlc/issues/closed/`. The retained artifact test creates each copied issue's parent directory. No issue record, publication disposition or public recipe claim changes. The catalog/help check, isolated recipe artifact checks and rendered recipe checks pass. The complete local build also verifies links, settings, Chromium redirects, cards, metadata and HTML/Markdown exports. Ticket and count-only privacy checks pass, and the lane remains under its storage cap.

Current help admission decides which commands share a flag. A setting can remain supported after its numeric bound disappears; readers must use the current allowed rule. Test fixtures should create destination directories from the source metadata they copy, so moving an owning issue preserves its validation.

## Development API and upgrade links

This bounded 0467 slice starts from `7785693d7`. The root README, specification index and site installation pages link the binding guide and `libraries/UPGRADING-0.2.md`. The guide collects confirmed R and C# replacement mappings and links additive Go, Ruby, JVM and Dart APIs to their owning package documentation. It adds no surface inventory. The shared text distinguishes public 0.1.2 installation, the stable JDK 22 session and the approved Apple Foundation replacement. The current JVM default build still compiles the JDK 21 preview door; the current Objective-C sources still use GNU `Object`. Neither is described as a finished public API switch. MCP framing text follows `mcp/protocol.rs::MAX_MESSAGE` and the transport attachment admission in `mcp/request.rs`.

Both changed Astro pages parse with the installed compiler. The three changed Markdown documents parse with the site's existing parser; fences balance, 68 local or repository link targets exist, ticket validation reports zero failures, and the focused count-only privacy check finds no private names. `git diff --check` passes. One storage measurement totals 41,861,152,768 bytes across target, libraries and databases, below 40 GiB. No executable example changes, so no replay runs. No full site build, provider call, package build, parity run, release qualification or product source change enters this slice.

Package migration and qualification are different claims. A stable session implementation does not prove the default Maven route uses it, and an approved platform restriction does not prove an Apple facade exists. Shared documentation should link the owning API and preserve those distinctions instead of describing every intended package as installed.

## External contributor reporting

This bounded documentation change starts from `5a7bdcfbe7547079f89e353859efc9e273cb45a9`. `AGENTS.md` adds an outside-agent section and links `CONTRIBUTING.md#external-contributions`. The contribution guide directs external contributors to labelled GitHub issues and fork links instead of pull requests. The existing bug template supplies `bug`; the guide also names existing `documentation`, `enhancement` and `question` labels. Internal pm, ticket, coordinator and release rules retain their scope.

Read-only GitHub queries confirm those labels and report `has_discussions: false` with no discussion categories. The guide retains the approved discussion-board destination conditional on enablement and gives a question-labelled issue as the current fallback. No repository setting changes.

Focused checks verify the contribution anchor, local links, repository URL paths, template label, private-name absence and the 5,000-character limit. `AGENTS.md` measures 4,993 characters; `git diff --check` passes. No executable example changes, provider calls, tests, builds, full site checks, package qualification or publication enter this change.

A reporting flow must state who can apply an issue label and whether its question destination is available. External contributors can name a label for a maintainer when GitHub withholds label controls.

Fresh read-only review accepted `1181338a883bbc71082c315438e56d4fdf92c222`. It checked the approved flow, live labels and disabled Discussions, confirmed the question-issue fallback, and verified that internal pm and safety boundaries remain intact. The agent instructions remain within their character cap.

## Settled cross-surface guidance

This bounded documentation pass starts from `da460ea4ebcb10cffa0f2700fd0290fe5ec7a5bd`. The root README and site installation index now describe the implemented Foundation API while preserving its Apple installed-consumer requirement and protected GNU compatibility sources. The upgrade guide follows the ordinary stable JVM builder and retains the final native-classifier assembly and Maven-resolution limits. It adds the owning TypeScript, C++, PHP, Foundation and Python migration mappings without copying a surface inventory, runtime-floor table or engine limits.

The function pages explain how a reader chooses a filter cut from labeled cases and checks a separate sample. They and `specification/result.md` explain the even per-row attempt share, the earliest-row remainder and the run total from `--facts`. These statements follow `core::share`, the observation serializer and the command facts writer. `specification/recording.md` links the native usage operations and package-owned language methods instead of claiming every library silently drops persistence failures. Its observation text follows the live writer state, current-delta scope and usage-lock-only deadline; it preserves SQL-owned reporting behavior. `crates/thinkthen/src/cli/args.rs` adds only the requested `thinkthen cache convert DIR` hint to `--record` help.

Existing explicit source selection, image admission, shared settings ownership, MCP framing and released-install guidance remain correct and unchanged. Public installation stays at 0.1.2. External reporting remains unchanged, and `AGENTS.md` stays at 4,993 characters. Compatibility APIs stay protected until their installed parity requirements pass.

Focused evidence: the repository pages check resolves all 27 covered pages; ticket validation reports zero failures; both changed Astro pages parse with the installed compiler; all 76 local or repository link targets in changed documents exist, and Markdown fences balance. The result-contract anchor is `the-run-facts-line`. The focused privacy scan checks 35 external private names across tracked paths and file contents and finds no matches. Rustfmt checks the single help source successfully, and `git diff --check` passes. The help hint is source-checked; no new executable is built or claimed. No executable example or expected output changes, so no fixture replay is needed.

The reused lane artifacts occupy 39.90 GiB across target, libraries and databases before this small text change. No full site, routine suite, package, parity, candidate, workflow, paid-call, large-input or load run enters this pass. Existing installed-case evidence and package README limitations remain the authority; this text does not certify publication or final cross-platform qualification.

A shared guide must distinguish an implemented facade from its pending installed package proof. Link language-specific operations to their owner, and keep a live persistence observation separate from historical call facts.

Fresh read-only review accepted `219008c3d076a8c5f747c096c7b879b3a78c2f48` after checking all changed documents against the owning APIs. It explicitly accepted the one counted Rust help-comment line and the resulting ceiling of 187880. Integration reconciles that measured ceiling; no additional product behavior changes.
