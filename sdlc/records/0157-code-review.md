# Accept the staged 0157 source

Status: fresh independent High review accepted source `85180c3f` on 2026-09-28. Owner: Codex. This is staged source acceptance, not ticket or all-platform closure.

The reviewer returned **ACCEPT** for the entire 0157 diff. The reviewed scope includes the public and native constructors, retry-count output, SQL builder paths, C++/Rust `repr(C)` order and size-bearing engine keys. The reviewer accepted the focused proof and the measured 217-line root Rust ratchet growth over main `42c9be31`. No required source finding remained.

The selected nine-host shared settings case and strict merged-source checks in [the build record](0157-build.md) form the related settings-batch integration checkpoint. Host checks ran at candidate `a43a4fdb`; strict merged-source checks ran at `85180c3f`. A later text-only main merge at `43371b01` changed no runtime source or measured ratchet. The three retained DuckDB C API packages still lack the 0201 C++ migration and remain outside 0157's staged platform scope.
