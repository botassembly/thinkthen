# Settings reference adoption

This bounded 0467 slice starts from `0d1f76bf84c7787558e6fae00c60e8cd2e2e3d1f`. It repairs the existing settings inventory without changing a product API or certifying the remaining cross-surface documentation work.

`sdlc/scripts/settings` excludes build helpers from the runtime environment scan. The JVM session builder's JDK, Kotlin and Scala homes select build tools; they are not engine settings. The C constructor delegates to `EngineBuilder::from_settings_json`. Its canonical `Document::apply` applies the Rust builder setters, and its admitted fields generate `EngineSettings` in `specification/request.schema.json`. The inventory follows that existing call instead of requiring removed C-local setters. Python still applies timeout through its wrapped builder chain.

`specification/settings.md` links the shared reader and schema, distinguishes published 0.1.2 from development targets, and describes the check's actual coverage. Frozen C symbols retain their deprecated compatibility role under `libraries/BINDING-AUTHOR.md`.

## Evidence

The existing settings self-test passes all 15 cases. Missing flag, environment name, question key, configuration field, Python engine cell and SQL cell remain distinct failures. Source negatives remove Python's timeout setter, Rust's shared C timeout setter, and C's connection to the shared settings reader. Each reports its expected source failure.

The real settings page passes against the warm main command with 84 rows, 85 flags, 9 product environment names and 25 question-file keys. These checks use a cleared environment and require no build or provider call. Offline policy passes, with existing file-size warnings in unchanged files. The diff and private-name checks pass.

The script changes add 32 handwritten lines and remove 6. The page changes add 3 and remove 1. This record adds 19. No runtime code, dependency, source ceiling or package version changes. Full suites, installed-package validation, parity, load and release qualification remain outside this slice.

## Lesson

An inventory must follow shared ownership when a binding sheds local code. Build-time tool homes belong outside the runtime settings inventory. A source negative must still detect a disconnected caller and a removed shared setter.
