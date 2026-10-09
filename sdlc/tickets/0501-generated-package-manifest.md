# 0501: Generate one package inventory

Status: OPEN.

Milestone: 0.2

Depends on: 0517

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

One generated product inventory names every file each package ships, including its native libraries. Builders and installed checks read that inventory, so no package keeps a second handwritten file or platform list.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 5 of [the surface assessment](../records/0521-surface-contract-assessment.md). The npm file list and JVM assembly are handwritten.
- Keeps: Existing installed artifact checks and the complete import-graph and compiled-output checks. The release hold.
- Changes: Extend the existing package definitions with the targets and native assets from 0517's design. Keep no second platform or native-file list. The first slice covers npm and the JVM jars; claim `libraries/typescript/package.json` and the JVM packaging files, naming producer and consuming scripts before editing. Each host migration's packaging slice then adds its package to the inventory instead of keeping its own list. This ticket is complete when the generator and the npm and JVM slice land.
- Proof: Built inventories agree with the generated definition, and a missing or stale product file fails the installed consumer. Agreement between a builder and the inventory cannot prove that both include what the caller needs, so the installed consumer stays the independent check.
- Defers: Final distribution assembly goes to 0530. Package design belongs to 0517. No new receipt framework or per-language report.
