# 0380 slice A: Add the Windows command release target

Status: in progress. Built in physical lane claude-1 from origin/main after 0397, at e760432c80dc22501fa51694964f2eeec5b8e63e. Fresh ticket review accepted. Independent ZIP packing and documentation are built; the shared release workflow changes wait for 0398 slice A and the coordinator's signal. This record does not claim the fifth target is complete or runner-proven.

`release-pack` accepts `x86_64-pc-windows-msvc`, builds or reuses `thinkthen.exe`, and writes a ZIP containing that one executable and its SHA-256 sidecar. Windows accepts only the command, first-run sample and crate parts at this stage. Unix target names, command payloads, archive format and default parts retain their prior paths.

The Windows ZIP helper checks the executable's PE32+ x86-64 header. Its check mode refuses malformed ZIPs, wrong member inventories, linked executables, wrong target names and checksum mismatches without extracting a file. ZIP timestamps are fixed so the same binary produces the same bytes. The README and crate metadata name the supported Windows platform and existing configuration, cache and usage folders.

The focused self-test runs the real packer with a synthetic Windows host and Cargo output. It checks both release-build and reused-debug paths, exact output inventory, binary contents and checksums. It also plants malformed binaries and ZIPs, wrong architecture, a DLL, extra and duplicate members, a traversal name, a linked member and checksum failures. It passes offline without compiling a product or running a synthetic executable.

Signing remains Ian's public release choice. This work permits unsigned development output. It enrolls in no signing service, spends nothing, dispatches no workflow and publishes no release. The Windows installer and runtime findings remain later slices of 0380.

Lessons: the archived release self-test verifies the entire archived source against Git HEAD, so the candidate packer must be committed before that proof runs. Windows source positions and library release work do not belong in this packaging slice.
