# 0483: Correct custom recognition wording and Unicode prefixes

Reviewed source `872b8426330352368f7aca940cb1fa211f4e2f63` starts from main `57be5f26ec8dc782dc0262246c00928adbb81d9d`. Recognition now selects explicit grammatical built-in entity descriptions and excludes only the specified initial BOM/zero-width prefix. Public command tests pin each prefix, a mixed prefix before a decomposed name, later scalar coordinates, retained input, internal joiners and another leading format character. Caller wording and default vectors retain their behavior. Owned custom replay questions and keys were reconstructed with retained controlled probabilities; the PHP body oracle retains its 39 requests. Ceiling commit `a599f64650486d7384bd54ecdfa5040a265fc76e` measures 96 added nonblank Rust lines. The public Unicode table owns the new behavior; existing combining-mark parser cases retain distinct claims.

## Checks and failures

`CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py`, `cargo test --locked --offline -p thinkthen --test backend recognize:: -- --test-threads=2`, the corrected `recognize::unicode` selection, `node sdlc/scripts/ratchet.mjs`, `sh libraries/php/check.sh`, `sh sdlc/scripts/spec` and the final `sh sdlc/scripts/lint` passed. Heavy checks used `systemd-run --user --scope -p MemoryMax=10G -p MemorySwapMax=1G`, offline Cargo, two build jobs and the lane lock at `target/0483/heavy.lock` where required. Logs are under `target/0483/`.

The original code failed both the grammatical-description assertion and the BOM public span regression. An added JSONL retained-input assertion initially supplied unframed text and correctly refused; encoding its input as a JSON string corrected the test. Initial lint failed the required measured ceiling; its correction passed lint without source changes. Existing policy size warnings concern unchanged files.

`sh sdlc/scripts/test` passed workspace tests, doctests, library-only tests, the external consumer, parity, schemas and transforms, then exited 5 at `demos/16-triage-pipeline/self-test` (`sdlc/scripts/test:88`). The rung inherits the actual configuration because it calls `usage_guard` without `config_home`; a pre-existing built-in openrouter configuration entry with custom routing caused refusal. The stopped demo and remaining self-test/smoke checks passed under an owned empty configuration with `sh -c '. sdlc/scripts/scratch.sh; usage_home; config_home; demos/16-triage-pipeline/self-test; sdlc/live-test; sh sdlc/scripts/smoke'`. Each self-test printed its final success statement and the remainder exited 0. The coordinator owns the separate test-runner gap. The Rust suite was not repeated.

## What the build taught us

Replacing a noun cannot supply the correct article. Built-in descriptions need authored sentences, while caller declarations retain their text. Skipping a narrow initial prefix during splitting preserves scalar offsets without rewriting input or changing internal formatting. A corrected custom request needs a reconstructed replay key. Replay and fixture boundaries should inherit an owned empty configuration.

General tokenization, model accuracy and overlapping-span policy remain outside this ticket. Paid calls, releases and platform qualification were not run.
