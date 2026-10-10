# SQLite NULL operands

The SQLite slice builds on d16ad6c4a43ff1cf59ec99e750755031a2b820c6. The required-NULL rule comes from ticket 0519 and the SQL value contract in `specification/types.md`.

Native SQLite NULL operands short-circuit judgment scalars, annotate, plan, complete calls and keyed/table scans before parsing partners, resolving files or constructing engines. The shared table scan checks borrowed native values before copying arguments. Find checks its two required arguments and lets NULL optional settings use defaults. Backend failures, native admission, limits and connection answer slots keep their existing paths.

The installed extension tests replaced the incompatible rank and rank-set NULL oracles. The eight rank cases pass, including optional NULL settings, packed request counts, request content, joins, ties and ordinary refusals. Focused value cases pass for required NULL with malformed operands and settings across all ten complete calls, ordinary scalars, plan, recognize and relate; their loopback backends count zero sends. Scalar bad non-NULL types keep their usage errors. Find preserves answers and uses default behavior for NULL settings. The existing rank-set invalid-control, empty-input and NULL case passes. After changing the shared table scan to inspect borrowed values before copying arguments, the affected required-NULL scalar, complete and table cases pass again.

File tracing of the installed focused cases with `strace -f -e trace=openat,newfstatat,statx,readlink` observes zero operations naming the unread question-file sentinel. The trace checks the explicitly supplied file reference; early guards also precede settings, question admission and engine creation in the changed paths.

Offline jobs-two release builds use the existing SQLite target. The final copied extension is 11,235,176 bytes with SHA-256 `4e7a7af47b50fbc35dd0a171271446681b44d7df1400b4e6a164c02ac3eec5de`. SQLite library Clippy with warnings denied and repository policy pass before the final borrowed-value table adjustment; its offline release build and affected installed cases pass afterward. Focused Rust formatting, diff whitespace and the final SQLite source ratchet pass. The ratchet rises from 5143 to 5174 for native guards; reviewer acceptance remains required. Policy reports existing large-file warnings outside this change.

Binary images, DuckDB JSON, function descriptions and other database implementations are separate slices. No full parity, routine suite, load, release or paid-call check ran.

## Lessons

Checking NULL after parsing a different operand can produce an error or read a file before the database's NULL result. Test NULL beside malformed partner operands. Required and optional operands need separate guards: treating optional NULL settings as required made find return NULL instead of using defaults.
