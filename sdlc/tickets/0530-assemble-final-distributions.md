# 0530: Assemble final distributions from the package inventory

Status: OPEN.

Milestone: 0.2

Depends on: 0494
Depends on: 0495
Depends on: 0496
Depends on: 0497
Depends on: 0498
Depends on: 0499
Depends on: 0501
Depends on: 0504
Depends on: 0505
Depends on: 0516
Depends on: 0518
Depends on: 0519
Depends on: 0522
Depends on: 0523
Depends on: 0524
Depends on: 0525
Depends on: 0526
Depends on: 0527
Depends on: 0528
Depends on: 0529

Reviews: revision a087f6dc3, accept

Reviews: revision 36382970a, accept

Reviews: revision 651edfb82, accept

Reviews: revision fe45b8d3f00f1400e013b96e781590f5b06a0af3, accept

## Outcome

The release scripts assemble every final registry-format artifact from 0501's inventory. Each artifact installs and runs with its declared dependencies, without a checkout, warm loader state or a manual library path. An unsupported target refuses clearly.

## Evidence

- Starts from: [the ticket cleanup ruling](../decisions/2026-10-09-rewrite-binding-tickets-in-place.md) and finding 5 of [the surface assessment](../records/0521-surface-contract-assessment.md). `sdlc/scripts/release-registry.py` selects only the existing JVM jars. `sdlc/scripts/release-workflow` permits Objective-C artifacts only on Linux. Artifact collection in `.github/workflows/release.yml` has target-specific inputs.
- Keeps: The existing installed artifact checks. The release hold.
- Changes: Registry assembly, workflow validation and artifact collection read the native inventory from 0501. Objective-C routing follows the Apple-only ruling in 0518. The `npm-assemble` operation in `sdlc/scripts/release-workflow` packs every inventoried addon, including `win32-x64`. Each host migration adds its own package to the inventory, as 0501 set out. Assembly refuses when a shipped package is missing from the inventory. The npm and JVM package definitions gain their native file names, as [the native package design](../decisions/2026-10-09-native-package-design.md) sets them, and `sdlc/scripts/package-inventory.py` reads the names there and keeps no list of its own. Claim `sdlc/scripts/package-inventory.py`, `libraries/typescript/package.json`, `libraries/jvm/pom.xml`, `sdlc/scripts/release-registry.py`, `sdlc/scripts/release-workflow`, their existing self-tests and artifact collection in `.github/workflows/release.yml`. Name each affected installed-consumer route before editing.
- Proof: Install each final registry-format artifact with its declared dependencies in a clean environment and run a real call with no library path. A missing or wrong native asset and an unsupported target fail in the existing installed checks. A passing development archive does not count.
- Defers: Native platform qualification goes to 0383–0385 at the first authorized candidate. Local workflow edits and assembly tests authorize no dispatch, candidate or publication.

Author: project manager.
