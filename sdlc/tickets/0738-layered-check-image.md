# 0738: Build the Linux check image in per-tool layers and keep only the live image

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Reviews: revision f9581f3, accept

## Outcome

- Changes: `sdlc/scripts/hosted-preflight.py`, `sdlc/scripts/hosted-workflow-self-test.py`, `sdlc/scripts/tools-self-test.py`, `sdlc/tools.json`, `sdlc/tools/*.json`, `sdlc/linux/Dockerfile`, `sdlc/linux/install.py`, `sdlc/scripts/linux-check`, `sdlc/scripts/tools.py`, `sdlc/scripts/README.md`, `sdlc/records/0738-layered-check-image.md`.
- The Dockerfile installs each toolchain in its own layer, one `COPY` of `sdlc/tools/NAME.json` followed by one `RUN` of `install.py` for that tool, in a fixed order. Editing one tool's file rebuilds that layer and the ones after it, and the build reuses every earlier layer from the cache.
- Scripts that are not toolchains (`sdlc/scripts/tools.py`, `conformance/children/children.py`) leave the image and its tag hash. `linux-check` mounts them from the checkout at run time. The tag hash covers only the pinned base, `rust-toolchain.toml`, `sdlc/tools.json`, `sdlc/tools/*.json`, the Dockerfile and `install.py`.
- After a successful `prepare`, `linux-check` removes that architecture's older `thinkthen-check` images, keeping the new one and any image a running container uses.
- The pinned base digest, the per-build memory and CPU limits, the selected tool groups and missing-tool failures stay unchanged.

### Proof

- On the M5 through pm job: `sdlc/scripts/linux-check prepare` builds the arm64 image. Changing one tool's pin in a scratch commit (not landed) and preparing again rebuilds only that tool's layer and later ones; the build log shows "Using cache" for the earlier layers. Afterwards `docker images thinkthen-check` lists one arm64 image.
- Editing `sdlc/scripts/tools.py` alone leaves the image tag unchanged, and `linux-check lint` runs with the edited script.
- The configured tickets gate passes in the container.

## Evidence

Ian asked why Docker kept filling the M5 (Jarvis, 2026-10-20). Seven arm64 images built in 18 hours, 12 to 21 GiB each, about 107 GiB in all. `linux-check` tags the image with a hash of seven inputs, including `tools.py` and `children.py`, which changed 9 and 3 times in 30 hours. One `RUN` installs every toolchain under the classic builder, so every edit rebuilt the whole image from nothing, and the old image stayed.

This ticket goes first: every lane that touches a tool file pays a full image build on each edit.

## Progress

- 2026-10-20 started
- 2026-10-20 next: The first configured gate failed native bootstrap because fragment order installed Cargo-based providers before Rust; cd00f67975fb4650621e175b3eef47e83c081376 repairs declaration-derived ordering, with fresh native bootstrap, all 17 normal-wrapper resolver fixtures and policy passing in M5 receipts 1792514432255-68912, 1792514482107-85534 and 1792514973933-61891; fresh review and a configured gate remain.
