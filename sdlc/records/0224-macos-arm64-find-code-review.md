# 0224 Apple Silicon find artifact review

Status: **ACCEPT** for proof-only candidate `37b57014`, independently reviewed with Sol High. Product source remains `c2db47e9`; the candidate adds only its proof record.

The reviewer inspected the exact pushed source and retained M5 output. Archive `7b986eb8accc6db3b545153717a5e94f327eda6e30731253f746c415481be11a` contains the same 54,511,686-byte extension as the installed and built files, SHA-256 `26f79addfae6045161cb498b1bb97a8479c804b3f44e78bbf1769d587ed3ada0`. It checked the bridge, backend, pinned DuckDB source and both genuine host CLIs against the build record. The extension is a single arm64 Mach-O with a relative install name, minos15.0, the exact 534-byte CPP/osx_arm64/ABI4 footer and no builder-home bytes. The official Rust inputs and standard-library floor match the record.

Without rebuilding, the reviewer independently ran installed package verification, stock typed-find NULL, all three selected find cases and captured conformance18/19. Current-host load, changed-footer refusal and genuine unchanged-artifact1.5.4 refusal passed. Full captured bodies matched the independent corpus, as did request hashes, actual served-URL digests and literal ordered results. It reproduced the old backend's zero-capture0/2 failure and the exact rebuilt backend's2/2 pass. This confirms a stale test input caused the first failure.

The host ran macOS26.4. Deployment metadata does not prove macOS15 execution, Intel support or broader release rehearsal. No source or package scripts changed. The reviewer relied on explicit remote pass/fail records because this environment's SSH wrapper did not propagate one deliberate remote nonzero exit; future remote checks must record and verify their actual remote exit status.
