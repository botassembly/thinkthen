# The loopback reset reply has two silent edges

Status: Open

Found by the 0089 code review (`sdlc/records/0089-code-review.md`, FU3). No test hits either edge today.

1. `peek_request` in `crates/thinkthen/tests/backend/harness/mod.rs` never sees end-of-file after a client sends part of a request and closes. `peek` keeps returning the same bytes. The serving thread then polls every millisecond until the test process exits.
2. A request of 32 KiB or more is read in full instead of peeked. A `Canned::reset()` reply to such a request drops a drained stream. The client then sees a plain close instead of a reset.

Smallest fix: have the reset path panic when the request was read in full (`used == 0`). Treat an unchanged `seen` across polls, followed by a zero-length `read`, as end-of-file.
