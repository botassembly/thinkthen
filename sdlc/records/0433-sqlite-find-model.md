# 0433: Preserve SQLite find’s selected model

SQLite find now applies its validated per-call model through the public question builder. A caller override reaches the wire and cache/replay identity without changing the engine default for later calls. Existing result forms, candidate order, none and failure behavior remain.

The new outside-in regression failed before the fix by observing model A instead of requested B. It passes after the four-line correction: invalid overrides send nothing, B cache and strict replay send nothing, unrecorded A replay refuses, and the later ordinary call sends A. SQLite 3.50 value, budget/cancellation and unit checks, release build, formatting, Clippy, policy and both ratchets passed. One fresh read-only review accepted. Full tests and lint run on the landing commit before push.

The same landing records Ian’s build-go, one aggregate $20 allowance and active lanes, and removes stale group-lookup wording in 0442. No paid call was needed. Shared formats and full SDK parity remain open in their owners.
