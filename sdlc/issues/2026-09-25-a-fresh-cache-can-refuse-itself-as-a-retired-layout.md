# A fresh cache can refuse itself as a retired layout

Status: Open

Filed by the qf-request-cost session on 2026-09-25.

## What happened

A scratch benchmark ran `decide_many` through the public Rust API at throttle 32. It sent 2,000 records to the loopback backend's generic arm with `cache_at` on a new empty folder. One run in about forty failed with `the recording folder uses a retired layout`. The build predates the qf-request-cost change, and nothing else wrote to the folder. Thirty more runs of 200 records at throttle 32 each passed on both builds.

## Cause, read from the code

`identity::check` in `crates/thinkthen/src/engine/recorder/identity.rs` reads the marker, and then it looks for entries:

1. Worker B reads `.thinkthen-backend.json` and finds none.
2. Worker A publishes the marker, sends, and installs its first entry.
3. Worker B calls `has_entry`, finds A's entry, and refuses the folder as a retired layout.

A worker installs an entry only after the marker exists, so an entry seen after a missing marker shows that another worker published in between. Faster entry installation narrows the window between steps 2 and 3. The PostgreSQL 20,000-row warm runs at throttle 32 over an empty cache, so it can hit this.

## Options

1. When `has_entry` finds an entry, read the marker again. If the marker exists, match it as usual. If it does not, refuse as today. The change is a few lines in one function. This is the recommendation.
2. Publish the marker under the folder gate's exclusive side. This costs more and touches lock ordering.

A regression test needs the existing `pause` hooks in `identity.rs` to hold worker B between its marker read and its entry check.
