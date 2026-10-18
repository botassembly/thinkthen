# 0664: Test TypeScript across its declared Node range without a pinned download

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- The gate takes Node versions from the 0615 tool manifest, which lists the lowest and newest supported Node majors. Both are tested.
- setup-toolchain.sh and the home-cache Node path are deleted. The TypeScript compiler comes from the locked devDependency.
- The release builds and packs every addon listed in native-platforms.json, and the gate checks the packed inventory.
- Conformance goes through a packet dumper following the 0642 contract. tests/native_case.ts and tests/native_case.mjs are replaced by it.

### Proof

On the M5 through pm job, natively and in the pinned arm64 container, the packed tarball installs and passes on each tested Node major. The packed file list contains every declared addon.

## Evidence

- package.json declares `"node": ">=22"`, but the gate refuses any Node other than exactly v22.22.3 (libraries/typescript/check.sh:62).
- The gate expects that Node under ~/.cache/thinkthen-toolchains, placed by setup-toolchain.sh with a linux-x64 archive name hardcoded (setup-toolchain.sh:9). Commit a6cfcabe4 added a pinned compiler download for the installed smoke.
- native-platforms.json lists five addon targets, but only the host's addon is built and tested (build-addon.sh:14-15).
- check.sh is 246 lines.
