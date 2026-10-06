# Distinguish CLI reader positions from scalar SDK results

The installed TypeScript check compared a CLI-read stdin document with a scalar SDK result as though both had reader metadata. The records contract assigns positions to CLI readers; the unchanged scalar SDK details type has no position. The existing decide and score shape test now pins the CLI position, asserts the SDK has no position, and compares every remaining result field. Its prior attempts exclusion stays unchanged.

One fresh read-only review accepted the change. The complete installed TypeScript check passes, including 59 tests, shared conformance, type checks, loader refusals and package contents. No product code changed.

## What the build taught us

Compare shared contract fields while asserting each surface's documented reader behavior explicitly.
