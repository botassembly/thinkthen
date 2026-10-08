# Quick Fix: Route ThinkThen mail through agents

Status: COMPLETE. The project routes mail through the new repository; JSON, routing checks and fresh review pass.

The PM moved the active mailroom and froze writes to the previous repository. `sdlc/pm.json` now names `agents` as the mailroom repository; ThinkThen's mailroom name remains `thinkthen`. Fresh read-only review accepted b59ab03c6. JSON validation passed. With the supplied local agents mailroom checkout selected through `PM_MAILROOM`, `pm inbox --json` read the new mailroom move request. The checkout's older, unrelated `pm status` findings remain open and are outside this configuration fix.

## What the build taught us

The mailroom repository name is a routing setting in `sdlc/pm.json`. Reading the moved request through `pm inbox` checks the route without writing to either mailroom.
