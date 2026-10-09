# 0501: Generate one package inventory and assemble final distributions from it

Status: OPEN.

Milestone: 0.2

Depends on: 0517

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

## Outcome

One generated product inventory names every file each package ships, including its native libraries. Builders, artifact collection, final assembly and installed checks all read that inventory. Each final registry-format artifact installs and runs without a checkout, warm loader state or a manual library path. An unsupported target refuses clearly.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and finding 5 of [the surface assessment](../records/0521-surface-contract-assessment.md). The npm file list and JVM assembly are handwritten. `sdlc/scripts/release-registry.py` selects only the existing JVM jars. `sdlc/scripts/release-workflow` permits Objective-C artifacts only on Linux. Artifact collection in `.github/workflows/release.yml` has target-specific inputs.
- Keeps: Existing installed artifact checks and the complete import-graph and compiled-output checks. The release hold.
- Changes: Extend the existing package definitions with the targets and native assets from 0517's design. Keep no second platform or native-file list. Slices, in order:
  - Inventory slices follow each accepted host packaging slice. Each names its producer and consuming scripts before editing. Claim `libraries/typescript/package.json` and the JVM packaging files for the first slices.
  - The final assembly slice follows all host packaging slices. Claim `sdlc/scripts/release-registry.py`, `sdlc/scripts/release-workflow`, their self-tests and artifact collection in `.github/workflows/release.yml`. Registry assembly follows the new native inventory. Objective-C routing follows the Apple-only ruling in 0518.
- Proof: Install each final registry-format artifact with its declared dependencies in a clean environment and run a real call with no library path. A missing or wrong native asset and an unsupported target fail in the existing installed checks. Agreement between a builder and the inventory cannot prove that both include what the caller needs, so the installed consumer stays the independent check.
- Defers: Native platform qualification goes to 0383–0385 at the first authorized candidate. Local workflow edits and assembly tests authorize no dispatch, candidate or publication. Package design belongs to 0517. No new receipt framework or per-language report.
