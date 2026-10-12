# 0573: Keep the caller's order and send one aggregate packet

Status: OPEN.

Milestone: 0.2

## Outcome

Every language library sends options and kinds in the order the caller wrote them, so cache and replay hit. `recognize` sends one aggregate packet.

## Evidence

- Starts from: a 2026-10-12 replay of the ten function samples in every language library against the site's saved recordings.
  - Go encodes choose's described options (`AuthoredOptionsObject`) and recognize's kinds (`RequestDefinitionRecognizeRecognize.Kinds`) as maps, which Go sorts by key. A caller who writes billing, shipping, account sends account, billing, shipping. A Go caller cannot send inline options or kinds in its own order.
  - Swift types choose options as `[String: InputAuthoredDescription]`, which loses order. The sample fell back to `client.parseQuestion`.
  - Scala's `ScalaEngine.javaMap` converts with `.toMap`. Scala maps with more than four keys lose insertion order, so five or more options or kinds may reach the engine reordered.
  - `recognize` sends a second, empty aggregate packet, `{"kind":"aggregate","function":"recognize","value":[]}`, after the filled one. It crashed the first C, Zig and COBOL readers until they skipped it.
- Keeps: each library's public calls and the request schema.
- Changes:
  - Generate ordered types for options and kinds in Go, Swift and any other library whose native map loses order: a list of pairs or an ordered map.
  - Convert Scala maps through an insertion-ordered map.
  - Send one aggregate packet per recognize request. Delete the reader workarounds that skip the empty one.
- Proof:
  - A conformance case with five options and five kinds in non-alphabetical order replays in every language library.
  - An outside-in test shows recognize emits exactly one aggregate packet.
  - Every language library's routine installed checks pass.
- Defers: nothing.

## Progress
