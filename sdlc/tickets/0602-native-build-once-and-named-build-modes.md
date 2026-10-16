# 0602 — native-build-once-and-named-build-modes

Status: OPEN.

Milestone: 0.3

Depends on: 0589

Draft: filed 2026-10-16 by the 0.2 PM from pm's build-efficiency mail. It starts after 0.2 publishes and after Ian's build-system evaluation, which may replace it.

## Outcome

Each check builds the native library once per revision, platform, compiler configuration and instrumentation mode, then installs copies of it into every language test that needs it. Routine, full and stress checks choose from a few named build modes, and every mode keeps the compiler cache and the fast linker.

## Evidence

- pm's build-efficiency review of 2026-10-16 found seven C-backed language test families that each make a fresh native build in their own Cargo output folder. Each one builds the shared and static libraries again. Full qualification also turns the compiler cache off.
- The separate output folders stop a real race, where one consumer copies the library while another process rebuilds it. The same isolation pays for the same build seven times.
- Java, Kotlin and Scala already prepare their family's assets once and test independent consumers against them.
- Routine, full and stress checks set their own optimization, debug, wrapper and linker flags. pm's review thought some overrides drop the fast linker; the audit below found otherwise.
- The 2026-10-16 read-only audit found the seven families in `native_install` (`sdlc/scripts/installed.sh`: c, cobol, cpp, csharp, dart, swift, zig) each make a cold build into fresh scratch. Ada, Go, JVM, C#, Swift and PHP build again in their own folders, under at least four different flag sets. A cold build takes 57 to 71 seconds for Ada and 37.6 seconds for PHP. Cost before: roughly 7 minutes of compiling per full family pass. Expected after: about 1 minute.
- The same audit found an empty `RUSTC_WRAPPER=` turns the compiler cache off in the Ada, C++, Go, C#, JVM, PHP and Swift families, `full-compiler`, `coverage`, DuckDB's `setup.sh` and `registry-readmes.py`. Every artifact it checked was linked by LLD, Rust's default, so the linker survives every mode today; the configured mold setting has no effect.
- Ian ruled during 0.2 that the release runners stay out of Git while he evaluates the build system, and that the build lock changes only by deletion. This ticket waits on that evaluation.

## Scope

- Changes: the scripts under `sdlc/scripts/` that build the native library for language tests, the language test families' build steps under `libraries/`, and the place that names build flags. The owner reads the current paths on main before work starts.
- Checklist:
  1. List every place a check builds the native library, with its flags and output folder.
  2. Name the build modes, such as routine, full, coverage and sanitizer, each with fixed flags that keep the fast linker.
  3. Build each mode once into a read-only folder, and install a copy into each consumer before it runs.
  4. Delete the per-family native builds and the flag overrides they replace.
- Keeps: every check's coverage; the separation that stops a consumer from copying a library mid-rebuild; release qualification on the Beelink at each candidate.

## Proof

One routine run and one full run each show a single native build per mode in pm's job record, with compiler cache hits, and every language test passes against the installed copy. After landing, measure again and record the result here.

## Defers

Moving routine builds to the M5 is ticket 0589.
