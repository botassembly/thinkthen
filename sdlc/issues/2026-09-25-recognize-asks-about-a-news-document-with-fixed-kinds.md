# recognize asks about a news document with fixed kinds

Status: Open

Filed by the marketing session on 2026-09-25, from Beatles Bench ticket 0006. Build: thinkthen 0.0.1, release of 2026-09-24.

## What happened

Every recognize request sends the words in `crates/thinkthen/src/core/recognize.rs` `DETECTION_WORDS` and `KIND_WORDS`:

- The snippet "shows five consecutive words from a news document."
- The kind question offers PER, ORG, LOC, and MISC, with their fixed descriptions.

The bench call named the kinds person, song, album, and place. The request never names a song or an album. It asks about organizations, events, and products the user did not name.

## Why it matters

The output still came back right on the talk's sentence. A user reading the recording sees a news-document question about a sentence on a song. A kind with no close CoNLL match may land in the wrong bucket.

## Asks

1. Say in `specification/recognize.md` how the user's kinds reach the request, and why the fixed kinds stay.
2. If the fixed wording is not needed, say "a text" and describe the user's kinds.

Ian can overturn both.
