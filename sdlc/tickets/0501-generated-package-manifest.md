# 0501: Generate package manifests from the binding outputs

Status: OPEN.

Milestone: 0.2

Depends on: 0513

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

Use one packaging definition to generate npm and JAR product inventories for builders and installed checks.

## Evidence

- Starts from: PM architecture ask2; handwritten npm files and JVM assembly, existing complete import-graph/compiled-output checks.
- Keeps: Existing installed artifact checks; no redundant withdrawn0486 checker.
- Changes: Derive manifest from the settled generated file layout; builders and checks consume it. Depends on direct/family migrations. Claim package generation, `libraries/typescript/package.json`, JVM packaging and existing package checks.
- Proof: Built inventories agree with the generated definition and missing/stale product files fail installed consumers.
- Defers: New receipt frameworks or per-language proof reports. Size: medium packaging change.

## 2026-10-09 amendment

Link this outcome to 0517's native-library packaging. Use one generated product inventory for both builder and installed checks, not a second manifest system. Follow final generated result and family layouts. Review the amendment and name existing npm/JVM builder and consumer files before implementation.

## Surface assessment amendment

Own the common inventory used by final assembly and collection as well as npm/JAR builders. Extend the existing package definitions only as needed for 0517's native assets and targets; keep no second platform or native-file list. Claim its actual producer and consuming script paths with 0517 before their slices. Target generators belong to their host migrations, so inventory work follows the relevant accepted host slice rather than waiting for 0513 to install all languages. The installed consumer is an independent behavioral check: agreement between a builder and its generated inventory cannot prove that both included everything the caller needs.
