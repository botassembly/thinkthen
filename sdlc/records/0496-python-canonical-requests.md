# Python canonical request design and evidence

## Starting point

This design builds on `66eba926f`. The [ticket](../tickets/0496-adopt-request-python.md) defines the complete Python outcome. The private native seam below establishes ownership for the named Python family. A private class and serialized packets do not establish public API completion.

## Execution boundary

`libraries/python/src/request.rs` owns one Python `_RequestSession`. `_Engine._request_session` decodes the shared canonical Request and starts the public native session with the closed Python, pandas or Python Polars surface identity. Native admission owns grammar, selector compatibility, defaults and limits. Python adds no semantic admission rules.

`_poll` returns `None` while work is pending, one owned packet when available, or an explicit end packet after exhaustion. It never waits for a provider. `_push` transfers one descriptor on `accepted`, retains caller ownership on `full`, and stops the producer on `closed`. `_finish` fixes EOF or forwards a shared typed reader failure. `cancel` stops intake. `close` is idempotent and drops the session without joining a held provider. Python finalization uses the same native Drop behavior. Cancellation does not fabricate terminal facts.

The session serializes owned packets as a temporary private bridge. The complete outcome requires graph-generated native Python result conversion and stubs, ordinary value conversion, a coherent named family, async scheduling and frame delegation. Existing public entry points and compatibility readers retain their owners until replacement parity permits removal. Shared R compatibility source remains available to its other consumers.

## Focused evidence

The existing installed Python consumer runner exercises two bounded native cases in `tests/test_complete_surface.py`. A held-provider case polls through an asyncio loop, queues a second record, cancels and closes before releasing the provider, then confirms exactly one send. A refusal case forwards malformed canonical requests and a reader failure through native rules and confirms zero sends. Both cases fail against the previous installed wheel because `_request_session` is absent.

The new wheel installs into an owned Python 3.13 environment outside the package source. The two focused cases pass there. Wheel compilation uses the lane's warm target with offline locked dependencies, two jobs, an 8 GiB memory scope and 1 GiB swap scope. The checks read no real caller credentials or home configuration.

The focused installed cases pass in 0.17 seconds. Offline Python Clippy passes with warnings denied. The source ceilings in this branch propose the measured totals for review: 9736 nonblank Rust lines and 8039 nonblank Python lines. These proposals add 132 native lines and 94 Python test lines. They do not establish acceptance for landing.

The required root policy check passes. Its existing source warnings include the Python engine class, which now holds 509 nonblank lines. The added method only dispatches to the new session module; a mechanical split would not reduce maintained behavior. Other warned files do not change in this slice.

## Source cost and lesson

The seam adds native ownership and scheduling operations rather than copying request grammar or admission from another binding. It retains one existing surface identity conversion. Generated Python conversion should consume the shared Rust graph; it must replace the handwritten output reader rather than restate its field inventory in a template. Handwritten growth in this slice consists of the private native seam and its two installed behavior cases; no compatibility code is removed by this prerequisite alone.
