# C++ consumer proof needs a supported package

Status: the local package experiment (local experiment 301) is complete through both stages, parent-verified and review-accepted, and its pin already matches the current main contracts (packed bulk, result envelope — see `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`). The queue owner decides the integration ticket and release; nothing is published. Ian authorized C++ on 2026-09-28 ("make c++ work").

## What was proven

A `thinkthen-cpp` CMake package at pin `4a0a2d6e`: C++17 header-only RAII door + typed models over the C door (extern "C", no FFI), dependency-free MIT JSON parser (nine negatives; integer lexemes outside exact 53-bit range rejected before conversion), shared and static interface targets with PACKAGE_PREFIX_DIR config export proven under a multi-component libdir by an executed find_package consumer. Four installed consumers (multilib, shared, static, clang ASan/UBSan) pass the full strict contract with exact 16-arrival multisets and 18 planted negatives; thread-local error isolation and join-before-free proven on failure paths; corpus cases exactly-once including non-BMP case 41 both endpoint pairs; `error_facts_json` and `engine_new_with` bound and tested; 21-export ABI check header-derived.

## Reviews

Review 1: two real findings (CMake three-directory prefix assumption; silent 2^53 integer rounding) — both fixed with executed receipts. Review 2: ACCEPT, magnitude-accumulation overflow and static paths checked.

## Handoff

Copy only `stage2/package/` into `libraries/cpp/`. Rebuild at the final release pin (its pin predates `71f25087` but review confirmed its assertions already match the current contracts; verify at the merge pin anyway). No registry account needed — distribution is GitHub Releases + CMake config; vcpkg/Conan recipes optional add-ons. The from-source triple (clone → native build → find_package/use) is in the package README.

## Evidence

Local experiment 301: FINDINGS.md (both stages), stage2/FINDINGS.md, BUILD-REPORTs, reviews/, FIX-REPORTs, gate logs.

## Limits

One Linux host/toolchain pair, synthetic loopback, decimal/exponent doubles documented, Rust not sanitizer-instrumented, full J1 corpus at J8.
