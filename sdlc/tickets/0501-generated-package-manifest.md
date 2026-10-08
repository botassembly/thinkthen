# 0501 — generated-package-manifest

Status: OPEN.

Milestone: 0.2

## Outcome

Use one packaging definition to generate npm and JAR product inventories for builders and installed checks.

## Evidence

- Starts from: PM architecture ask2; handwritten npm files and JVM assembly, existing complete import-graph/compiled-output checks.
- Keeps: Existing installed artifact checks; no redundant withdrawn0486 checker.
- Changes: Derive manifest from the settled generated file layout; builders and checks consume it. Depends on direct/family migrations. Claim package generation, `libraries/typescript/package.json`, JVM packaging and existing package checks.
- Proof: Built inventories agree with the generated definition and missing/stale product files fail installed consumers.
- Defers: New receipt frameworks or per-language proof reports. Size: medium packaging change.
