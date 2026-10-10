# 0530: Repair current Go and C++ source package inventories

The local archive self-test at starting revision `b2c652a6270912527c82f66fbb8626808f95d3ea` failed with `release-go-cpp-pair: Go source members differ`. The self-test invokes the real source archive builder and substitutes only native build outputs. The Go builder already copied every root Go source file, while its gate retained a stale handwritten inventory missing `link_linux_amd64.go`, `owned_results_generated.go`, `owned_values.go` and `session.go`.

`release-go-cpp-pair` now derives root Go source names from committed HEAD and retains the fixed document, example, directory and manifest expectations and exact archive member comparison. It reads committed source independently of the package under test. The existing altered source identity rejection keeps its original failure ordering and cause.

The same inspection found that `release-pack` omitted C++ `client.hpp`, `inputs_generated.hpp`, `result_value.hpp` and `results_generated.hpp`. A consumer of the current client could not compile from that archive. The builder now copies the public header directory's `.hpp` files, following the existing CMake directory installation. It keeps the archived source byte checks and native pairing checks.

## Checks

The existing `release-archive-self-test.py` passed using local synthetic native artifacts and fake transport tools. Its existing missing archive, checksum, extra family member, altered source identity, private native bytes, source mutation and copied source refusal cases remain unchanged. During that run, direct inspection of the actual C++ synthetic archive found all nine canonical public headers and compared each header's bytes with committed HEAD. This includes all four previously omitted headers.

Offline policy passed for 268 resolved packages. Existing size warnings concern unchanged files. Shell syntax and whitespace checks passed. The Rust source ceiling remains 183253/183253; the Python source ceiling remains 9052/9052. Nonblank shell source changed from 266 to 265 lines in `release-go-cpp-pair` and from 578 to 576 lines in `release-pack`. The Python archive self-test remains 751 lines. No ceiling changed.

This repair runs no real native build, full parity check, final distribution assembly, workflow dispatch, candidate, publication or paid call. Installed consumer qualification and final distribution assembly remain with their existing owning tickets and authorized release checks.

## What the build taught us

A fresh read-only reviewer accepted `73065ed38601fc3c3373a6d3cee7d85feb207841`. The reviewer checked all 31 root Go files and nine public C++ headers against committed source and confirmed that the existing source identity, zero-SHA and archived byte rejection checks remain intact. Integration changes no source ceiling.

The apparent synthetic fixture failure exposed two stale copies of package contents in real scripts. Deriving Go source names from committed source and C++ headers from the public directory removes those copies while preserving independent package identity and rejection checks.
