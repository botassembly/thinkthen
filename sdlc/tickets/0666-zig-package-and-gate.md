# 0666: Package Zig for every supported target and simplify its gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- Zig adds darwin-arm64 and linux-arm64 to its supported targets in the 0615 manifest. The package carries native archives for each supported target. build.zig selects one by target and links its system libraries per target.
- The ABI version check uses a generated Zig constant compared with `thinkthen_version()` at first call. The header-parsing code is deleted.
- The supported Zig version range is stated. Its lowest and newest versions are both in the tool manifest and the pinned image, and both are tested.
- Tests/request_fixture.zig and Tests/session_consumer.zig are replaced by one packet dumper following the 0642 contract.

### Proof

Through pm job: natively on the M5 (darwin-arm64), in the pinned arm64 container on the M5 (linux-arm64), and in the pinned x86_64 container on Yellow (linux-x86_64). On each, an outside Zig project builds and runs the example with `zig build`, and its dump matches the 0642 oracle.

## Evidence

- `linkNative` panics on anything but x86_64-linux-gnu, "in this rehearsal" (libraries/zig/build.zig:44-45).
- build.zig checks the ABI by parsing `#define` lines out of the C header by hand (:5-37) and by searching the header text for three symbol names (:51-53).
- The package requires Zig 0.15.2 (build.zig.zon, README.md:3). The gate exits 77 silently when zig, python3, node or cc is missing (check.sh:11-13); 0618 makes that a failure.
- The smoke links with a hand-written list of system libraries (check.sh:42-44).
