# Site C examples need the approved JSON value wrapper

Status: open, required before public 0.1. Owner: the marketing lead under `sdlc/planning/ownership.md`. Fresh High review of ticket 0230 source `360f4e7a`, integrated candidate `6ff22a8e`, found this migration gap. The candidate is not landed when this issue is filed. Ian approved the C JSON success contract; the site examples must migrate with that contract before release.

`thinkthen_call` will return successful JSON as `{"value":...,"facts":...}`. Typed C entry points, the NULL failure signal and the direct process usage response retain their contracts. Six examples compare the complete allocated reply with a bare JSON answer: `site/examples/functions/{filter,choose,annotate,find,rank,tag}/c.c`. The seventh, `site/examples/install/c/first-call.c`, calls `atof` on the complete reply and gets zero from the opening brace. These examples will fail or teach incorrect consumption once 0230 lands.

Update all seven to read and use `value` through an appropriate JSON parser, preserve the existing result assertions and release the allocated reply using the C API. Search other direct consumers and any generated copies before declaring the migration complete. Keep NULL failures distinct from a successful JSON null. Update the site's recorded sample expectations where the displayed contract changes. Do not use substring extraction that breaks on nested values or escaped strings.

Proof: compile the changed examples against the reviewed 0230 header/library and run the closest existing offline sample checks. Verify the six function answers and numeric first-call answer from the value member. Reuse recorded responses; no provider or benchmark regeneration is needed. Record the exact linked candidate and final landed revision. The library's own corrected example does not prove the site copies.

The queue owner has filed this issue without editing site files or sending an external message. Ticket 0230 must retain this outstanding consumer migration explicitly. The public release checklist must not treat all direct consumers as migrated while this issue is open.
