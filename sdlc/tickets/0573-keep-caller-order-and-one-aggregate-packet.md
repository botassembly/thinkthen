# 0573: Keep the caller's order and send one aggregate packet

Status: OPEN.

Milestone: 0.2

Reviews: revision 69f8dd8, accept

Reviews: revision 574acaa, accept

## Outcome

Every language library sends options and kinds in the order the caller wrote them, so cache and replay hit. `recognize` sends one aggregate packet.

## Evidence

- Starts from: a 2026-10-12 replay of the ten function samples in every language library against the site's saved recordings.
  - Go encodes choose's described options (`AuthoredOptionsObject`) and recognize's kinds (`RequestDefinitionRecognizeRecognize.Kinds`) as maps, which Go sorts by key. A caller who writes billing, shipping, account sends account, billing, shipping. A Go caller cannot send inline options or kinds in its own order.
  - Swift types choose options as `[String: InputAuthoredDescription]`, which loses order. The sample fell back to `client.parseQuestion`.
  - The Objective-C client takes choose and annotate options only as `NSDictionary`, and `NSJSONSerialization` writes keys alphabetically. Choose sends account, billing, shipping instead of the written order.
  - `smoke-bindings.mjs` still uses Linux-only paths for the JVM, C#, PHP, Dart, Swift, Zig, TypeScript and Ruby, so those installed checks cannot run on macOS.
  - Scala's `ScalaEngine.javaMap` converts with `.toMap`. Scala maps with more than four keys lose insertion order, so five or more options or kinds may reach the engine reordered.
  - `recognize` sends a second, empty aggregate packet, `{"kind":"aggregate","function":"recognize","value":[]}`, after the filled one. It crashed the first C, Zig and COBOL readers until they skipped it.
- Keeps: each library's public calls and the request schema.
- Changes:
  - Generate ordered types for options and kinds in Go, Swift, Objective-C and any other library whose native map loses order, for choose and annotate options and recognize kinds: a list of pairs or an ordered map.
  - Convert Scala maps through an insertion-ordered map.
  - Pick each platform's library paths in `smoke-bindings.mjs`, as `native_install` now does for `.dylib` on Darwin.
  - Send one aggregate packet per recognize request. Delete the reader workarounds that skip the empty one.
- Proof:
  - A conformance case with five options and five kinds in non-alphabetical order replays in every language library.
  - An outside-in test shows recognize emits exactly one aggregate packet.
  - Every language library's routine installed checks pass on Linux, and on macOS at the candidate.
- Defers: nothing.

## Progress

PM amendment, 2026-10-13. Remaining work. The single recognize aggregate packet is done (7ed9f7829). Lane G3 adds one case to `conformance/cases.json` with five options and five kinds in non-alphabetical order, which every library replays, and makes `site/scripts/smoke-bindings.mjs` pick `.dylib` paths and `DYLD_LIBRARY_PATH` on Darwin. In the language lanes: Java, Kotlin and Scala take options and kinds through an ordered type instead of `java.util.Map`; C# takes a list of pairs; Objective-C sends its `TTOrderedMap` through a generated encoder instead of `NSJSONSerialization`. Correct the Python test name that mentions the removed empty final packet.
