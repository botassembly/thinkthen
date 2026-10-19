# 0657: Make go get deliver the Go module's native library

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0619, 0641, 0688

## Outcome

- Changes: `libraries/go/thinkthen.go`, `libraries/go/link_*.go`, `libraries/go/fixtures/package_owned.py`, `libraries/go/check.sh`, `libraries/go/README.md`, `libraries/go/README-DEVELOPMENT.md`, `sdlc/scripts/release-pack`, `sdlc/tools.json`.
- A chosen and documented distribution: either a tagged module that carries per-platform static archives, or a small separate module per platform selected by build tags. It covers linux/amd64, linux/arm64 and darwin/arm64, plus windows/amd64 if the 0615 manifest lists it for Go. New per-platform link files sit beside `link_linux_amd64.go`.
- Go's `check_platforms` in `sdlc/tools.json` adds darwin-arm64 and linux-arm64, so the Go gate runs there.
- `release-pack` packs the Go module for every target the manifest lists, not only `x86_64-unknown-linux-gnu`.
- `go get` of the release version builds an example with no pkg-config and no manual paths on each supported platform.
- The pkg-config fallback in `thinkthen.go` is deleted, or kept only for platforms the design lists as source-only.
- The README's `go get` line and README-DEVELOPMENT.md describe the chosen distribution.

### Proof

- On the M5 through `pm job run --machine m5 0657 SHA --memory 8 -- sh -c 'sdlc/scripts/release-pack aarch64-apple-darwin OUT c go && THINKTHEN_ARTIFACT=OUT/thinkthen-go-VERSION-aarch64-apple-darwin.tar.gz THINKTHEN_C_ARTIFACT=OUT/thinkthen-c-VERSION-aarch64-apple-darwin.tar.gz sh libraries/go/check.sh'`, the installed-mode check serves the packed module from a `GOPROXY=file://` directory, fetches it into a clean module cache, and builds and runs the example with no pkg-config. Before the change `release-pack` refuses Go on this target.
- linux-arm64 runs at the 0619 container checkpoint on the M5, and linux-x86_64 in the 0619 x86_64 container on Yellow.
- windows/amd64, if supported, is proven on the candidate in 0687.

## Evidence

- The README tells callers to run `go get github.com/botassembly/thinkthen/libraries/go@v0.2.0`.
- `link_linux_amd64.go` links a static library from `native/x86_64-unknown-linux-gnu`, which is not in git. Every other platform falls back to `#cgo pkg-config: thinkthen` in `thinkthen.go`.
- `part_source_wrapper` in `sdlc/scripts/release-pack` refuses Go on any target but `x86_64-unknown-linux-gnu` and builds the module through `libraries/go/fixtures/package_owned.py`.
- `sdlc/tools.json` already lists darwin-arm64, darwin-x86_64, linux-arm64 and linux-x86_64 as Go platforms, but `check_platforms` lists only linux-x86_64, so `libraries/go/check.sh` refuses to run on the M5.
- README-DEVELOPMENT.md says other targets and registry install "await distribution qualification". A module fetched from the Go proxy therefore cannot link today.
- The package version in `go get` and the cgo package-version guard in `thinkthen.go` are release numbers, which the 2026-10-19 no-version ruling keeps.
