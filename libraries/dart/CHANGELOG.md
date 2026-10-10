## Unreleased: 0.2.0

- Replace `Door`, `CompleteApi`, older judgments and handwritten native layouts with one asynchronous `Engine` API. The ten named methods return generated, owned results through the native session route. The main package import replaces the former session import.
- Preserve absent values, explicit null, unknown fields and exact `BigInt` counters in generated declarations. Typed failures retain completed rows and native terminal facts.
- Add explicit cancellation, bounded feed intake, owned session packets and native cleanup. Ordinary completion awaits stream cleanup and reports its failure. Usage persistence methods return owned status without changing successful answers.
- Bundle the native library through the Dart build hook. Dart 3.10 or later is required. The hook verifies pinned bytes; it compiles no Rust and downloads nothing at runtime. The Flutter integration supports Linux.

Version 0.2 is implemented on main and remains unreleased. Public installation remains 0.1.2. Routine installed Linux Dart and Flutter checks cover the current API. Final release asset assembly, full parity and platform qualification remain candidate checks. Public-package installation checks follow publication. The [package guide](README.md) gives replacement calls and the current development asset limits.

## 0.1.2

The 0.1.2 release fixes the macOS Ruby gems. The Dart package is unchanged. Take the C library from `thinkthen-c-0.1.2-TARGET.tar.gz` on the same GitHub release.

## 0.1.1

The 0.1.1 release republishes every registry package after the 0.1.0 publish steps failed for PyPI, npm and NuGet. The Dart package is unchanged. Take the C library from `thinkthen-c-0.1.1-TARGET.tar.gz` on the same GitHub release.

## 0.1.0

The first release on pub.dev. The package calls the ThinkThen C library through Dart FFI. Take the C library from `thinkthen-c-0.1.0-TARGET.tar.gz` on the same GitHub release.
