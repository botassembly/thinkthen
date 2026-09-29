# Dart named-answer fence Quick Fix

Date: 2026-09-29. Branch: `ticket/qf-dart-named-answer-fences`. Starts from the [open lint issue](../issues/2026-09-29-named-answer-scan-rejects-dart-readme-fences.md) and main `96c40a61e`.

## Failure and correction

The prior integrated lint receipt reached the named-answer gate after its earlier checks, then rejected `libraries/dart/README.md:20` as unknown `yaml` and `:28` as unknown `dart`. The focused red rerun reproduced those two lines while the old self-test passed. The README's YAML is the three-line `dependencies / thinkthen_dart / path` pubspec source dependency, so the repo wrapper skips only that exact structure in that exact README. A YAML fence on another page, or one with another key, still fails as unknown. The existing unknown-YAML plant stays intact.

Executable Dart stays a checked code fence. The repo wrapper blanks Dart strings and comments while preserving line breaks, maps Dart call and `final` assignment spelling to the shared scanner's TypeScript spelling, and delegates direct-use and generic-name decisions to the unchanged site-owned module. Dart string interpolation or an unclosed literal/comment is refused because code could otherwise be hidden inside a string. The README's generic `result` variable became `decisionEnvelope`; it still prints the same `value` and `facts`. No product API, site scanner, or other language rule changed.

## Proof and limits

`node sdlc/scripts/named-answers.mjs --self-test` passed its retained five folder plants and fence plants, plus new Dart direct-use, typed direct-use, generic-name, meaningful-name, string, line-comment, block-comment and interpolation cases. The new YAML cases prove the one supported pubspec shape and refusal in another page or with an extra key. These tests matter because the old self-test verified only that an unknown Dart fence was rejected; it never reached the shared semantic rule. The self-test also scans the real Dart README and plants its old generic name back into the actual call; that mutation reaches the shared `generic` rule. The unchanged site module self-test passed all 57 cases and 7 sample rows. The actual repository scan passed: 240 checked code blocks in 101 pages and 64 tag-skipped blocks. No executable Dart was retagged as text.

`pages`, `tickets`, `git diff --check`, and the private-source check of this record passed. Full lint and product/package builds were not run; root owns the integrated checkpoint. Marketing's `site/scripts/named-answers.mjs` remains unchanged. The bridge intentionally refuses interpolated Dart examples rather than claiming to parse their executable expressions; extending that syntax or changing the shared language contract needs the site owner's scope.

## What the build taught us

Fence support and semantic checking are separate. Adding `dart` to a skip list would have made the README pass without enforcing the answer rule. The narrow bridge let the already accepted shared rule reject both a direct `Door.ask` use and a generic name. Configuration needed an exact data shape because treating every YAML fence as inert would weaken the existing unknown-language refusal. The existing self-test's passing result before this change did not prove the real README was scanned successfully, so the new self-test includes that page as well as negative Dart plants.
