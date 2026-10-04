# 0380 slice C design acceptance and risk

Status: accepted for implementation on 2026-10-04. Native Windows proof remains pending.

The coordinator supplied fresh independent High design ACCEPT against main `3c811971ad168b29bb0ba6abbc427d3c03240606`. The exact frozen design is retained in `0380-c-windows-design.md` with SHA-256 `b4fd9f157caf78fbe3d77dde541cfbf1a4d65f2ef2c29364d8a25c58b41c34ce`.

Risk is High. Native token and descriptor ownership, DACL secrecy, handle identity, the confined unsafe boundary and cancellation affect trust and cleanup. The accepted design confines native ownership to two FFI leaves, admits only the token user and LocalSystem for usage grants, keeps configuration warning and named-backend refusal distinct, creates private objects with protected descriptors, and retains the existing signal action ordering with safe conditional shutdown on Windows. Policy plants must enforce each boundary before fresh code review.

Linux checks and cached native binding inspection establish portable evidence only. Actual console events, native ACL and NTFS identity/creation proof remain pending an approved exact-commit runner dispatch. This implementation authorizes no dispatch, signing, publication, provider request, credential-file read or environment change. The coordinator owns fresh code review, checkpoints and landing. Ian can overturn the recorded choices.
