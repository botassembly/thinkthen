# 0568: Give every language library the same surface and measure each one's cost

Status: OPEN.

Milestone: 0.2

Depends on: 0562

## Outcome

Every language library exposes the same functions, options, settings and helpers, checked against one manifest. Each library keeps the same layout and has the same stale-generation check. A release-suite report shows each library's handwritten, generated and test lines against the median, plus the handwritten lines each change adds per language. An agent adding a feature can then see which languages it must touch and when one costs far more than the rest.

## Evidence

- Starts from: Ian's 2026-10-12 question about whether the libraries are equivalent, and a read-only measurement of main at 78a11c47e.
  - All 16 host libraries expose the ten functions from `conformance/cases.json`.
  - Typed options differ. TypeScript, Ruby, PHP and Objective-C take options as untyped maps. They name 10 to 13 of the 22 request options, and 5 to 8 of the 19 engine settings. C++, Ada, Zig and Objective-C name 5 or 6 engine settings.
  - Helpers differ.
    - `plan` is missing in TypeScript, PHP, Dart, Objective-C, C++, Ada and MCP.
    - Question-file, name and reference helpers exist only in TypeScript, Ruby, PHP and Swift.
    - Item and file builders exist only in TypeScript, Ruby, PHP, Swift, R and Python.
    - MCP has no usage-persistence call.
    - Python adds about 600 lines of dataframe and pydantic accessors. R adds `tt_feed`, `tt_record`, `tt_batch`, `tt_reader_failure` and `tt_completion_read`. Java makes its JSON helpers public.
  - Dart and Python have no `generate.py --check` for stale generated files.
  - Test folders go by five names: `tests/`, `Tests/`, `checks/`, `fixtures/` and `tests/source`.
  - The JVM writes the same session and usage cases three times, in Java, Kotlin and Scala.
  - Size against the median of 490 handwritten lines:
    - C has 12,572 lines, about 11,000 of them the frozen 0.1 views and ABI.
    - Python has 2,672 lines and R has 2,140. R carries its own worker, receipt and diagnostics code, about 470 lines.
    - Python, R, Ruby and TypeScript each keep their own Rust conversion crate, of 1,000 to 2,700 lines.
  - The usage-status change cost 15 to 70 handwritten lines per language. Its test lines ran from 0 to 310 per language, with Ada, C and the JVM paying three to five times the median.
  - The ratchets cap each library's lines but differ in scope, and none compares one library with another.
- Keeps: each library's language idioms, and the C layer and generator ownership that 0563 sets.
- Changes:
  - Add one surface manifest under `conformance/`. It lists the functions, the request options, the engine settings and the helpers every library must expose, with the native name each library uses. Use the generator's inputs where they already list these names.
  - Add a lint check that fails when a library lacks a manifest entry or exposes a public name the manifest does not list.
  - Generate typed options and settings for every library from the schema. Delete the untyped option maps.
  - Settle each helper with a ruling recorded in Progress: every library gets it, or it is deleted everywhere. Python's dataframe accessors stay as one named exception, because they are that library's idiom.
  - Run `generate.py --check` for Dart and Python.
  - Use one test folder name per library, following each language's convention. Record the choice in `libraries/BINDING-AUTHOR.md`.
  - Write the JVM's shared cases once in Java. Kotlin and Scala check only what they add.
  - Remove the frozen 0.1 views from C, which 0565 also covers. Move R's worker and receipt code onto the shared session.
  - Add `sdlc/scripts/library-size`, which runs with the release suite. It reports each library's handwritten, generated and test lines, and each one's ratio to the median. It also reports the handwritten lines each commit since the last release added per library. It stores the result beside the coverage records.
  - Give each ratchet the same scope: handwritten production lines only. Lower each one to its measured value.
- Proof:
  - The manifest lint passes on every library, and fails on a fixture library missing one option.
  - The size report runs on the candidate. It shows no host library above three times the median, except the named Python exception.
  - Every language library's routine installed checks pass.
- Defers: one shared Rust conversion crate for Python, R, Ruby and TypeScript goes to idea 0571, which a later release decides.

PM amendment, 2026-10-12. The coordinator's scope recommendation stands. Every host library gets every helper: `plan`, the question-file, name and reference helpers, the item and file builders, and usage persistence. Each helper's logic lives in Rust, and each library adds only a thin generated or handwritten call. A helper that needs more than a thin call in any library is deleted everywhere instead, with the reason in Progress. MCP stays outside the helper manifest. It keeps exactly the ten tools from the README and the request capabilities they share. Java's transport JSON becomes private. Python keeps its dataframe and pydantic adapters as the one named idiom exception.

## Progress

PM amendment, 2026-10-13. C is the shared native layer that the C-session host libraries call, so the three-times-median proof does not apply to it. Stop cutting C at 1,982 handwritten lines and set its ratchet there. Keep a diagnostic, the raw JSON entry points or an export only while a library or the CLI calls it. Delete any that nothing calls, and make no further C cuts beyond that.
