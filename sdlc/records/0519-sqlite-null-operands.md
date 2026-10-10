# SQLite NULL operands

The SQLite slice builds on d16ad6c4a43ff1cf59ec99e750755031a2b820c6. The required-NULL rule comes from ticket 0519 and the SQL value contract in `specification/types.md`.

Native SQLite NULL operands short-circuit judgment scalars, annotate, plan, complete calls and keyed/table scans before parsing partners, resolving files or constructing engines. The shared table scan checks borrowed native values before copying arguments. Find checks its two required arguments and lets NULL optional settings use defaults. Backend failures, native admission, limits and connection answer slots keep their existing paths.

The installed extension tests replaced the incompatible rank and rank-set NULL oracles. The eight rank cases pass, including optional NULL settings, packed request counts, request content, joins, ties and ordinary refusals. Focused value cases pass for required NULL with malformed operands and settings across all ten complete calls, ordinary scalars, plan, recognize and relate; their loopback backends count zero sends. Scalar bad non-NULL types keep their usage errors. Find preserves answers and uses default behavior for NULL settings. The existing rank-set invalid-control, empty-input and NULL case passes. After changing the shared table scan to inspect borrowed values before copying arguments, the affected required-NULL scalar, complete and table cases pass again.

File tracing of the installed focused cases with `strace -f -e trace=openat,newfstatat,statx,readlink` observes zero operations naming the unread question-file sentinel. The trace checks the explicitly supplied file reference; early guards also precede settings, question admission and engine creation in the changed paths.

Offline jobs-two release builds use the existing SQLite target. The final copied extension is 11,235,176 bytes with SHA-256 `4e7a7af47b50fbc35dd0a171271446681b44d7df1400b4e6a164c02ac3eec5de`. SQLite library Clippy with warnings denied and repository policy pass before the final borrowed-value table adjustment; its offline release build and affected installed cases pass afterward. Focused Rust formatting, diff whitespace and the final SQLite source ratchet pass. The ratchet rises from 5143 to 5174 for native guards; reviewer acceptance remains required. Policy reports existing large-file warnings outside this change.

Binary images, DuckDB JSON, function descriptions and other database implementations are separate slices. No full parity, routine suite, load, release or paid-call check ran.

## Lessons

Checking NULL after parsing a different operand can produce an error or read a file before the database's NULL result. Test NULL beside malformed partner operands. Required and optional operands need separate guards: treating optional NULL settings as required made find return NULL instead of using defaults.

## Native SQLite complete image input

The native binary slice builds on fe328f824. `thinkthen_decide_complete` accepts the persistent BLOB produced by `thinkthen_images` as one image-only record. Its text descriptor form remains valid. The image module shares its existing tagged collection decoder and image-to-request conversion with scalar image calls. Shared `Inputs` carries a native `RequestItem` alongside its existing descriptor path, composes it with the question reading, and executes it through the same admitted Rust request. No JSON byte array or second semantic validator enters this path.

The installed consumer regression loads a copied release extension and checks complete value, ordinal and request facts, authored image order and duplicates, strict replay with a zero-send cap, malformed tags, required NULL beside malformed partners, and a backend failure without leaked response text. The existing scalar image cache witness also passes after the conversion helper is shared. The regression fails against the starting installed extension because BLOB admission produces a Usage envelope instead of a completed record.

Offline jobs-two release build and release-library Clippy with warnings denied pass under systemd limits of 8 GiB memory and 1 GiB swap. Rust formatting, diff whitespace and repository policy pass; policy reports existing size warnings outside this slice. The copied extension is `target/0519-images-installed/libthinkthen0.so`, 11,236,424 bytes, SHA-256 `19377ad9aa010f7870e7555d165b27be36147cf51be87b9088a095dbb43b73ef`. Offline release-library checks of the DuckDB bridge and PostgreSQL compile their unchanged callers against the shared input module. Their combined systemd memory peak is 278.6 MiB with zero swap. The lane measures 41,886,694,638 bytes after these checks, below the 40 GiB lane cap.

The SQLite Rust ceiling rises from 5174 to 5215 for the native carrier and adapter dispatch. The image conversion duplication is removed before accepting that growth. The Python ceiling rises from 4145 to 4205: the new outside-in test adds 37 nonblank lines, and the inherited baseline already exceeds its ceiling by 23. Fresh reviewer acceptance is required for these increases.

DuckDB binary bridge plumbing and PostgreSQL native composite-array complete calls require their own coherent host API slices. This change adds no SQL functions and changes no shared generator or template. No full parity, large-input, load, release, paid-call or remote-machine check runs for this slice.

The image carrier must retain native bytes instead of serializing a database BLOB into JSON byte numbers. Reusing the same conversion helper protects existing scalar behavior and keeps the host adapter thin. Replay metadata differs from an original response; the regression compares answer identity and evidence while checking the replay's zero sends separately.
