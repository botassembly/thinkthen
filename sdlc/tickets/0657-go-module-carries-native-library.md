# 0657: Make go get deliver the Go module's native library

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0619, 0641, 0688

## Outcome

- A chosen and documented distribution: either a tagged module that carries per-platform static archives, or a small separate module per platform selected by build tags. It covers linux/amd64, linux/arm64, darwin/arm64 and windows/amd64, or the supported list in the 0615 manifest. darwin-arm64 and linux-arm64 are added to Go's supported targets.
- `go get` of the release version builds an example with no pkg-config and no manual paths on each supported platform.
- The pkg-config fallback is deleted, or kept only for platforms the design lists as source-only.

### Proof

- Through pm job: natively on the M5 (darwin-arm64), in the pinned arm64 container on the M5 (linux-arm64), and in the pinned x86_64 container on Yellow (linux-x86_64). On each, a clean module cache fetches the release-candidate module from a local proxy and builds and runs the example.
- windows/amd64 is proven on the candidate in 0687.

## Evidence

- The README tells callers to run `go get github.com/botassembly/thinkthen/libraries/go@v0.2.0` (libraries/go/README.md:5).
- The module links a static library from `native/x86_64-unknown-linux-gnu`, which is not in git (link_linux_amd64.go:6-7; `git ls-files libraries/go`). Every other platform falls back to pkg-config (thinkthen.go:7-8).
- README-DEVELOPMENT.md says other targets and registry install "await distribution qualification". A module fetched from the Go proxy therefore cannot link today. It is unclear whether release tagging adds the archive.
