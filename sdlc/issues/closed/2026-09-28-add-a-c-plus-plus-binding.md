# Add a C++ binding

Status: closed 2026-09-30. Fixed by ticket 0249: `libraries/cpp/` is integrated. The release channel stays in `2026-09-26-language-packages-need-a-release.md`, which absorbed `2026-09-28-cpp-consumer-proof-needs-a-supported-package.md`.

Filed 2026-09-28 by the marketing lead, on Ian's instruction: "I definitely want to support C++. I'm going to have the team work on that as well." Ian ranks C++ ahead of COBOL.

## Current status

Ticket 0249 landed the C++ source package at `libraries/cpp/` with an accepted public binding and four installed `find_package` consumers on the recorded Linux pin. See [0249 integration closure](../../records/0249-integration-closure.md) and [Go/C++ build proof](../../records/0249-go-cpp-build.md). Marketing can use the integrated binding as source evidence. This issue stays open for the original supported release channel: final-pin rebuild, actual `ubuntu-24.04` Actions build and release, checksummed native assets, direct CMake consumer installation and other-host proof. No release dispatch or publication is claimed.

## Original marketing intake

The deck and site list 24 bindings from one shared list: Ada, Bash, C, C#, COBOL, Dart, DuckDB, Go, Java, Kotlin, Objective-C, pandas, PHP, Polars, PostgreSQL, Python, R, Ruby, Rust, Scala, SQLite, Swift, TypeScript and Zig. C++ was not among them at intake. The earlier talking points named C++ as a C-header consumer before experiment 301 and ticket 0249 supplied proof.

## What marketing needs

The typed C++ layer and contract checks have landed. Marketing still owns the shared deck binding list and its count; release-channel claims wait for the open criteria above.
