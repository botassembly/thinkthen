# 0619: Run every Linux check in one pinned container per architecture

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0615

## Outcome

- The existing `manylinux_2_28` base from `release-container`, one image per architecture referenced by digest, holds every Linux tool in the manifest. Building and testing share this one environment. The separate image list in `release-container` is deleted in favor of the one image reference.
- One entrypoint runs `lint`, `test`, `test-full-cases`, `package` and `test-stress` in that image with CPU and memory capped to the hosted Linux runner's size. Containers start with `docker run --rm --label pm.repo=thinkthen --label pm.ticket=NNNN`, and pm jobs that run them pass `--memory 16`.
- Linux arm64 runs in the container on the M5 through pm job. Linux x86_64 runs in the container on Yellow through pm job. Everyday Linux proofs use the M5 container. Proofs specific to x86_64 use the Yellow container.
- The hosted Linux jobs run the same image and install no Linux tools of their own. Deleting the hosted install steps is 0620's work.
- The pm.json tickets gate runs `lint` and `test` in the M5 container, plus a native macOS `test` for Apple-only cases.
- The routine nextest time limit is set from timings measured in the capped container, with the reason written beside the setting.
- `machines.json` records Yellow's container role for ThinkThen. The PM requests that change; this ticket does not edit `machines.json`.
- The repo instructions and `AGENTS.md` say that everyday builds and tests run on the M5 through pm job, that Linux runs in the pinned container on the M5 and on Yellow, and that hosted workflows run by hand only for releases. The "Linux CI runs on each push" claim is deleted.

### Proof

- On the M5 through pm job with `--memory 16`, the tickets gate passes: `lint` and `test` in the arm64 container and the native macOS `test`.
- On the M5 through pm job, the capped arm64 container runs `test-full-cases`, `package` and `test-stress`.
- On Yellow through pm job with `--memory 16`, the capped x86_64 container reproduces the failing cases of hosted run 39102240049 on main before 0621's fix, or records that they pass.
- A grep finds no `push:` or `pull_request:` trigger in `.github/workflows/`, and no instruction claims one.

## Evidence

Qualification has moved between machines: the Linux mini PC (load cap only there, `535653f6c`), the M5 (`c53055095`, `cb3473396`, both 2026-10-16) and the hosted runners. Each move brought its own run of fixes, such as the Mac path aliases in `fd8744a30` and the Mac shebang delay in `42cbde2e2`. Linux is now exercised only on hosted runners, so Linux-only failures appear late and cannot be reproduced locally. Release record 0543 (its final entry, 2026-10-18) says the C image groups pass on the M5 in 16.10 seconds at about 4.06 GB peak, and the Linux timeouts remain unexplained.

`sdlc/scripts/release-container:16-18` already pins a `manylinux_2_28` image by digest for each Linux architecture, but only release packing uses it. The M5's default pm job memory is 8 GiB, below the hosted runner's 16 GB.

The repo instructions are also wrong about Linux. `CLAUDE.md:12` and the matching line in `AGENTS.md` say "Linux CI runs on each push". Every workflow in `.github/workflows/` starts only by `workflow_dispatch` or `workflow_call`, which matches the ruling that GitHub Actions run only for public releases after local testing.

## Progress

- 2026-10-18 started
- 2026-10-19 next: Clean extraction dad2b4ac5 has source review with execution proof pending; existing PM answers2-4 in the0619-container-prerequisites mail authorize Ubuntu checks, distribution-only manylinux, recipe and manifest plus actual image IDs, and Yellow13.5GiB while M5 and hosted retain their manifest limits, so reconcile the older ticket wording before container qualification.

## Amendment, 2026-10-18

All Linux checks run in one pinned Ubuntu 24.04 image per architecture, which the hosted Linux jobs also use. The existing manylinux_2_28 step stays only to build distribution artifacts at the old glibc floor; the installed-artifact checks then run in the Ubuntu image. Yellow books 14 GiB through pm job with a 14 GiB container cap and one heavy job at a time, because it has 15 GiB of physical memory. The M5 container waits until its Docker runtime is running again; ThinkThen changes no host configuration.
