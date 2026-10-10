# Settings reference adoption

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

This bounded documentation repair starts from `d05def2bb44345c230e76f392f165f6d98d7fde1`. The described-kind recognition example uses `--plan` and shows its actual first question. `core/recognize/questions.rs` gives caller descriptions to every recognition step and uses entity wording for its boundary choices. The saved site answers describe the older name-only questions. Cache conversion preserves stored question identity; it cannot answer the changed wording. The example makes no new model claim. The existing bare-kind recognition example and all recordings remain unchanged.

The located relation output follows `cli/relate/source.rs::Occurrence`, whose serialized ordinal precedes its original record. Its values and physical positions remain unchanged. The answer-cache examples now show the current `PlanSummary` and `cli/facts.rs` request-size fields. Replay facts retain zero actual sends and zero actual sent-body maxima; planned bodies retain their estimated size.

The four affected examples pass against the current development command supplied through `THINKTHEN_BIN`. The complete local site build passes, including every CLI example and the retained bare-kind recognition replay. The build uses an owned scope capped at 4 GiB memory and 1 GiB swap; it rebuilds no Rust artifact. Logs are ticket-owned build output at `target/0467-affected-replays.log` and `target/0467-site-build-current.log`. Sample formatting, ticket records and count-only private-name checks pass. Public install instructions remain at 0.1.2. No provider call, host binding replay, release build, workflow or publication runs.

A saved answer can become incompatible when a caller description changes the generated question. Show the prepared question or retain a compatible recorded case instead of reusing a different question's reply. Compare native serialization and facts before updating example output.

The same current-contract repair moves `--image` from global flags to the shared flags used by commands whose help exposes it. `find` no longer advertises it. Recognition lists its current mode, literal stage contexts, examples, span proposal pointer and snippet width, and reuses the shared record-context flag. Defaults and ranges follow the existing settings table. The positional kind description reads that table's allowed rule instead of asking it for a removed numeric range.

Recipe metadata and its existing scope contract now point to the three owning issues under `sdlc/issues/closed/`. The retained artifact test creates each copied issue's parent directory. No issue record, publication disposition or public recipe claim changes. The catalog/help check, isolated recipe artifact checks and rendered recipe checks pass. The complete local build also verifies links, settings, Chromium redirects, cards, metadata and HTML/Markdown exports. Ticket and count-only privacy checks pass, and the lane remains under its storage cap.

Current help admission decides which commands share a flag. A setting can remain supported after its numeric bound disappears; readers must use the current allowed rule. Test fixtures should create destination directories from the source metadata they copy, so moving an owning issue preserves its validation.
