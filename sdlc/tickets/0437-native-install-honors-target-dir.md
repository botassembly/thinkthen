# 0437: Honor the configured native build output folder

Status: ready. Planning only; implementation follows accepted ticket review.

Milestone: 0.2

Owner: builder.
Ticket review: accepted 2026-10-06; blocking findings corrected.

## Outcome

native_install finds/copies the native artifact from the effective Cargo target directory, including relative and absolute configured paths.

## Evidence

- Starts from: Issue 2026-10-03-native-install-ignores-cargo-target-dir.md and current native_install script. PM message `2026-10-06-pm-0-2-is-not-done-every-sdk-consistent-and-the-sdk-ready-for-the-proxy.md`, asks 8.
- Keeps: Preserve existing bare calls and generic JSON compatibility doors, backend selection from 0377, six error kinds, cancellation, secrecy, count-only usage, and zero-send strict replay. Reuse the Rust engine, C boundary and native file reader; add no host cache, scheduler or second parser.
- Changes: Remove assumptions that all artifacts are in libraries/c/target. Resolve unset, absolute and relative CARGO_TARGET_DIR using Cargo’s actual invocation directory; quote paths with spaces.
- Proof: Use the existing installation fixture with default, relative, absolute and spaced target paths. Plant artifacts only in the effective directory and verify the copied library; missing artifacts fail clearly. Retain platform library names and existing checksum/install safety.
- Defers: Proxy service/screens, images, unrelated features and Windows Node/C#/JVM packaging remain outside this outcome.
