# 0649: Package Ada without fixed paths and replace its fixture consumers

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0688

## Outcome

- The gate uses the shared scratch helper for its build folder.
- Ada adds darwin-arm64 and linux-arm64 to its supported targets, beside linux-x86_64, in the 0615 manifest.
- thinkthen.gpr selects linker options by target through a scenario variable. The package carries the native archive for each supported target.
- The supported GNAT range is stated. Its lowest and newest versions are both in the tool manifest and the pinned image, and both are tested.
- One Ada packet dumper following the 0642 contract replaces full_case.adb, native_session.adb and their drivers, which are deleted.

### Proof

Through pm job: natively on the M5 (darwin-arm64), in the pinned arm64 container on the M5 (linux-arm64), and in the pinned x86_64 container on Yellow (linux-x86_64). An outside project builds against the unpacked package with gprbuild on each target, and its dump matches the 0642 oracle.

## Evidence

- libraries/ada/check.sh:18 builds into a hardcoded `/tmp/thinkthen-target-ada`, against the workspace rule for scratch space.
- It exits 77 off Linux x86-64 with no message (:5) and when gprbuild, gnatmake, gcc, nm or node is missing (:7-9). 0618 makes these failures.
- thinkthen.gpr links `-ldl -lpthread -lm` unconditionally (:9-10).
- The README supports only Linux x86-64 with GNAT 13.3.
- Conformance runs through checks/full_case.adb (226 lines), native_session.adb (236) and their Python drivers. Commit e08c6903e worked around compiler line limits for large fixture strings.
