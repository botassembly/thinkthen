# Public source retains private path literals

Status: Open. Confirmed by the integrated lint checkpoint for ticket 0275 on 2026-09-29.

The configured private-name guard refuses 49 tracked locations. Most are machine-specific home paths in package privacy checks and historical evidence references; three are private repository references in issue records. The guard reports only file and line. The external name list and its contents must stay outside this repository.

## Outcome and proof

Remove the private literals from tracked source and records without weakening the scanner or package privacy checks. Replace machine-specific package rejection strings with appropriate generic home-path checks, preserving existing negative fixtures and installed-consumer receipts. Use portable workspace-relative evidence references in records and a generic owner description for the private repository. Preserve commit identities, measured results and original issue criteria.

Use the saved location list, targeted privacy/guard fixtures and the existing configured private-name scan as the smallest proof. Do not rebuild every language or start a package campaign. This source-hygiene correction changes no SQL/DataFrame behavior or validation. The held DuckDB child-environment finding remains separate, and passing this scan alone does not establish full lint green.

## Exact affected paths

- `libraries/ada/checks/privacy.py`
- `libraries/cobol/checks/privacy.py`
- `libraries/cpp/fixtures/guard.py`
- `libraries/csharp/tests/isolated_consumer.py`
- `libraries/csharp/tests/package_check.py`
- `libraries/csharp/tests/source/Installed.cs`
- `libraries/dart/checks/privacy.py`
- `libraries/go/fixtures/guard.py`
- `libraries/jvm/tests/consumer-run.py`
- `libraries/jvm/tests/installed.py`
- `libraries/jvm/tests/package_check.py`
- `libraries/objective-c/checks/privacy.py`
- `libraries/php/fixtures/installed.py`
- `libraries/swift/Tests/fixtures/guard.py`
- `libraries/swift/Tests/fixtures/isolated_consumer.py`
- `libraries/zig/Tests/guard.py`
- `libraries/zig/check.sh`
- `sdlc/issues/2026-09-20-new-user-stumble-register.md`
- `sdlc/issues/2026-09-29-readme-key-backend-and-overhead-lines.md`
- `sdlc/issues/closed/2026-09-27-marketing-audit-diff-wording.md`
- `sdlc/records/0249-csharp-jvm-build.md`
- `sdlc/records/0249-php-build.md`
- `sdlc/records/0249-swift-zig-build.md`
- `sdlc/records/0263-swift-zig-release-build.md`
- `sdlc/records/0263-swift-zig-release-preparation.md`
- `sdlc/records/0264-php-dart-release-preflight.md`
- `sdlc/records/0265-ada-objc-cobol-release-build.md`
- `sdlc/records/0266-installed-linux-flutter-build.md`
- `sdlc/records/0266-installed-linux-flutter-preparation.md`
- `sdlc/records/0267-release-bundle-preparation.md`
- `sdlc/records/0269-language-workflow-preparation.md`
- `sdlc/records/0269-php-dart-runner-prerequisites.md`
- `sdlc/records/0270-preserved-package-selector-check.md`
- `sdlc/records/2026-09-29-other-remainder-readiness.md`
- `sdlc/records/2026-09-29-recognition-release-decision.md`
- `sdlc/scripts/release-managed-pair.py`
- `sdlc/tickets/0264-php-dart-release-packages.md`
