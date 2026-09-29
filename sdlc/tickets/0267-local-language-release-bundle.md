---
flow: build
priority: 267
opens: sdlc/issues/2026-09-25-release-and-install-for-0-1.md sdlc/records/0249-release-preparation.md
---

# 0267: Require the installed language files in one local Linux release bundle

Status: design candidate at main `060feb92`; no implementation or new package proof. Build waits for reviewed 0265 Ada/GNU Objective-C/COBOL and 0266 private Flutter file routes to land. The [preparation](../records/0267-release-bundle-preparation.md) maps the accepted 0128 and 0261–0264 work and the two active lanes.

## Concrete gap and bounded outcome

The current `.github/workflows/release.yml` builds the original 0128 families only. `release-pack` makes the later binding archives only when explicitly named. `release-smoke` runs recognized binding files when present, but its final required-presence list includes only the original nine families. A local nine-family bundle can therefore pass with all eleven integrated source binding packages absent. The accepted 0261–0264 file pilots demonstrate separate, source-pinned C/wrapper installations, not one complete bundle from one commit. Ticket 0265 adds the last three binding files; ticket 0266 adds a private installed Flutter app file. This ticket makes their omission visible and runs one complete local Linux x86-64 installed-bundle check. It does not claim four-platform release or publication.

Add one explicit, private `release-smoke --source-packages DIR` selection. Before `backend_start`, require the original nine families, one C archive, each of the eleven binding archives and adjacent sidecars, and the private Flutter file once 0266 lands. The eleven are Go, C++, C#, JVM, Swift, Zig, PHP, Dart, Ada, GNU Objective-C and COBOL. JVM is one file for Java/Kotlin/Scala. Reject missing, duplicate, wrong-target or linked selected files with a named failure. Then reuse the accepted pair-mode preflights and installed selector loop, with 77 still failing. A direct default `release-smoke DIR` keeps 0128's original nine-family presence rule; the new selection does not alter the four-target release workflow by implication. The private Flutter file proves a local app installation, not a public twelfth binding or a pub.dev package.

## Retained behavior and file boundary

Keep 0128's dispatch-only workflow, resolved SHA, Linux container floor, four runner targets, original nine required families, checksum scan, first-run case, strict exit-77 rule and gated outward jobs. Keep the source wrappers and exact installed selectors from accepted 0261–0264 and the final reviewed 0265/0266 versions. Keep the shared C/wrapper manifests as fixed data and invoke their existing preflight modes before a backend. The current accepted wrapper checks that explicitly allow only Linux x86-64 must not become mandatory on macOS or ARM jobs. No source port, runtime API, registry dependency or new package format belongs here.

The expected implementation claim after fresh design review is the smallest change to `sdlc/scripts/release-smoke` for the opt-in presence selection, its existing focused checker/fixture if one is needed, this ticket and one build record. Use `sdlc/scripts/release-pack` and `release-go-cpp-pair` as read-only dependencies after 0265/0266 integration; edit either only for a specific review-found incompatibility and a new exact claim. `.github/workflows/release.yml`, `release-workflow`, `release-container`, `libraries/*` product/check code, `site/` and registry files are outside this ticket. The packer guard accepted in 0264 must continue to reject an old C archive or sidecar and a repeated `c` before output changes.

## One-pin proof

First refresh the file inventory against the accepted 0265 and 0266 landings. Use one clean committed Linux x86-64 source pin and a fresh empty OUT for an explicit pack of the original nine families, eleven wrappers and private Flutter file. Keep one C part before every wrapper and Dart before Flutter; never merge the earlier 0261–0264 archives, whose C hashes and source commits differ. Capture the source commit, actual Rust/language tool versions, host target, exact archive basenames, outer SHA-256 values, C header/shared/static member hashes, manifest fields, output path, selected native loads and all exit codes. This is a local host integration; it does not replace 0128's pinned manylinux build or runner proof.

Run `release-smoke --source-packages OUT` once against those installed files. Its existing checks must load each wrapper and C from their unpacked paths, run the retained public callers and count their exact fixture requests; the private Flutter case uses 0266's one real app call. One integrated smoke exercises every file once. Reuse the accepted exact body and no-send receipts when product inputs remain byte-identical instead of repeating each language's source/J1/matrix gate. If a tool is absent, record its named 77 and stop; do not install a new toolchain or silently drop the family. Compare actual bundle files with the recorded source pin, not a later notes-only HEAD.

For the new selection, plant a missing whole binding archive and sidecar in an otherwise valid copied bundle, then plant an extra selected-family archive or linked file. Each must fail before `backend_start` with the precise family named and leave the copy untouched. Reuse one accepted pair preflight mutation for changed checksum or mismatched C identity; do not copy its validator. A small checker can use fake sentinel selectors for the early omission branch, but the passing case must run the real installed files. Report zero sends only if a listener measured them; pre-start refusal alone proves no consumer began. Validate shell syntax, focused script policy, ticket/page/diff checks and any affected derived counter. No provider call, broad source matrix, stress run, workflow dispatch or publication belongs to this proof.

## What remains after this batch

The local bundle selection does not make the eleven files appear in `.github/workflows/release.yml`; that workflow still needs a separately reviewed per-target support table and pinned host setup. At least Go, C++, PHP, Ada, GNU Objective-C and COBOL selectors currently refuse non-Linux-x86-64 hosts. A later workflow batch must decide which archives build on which runner and require only supported targets, while preserving 0128's four original target claims and Ian's manual dispatch/approval controls. Actual GitHub Actions runs, final tag/rebuild, checksummed public assets, direct/registry installation, private Flutter publication-safe dependencies and other-platform claims remain in 0128 and the original language issues. One local bundle does not close them.

## Evidence

- Starts from: [0128](0128-release-and-install.md) Phase 2/3 contract and current release scripts, accepted [0261](../records/0261-go-cpp-release-code-review.md), [0262](../records/0262-csharp-jvm-release-code-review.md), [0263](../records/0263-swift-zig-release-code-review.md), [0264](../records/0264-php-dart-release-code-review.md), plus the active 0265/0266 accepted designs at their recorded branch pins.
- Keeps: Original full-smoke defaults, one clean source/native identity, existing pair validators and installed selectors, platform limits, source-gate evidence, and manual outward release controls.
- Changes: An explicit Linux x86-64 bundle presence rule and one actual same-pin installed integration run after the remaining pilots land.
- Proof: Missing/extra/linked family refusals before backend, strict pair checks, one complete local installed smoke with exact outputs and archive/source receipts, and focused script/policy/record checks.
- Defers: Four-runner workflow wiring and execution, final release pin, registry/account steps, public distribution, other platforms, and closure of original release issues.
