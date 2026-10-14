# 0562: Check, trim and rename generated code before 0.2 freezes it

Status: OPEN.

Milestone: 0.2

Depends on: 0561, 0565

Reviews: revision dcafe90, accept

Reviews: revision 4d8d1a8, accept

Reviews: revision c6792fe, accept

Reviews: revision ffd077a, reject

Reviews: revision d7df691, accept

## Outcome

Lint checks every generated file against its generator. Each language carries only the generated declarations it uses. Generated public names read as real names, not schema paths. The C view layout shares one presence struct per value type. These changes land before the candidate, because the names and the `_v1` layouts become public API at 0.2.

## Evidence

- Starts from: the [2026-10-12 ruling](../decisions/2026-10-12-solid-0-2.md) and a read-only review of the 0.2 combined branch.
  - Every output regenerates byte-for-byte except the Swift Linux bridge. `libraries/swift/Sources/CThinkThen/bridge.c` lacks `thinkthen_session_new_named_with_surface`.
  - Lint checks only the C header and the C views. The Rust schema test checks only the C# outputs. Dart and the Python generated files have no drift check.
  - Unused generated declarations:
    - PHP's `ffi_generated.h` copies the whole C header, about 3,950 lines, and PHP calls 16 of its functions.
    - Swift's `bridge.c` forwards all exports, and Swift calls about 25.
    - Ada's `thinkthen_session_c.ads`, about 5,550 lines, has about 1,600 lines of frozen 0.1 API, and Ada uses about 56 declarations.
    - Dart's `abi_generated.dart` emits 13 unused constants.
  - Three untitled inline schema arms produce names derived from schema paths in every language. The longest Ada identifier is 157 characters. C# inputs skip the shared preparation step, so they name arms differently from the other languages.
  - 136 one-element `type` lists in the result schema produce about 110 one-case Swift enums and `Union[int]` in the Python stubs.
  - The header defines 249 separate presence structs over 73 value types.
  - Three generated files have no generated marker: `ffi_generated.h`, `graph_generated.json` and `thinkthen-sessions-calls.adb`. `policy.py` lists some hand-written files as generated and leaves some generated files out.
  - Most targets write whole bodies on one line. Some lines run past 3,000 characters.
- Keeps: the wire JSON, the request and result schemas' meaning, every language's routine installed checks, and the C session interface's behavior.
- Changes: these generator and schema changes, in this order.
  - Add `generate.py --all --check` to lint for every target the Beelink can build. Fix the Swift bridge. Add the missing generated markers. Mark generated paths with `linguist-generated` in `.gitattributes`. Make the `policy.py` generated list match the generators.
  - Emit only the declarations each language uses, from one list per language: the PHP FFI header, the Swift bridge and the Ada C copy.
  - Normalize one-element `type` lists in the shared preparation step.
  - Give the three inline arms schema titles, and route C# inputs through the shared preparation step.
  - Share one presence struct per value type in the C views, and regenerate every consumer.
  - Have the generated Rust view reader call one helper per field kind.
  - Collapse `RequestCall` to one object with a `function` field. The wire JSON stays the same. Keep the named call helpers in each language.
  - Use one generic optional type in Ada.
  - Run each target's formatter, or emit multi-line bodies, so generated diffs can be reviewed.
  - Lower each changed ratchet to its measured value.
- Proof: these checks.
  - `generate.py --all --check` passes in lint.
  - A planted edit in any generated file fails lint.
  - Every language library's routine installed checks pass.
  - No generated public identifier is longer than 60 characters.
- Defers: generic `Atomic<T>` emission, and moving Kotlin and Scala onto the Java records, which 0563 owns.



PM amendment, 2026-10-13. Remaining work. One generator developer owns `sdlc/generators/`, every generated file, `.gitattributes` and `policy.py`, and no other lane edits them. (1) Mark the 13 unmarked generated files, such as `libraries/c/src/session/primitives_generated.rs`, `libraries/cpp/include/thinkthen/calls_generated.hpp` and `libraries/python/thinkthen/_named_generated.py`, and delete the 4 `.gitattributes` entries that name deleted files. (2) Give each untitled schema arm a title, so no public name contains `arm_`, `ARM_` or `Alternative`. (3) Add a lint check that fails when a header, public type or public function name in a generated file exceeds 60 characters; static and private helpers are exempt. (4) Delete the unused C export named in the 0561 amendment. (5) Generate everything the 0574 and 0576 language lanes need from the generator before those lanes start: per-function typed answer accessors, typed errors with typed facts, plain lists for rank and filter, and typed recognize and relate builders. Record in Progress whether the collapsed `RequestCall`, the Ada generic optional type and the Rust per-field-kind helper are done, and drop any that are not. 0562 owns generated names; 0576 only grades them. Generated file sizes appear in the 0568 `library-size` report, and this ticket sets no separate budget. Proof: `sdlc/scripts/lint` passes, a planted edit in one generated file fails it, and every changed language's `check.sh` passes.

## Progress

- 2026-10-13 started
- 2026-10-13 next: Merged reviewed released-generator coherence as 73558a3e89f0bd76b5c69d72f7afbcbfc2aad747 with installed Dart and PHP checks passing; remaining language outputs stay with their owners.
PM amendment, 2026-10-14. 0581 supersedes item (5): generate no per-function answer accessors. Still open here: the public C header's `arm_1` and `arm_2` field names and the unused `thinkthen_request_plan_named_value` export.
