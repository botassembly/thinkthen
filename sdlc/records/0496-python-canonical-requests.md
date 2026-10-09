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

## Generated result design

This slice starts at `1ca01f5be` and incorporates the shared library-only diagnostic repair in `8275e656a`. The Python result target consumes `graph` and `prepare` for the actual `completesessionPacket` root. It generates native union dispatch, native field conversion, Python subclasses and editor declarations from the same Rust-derived schema. The template holds no result member inventory. Python keeps no copied decoder, admission rule or cache identity logic.

`_RequestSession._poll_typed()` converts actual native packets into generated subclasses of a frozen Rust `_NativeResult`. Each carrier stores its own `serde_json::Value` and schema identity. A nested access creates another independent carrier. Neither stores an engine or session handle. The existing serialized `_poll()` remains available as the private compatibility bridge. Pending remains `None`, and exhausted sessions retain the explicit end mapping. These private seams do not establish completion of the public named family.

Generated object classes and their type aliases are importable from `thinkthen._native_results`. Known attributes and index access use the generated native field conversion. Presence uses membership; missing attributes raise `AttributeError`, missing indexes raise `KeyError`, and present null remains `None`. Unknown members remain owned JSON. Mapping conversion makes independent mutable copies. Equality and pickle restoration preserve the whole document. Native JSON integers retain signed and unsigned 64-bit values; this slice adds no wider-number rule beyond the engine's existing JSON representation. Printing exposes the schema identity without input or failure content. Successful value carriers follow their ordinary value's truth behavior; failure carriers and carriers without a value raise `TypeError`. No missing sentinel or new null grammar is introduced.

## Generated result evidence and cost

The installed wheel's three focused cases pass: the retained held-provider scheduling and cancellation case, the retained zero-send admission and reader-failure case, and one bounded owned-result corpus. The corpus obtains a row through a real native session, closes the session and deletes the engine, then exercises typed nested access, mapping conversion, frozen attributes and pickle restoration. The same native conversion used by pickle preserves missing, null, false, zero, empty collections, unsigned integers, nested JSON, unknown members and embedded failures. The changed cases fail against the previous installed wheel because typed polling and generated classes are absent.

The wheel uses the lane's warm native target, locked offline dependencies, two build jobs and an 8 GiB memory scope with 1 GiB swap. Its artifact is `target/0496-native-edge/wheels/thinkthen-0.2.0-cp310-abi3-manylinux_2_39_x86_64.whl`; SHA-256 is `9e0c0348f1be04fd011f23e020457bef2febb3abe20f71dbaaa90fa12e0f18bb`. Focused native Clippy passes with warnings denied. The generated field dispatcher has an explained generated-code cognitive-complexity expectation. Regeneration checks, Python source and stub syntax, both exact source ratchets and root policy pass. Existing policy size warnings remain in unchanged files.

The source ceilings proposed for explicit reviewer acceptance are Rust 12313 and Python 8435, against prior ceilings 9736 and 8039. The Rust proposal includes 2405 generated dispatch lines; the Python proposal includes 311 generated class and alias lines. The generated stub adds 1597 nonblank lines. The target template adds 115 handwritten lines and the native carrier adds 153. Small module registration and typed polling route into those implementations. The existing installed consumer and type fixture receive bounded extensions. No handwritten compatibility reader is removed by this prerequisite.

The existing `tests/native_types.py` now declares concrete generated field use. Cached Mypy 1.18.2 passes a focused consumer of those fields against the installed wheel's stubs under a clean environment, using the owned interpreter through `--python-executable`. Named public calls, async scheduling, data frames and compatibility-reader removal remain subsequent ticket outcomes. Full shared parity, routine gates and release checks were not run for this bounded slice.
