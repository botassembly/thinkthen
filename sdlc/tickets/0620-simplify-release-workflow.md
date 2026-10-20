# 0620: Make release.yml one script per target driven by the manifest

Status: COMPLETE.

Milestone: 0.2.1

Phase: 1

Depends on: 0613, 0614, 0615, 0619, 0622

Reviews: revision 5c3886a, accept

## Outcome

- Changes: `.github/workflows/release.yml`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-language-tools.py`, `sdlc/scripts/release-language-tools-self-test.py`, `sdlc/scripts/hosted-workflow-self-test.py`, `sdlc/scripts/workflows`, `sdlc/scripts/README.md`, `sdlc/tools.json`, `sdlc/scripts/tools.py`, `sdlc/scripts/linux-check`.
- Each release job runs one script entrypoint for its target and stage. The script reads `sdlc/tools.json` to decide what to install and check.
- `release.yml` holds triggers, the job graph, runners, permissions, time limits and passing build outputs between jobs, with no per-language `if:` steps.
- The per-target install steps in the YAML and the `release-workflow` subcommands they called are deleted, and only the per-stage entrypoint is kept.
- Each tool-install step in the hosted Linux jobs that the 0619 image covers is deleted. Those jobs run in the image.
- Every hosted tool version comes from the manifest. The pinned `maturin==1.15.0` and the separate uv install in `release-workflow` go.
- The `resolve` step admits only main and `rc/` tags for a rehearsal and `v*` tags for a release. Its routes for the retired `ticket/0543-final` and `release/0.2.0-candidate-*` refs are deleted.

### Proof

- On the M5: `pm job run --machine m5 0620 SHA --memory 8 -- sh -c 'python3 sdlc/scripts/hosted-workflow-self-test.py && python3 sdlc/scripts/release-language-tools-self-test.py && python3 sdlc/scripts/workflows'` passes. The self-test checks that every hosted tool resolves at its manifest version; that case fails on current main, where `release-workflow host-setup` installs maturin 1.15.0 and `tools.json` pins 1.9.4.
- On the M5: the macOS arm64 stage entrypoint runs through `pm job run --machine m5 0620 SHA` and passes.
- The Linux arm64 and x86_64 entrypoints run at the 0619 container checkpoint.
- The workflow tickets 0613, 0620 and 0622 are proven together by the one nonpublishing hosted rehearsal that 0691 dispatches at the next checkpoint: it publishes nothing, its archive list matches `sdlc/scripts/package-inventory.py`, and the installed smokes pass.

## Amendment, 2026-10-20 (core cut)

0619 closed on a core-only image, so this ticket follows the same cut.

- It may make minimal additions to `sdlc/tools.json`, `sdlc/scripts/tools.py` and `sdlc/scripts/linux-check` where a hosted stage needs a tool or stage those files do not yet declare. It adds no new runner and changes no public release eligibility.
- Hosted Linux jobs delete only the install steps the core image covers. Language setup the image does not yet cover, and the manylinux distribution build setup, stay until the language's own wave adds those tools to the manifest.
- The Linux arm64 and x86_64 stage entrypoints and the full Linux stage proof move to 0691's hosted rehearsal, instead of the 0619 checkpoint.

- Ruling, 2026-10-20: hosted stages that still run natively (language smokes, the manylinux build) keep their native prerequisites, pinned from the manifest, until each consumer's wave moves its stage into the image. This replaces the Outcome bullet that deletes every covered install step now. No preparation-only Docker call stands in for a native prerequisite, and no artifact transfer is added to `linux-check` in this ticket.

## Evidence

- Starts from: `.github/workflows/release.yml`, whose jobs call `sdlc/scripts/release-workflow` subcommands (`host-setup`, `managed-tools`, `go-cpp-tools`, `swift-zig-tools`, `php-dart-tools`, `ada-objc-cobol-tools`, `dart-tools` and the per-family gates) and hold steps behind `if: matrix.target == '...'` lines. `release-workflow` is about 1,100 lines, and each new tool or target adds branches to both files. `sdlc/scripts/release-language-tools.py` acquires the Linux x86 language tools on its own. `sdlc/scripts/hosted-workflow-self-test.py` and `sdlc/scripts/workflows` check the YAML.
- The hosted Linux jobs install their own tools, which the 0619 image covers.
- The free-text candidate input is gone: `release.yml` takes only `mode`, and `release-workflow resolve` reads the dispatched ref. It still admits refs from the retired 0.2.0 candidate flow.
- 0622 (running) changes `release-smoke`, `release-registry-self-test.py` and `hosted-workflow-self-test.py`; this ticket waits for it and batches with it.

## Amendment, 2026-10-18

The hosted acquisition in `sdlc/scripts/release-workflow` drifts from the declared tool manifest: it installs maturin 1.15.0 where `tools.json` pins 1.9.4, and uv 0.9.17 where it pins 0.8.22. This ticket deletes those acquisition steps and takes every hosted tool version from the manifest. Its proof shows the hosted jobs resolve each tool at the manifest's version.
