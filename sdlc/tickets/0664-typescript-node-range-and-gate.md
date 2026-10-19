# 0664: Test TypeScript across its declared Node range without a pinned download

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688, 0728

## Outcome

- Changes: `libraries/typescript/check.sh`, `libraries/typescript/setup-toolchain.sh`, `libraries/typescript/node.sha256`, `libraries/typescript/README-DEVELOPMENT.md`, `libraries/typescript/tests/`, `sdlc/tools/typescript.json`, `sdlc/scripts/tools.py`, `sdlc/scripts/hosted-preflight.py`, `sdlc/scripts/hosted-workflow-self-test.py`, `sdlc/scripts/coverage_installed_extensions.py`.
- The gate takes Node versions from the tool manifest `sdlc/tools.json`, which lists the lowest and newest supported Node majors. Both are tested. The `"node": ">=22"` floor in `package.json` stays; it is a minimum host version, not a format version.
- `setup-toolchain.sh`, `node.sha256` and the home-cache Node path are deleted, with the readers of `node.sha256` in `hosted-preflight.py`, `hosted-workflow-self-test.py` and `coverage_installed_extensions.py`. The TypeScript compiler comes from the locked devDependency.
- Conformance goes through a packet dumper over the public API that follows the 0642 contract, for the JavaScript and compiled TypeScript consumers. `tests/native_case.ts`, `tests/native_case.mjs` and `tests/native_case.d.mts` are replaced by it.
- Release packing stays as it is: main already packs every addon in `native-platforms.json` into one tarball and checks the inventory.

### Proof

- On the M5 natively: `pm job run --machine m5 0664 SHA --memory 8 -- env THINKTHEN_TEST_PROFILE=full sh libraries/typescript/check.sh` packs the tarball, installs it into a fresh project, and passes on each Node major the manifest lists, with the dump matching the 0642 oracle for every shared case. Before the change the check refuses any Node other than v22.22.3, so the newest-major run fails.
- `git grep -n 'thinkthen-toolchains\|node.sha256\|22\.22\.3' libraries/typescript sdlc/scripts` finds nothing.
- The pinned Linux arm64 and x86_64 runs happen at the 0619 container checkpoint. Windows and Intel macOS are proven on the candidate in 0687.

## Evidence

- `package.json` declares `"node": ">=22"`, but `libraries/typescript/check.sh` refuses any Node other than exactly v22.22.3, both in installed mode and at its "toolchain" step. `sdlc/tools.json` pins the `node` tool to 22.22.3 with one version.
- The gate's "not run" messages point at `setup-toolchain.sh`, which places Node under `~/.cache/thinkthen-toolchains` with a linux-x64 archive name hardcoded. `sdlc/scripts/hosted-preflight.py` reads `node.sha256`, `hosted-workflow-self-test.py` plants it, and `coverage_installed_extensions.py` names the `node-v22.22.3-linux-x64` folder. Commit a6cfcabe4 added a pinned compiler download for the installed smoke.
- Installed mode runs `tests/native_case.mjs` and the compiled `tests/native_case.ts` through `conformance/children/native_fixture.py`.
- The release already packs every addon: each target's `release-pack ... typescript` builds its addon, and `release-workflow npm-assemble` gathers the five addons and checks them with `package-inventory.py npm --check`.
- check.sh is 242 lines.
- TypeScript's dumper lands here; 0665 then proves its run-call switch against the same dump.
