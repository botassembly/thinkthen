# Use counted JSON for C call input

Ticket 0635 authorizes this choice between typed structs and JSON text. Ian can overturn it.

C callers supply length-delimited UTF-8 JSON under the [shared Request contract](../planning/adr/0125-one-request-contract-and-native-admission.md). The current session and plan consumers pass a complete canonical Request. The named blocking call in 0638 must use this same JSON input path for its question, input and options, with the function name supplied separately as its outcome requires. It must reuse Rust admission and must not introduce typed request carriers or a second grammar. This decision does not implement that call or choose its result representation.

## Reasons and costs

The [C guide](../../libraries/c/README.md), [example](../../libraries/c/examples/session.c) and [installed conformance consumer](../../libraries/c/tests/installed_cases.py) already construct canonical JSON. The session constructor in [session/ffi.rs](../../libraries/c/src/ffi/session/ffi.rs) validates the counted UTF-8 and calls `Request::from_json`. The plan constructor in [request_preview/ffi.rs](../../libraries/c/src/ffi/request_preview/ffi.rs) uses the same admission. Rust owns semantics, including the settled answer in the terminal packet from 0612. Selecting JSON requires no new public function or runtime routing layer.

JSON costs the caller serialization, correct string escaping and a request buffer. Rust parses the bytes and copies accepted input before returning. C receives malformed-input failures at runtime rather than compile time. Existing plan and session consumers check byte extents, UTF-8, admission, safe diagnostics and buffer independence at this boundary. Fed descriptors and reader failures also use counted JSON.

Typed input structs are rejected as the continuing C input path. They describe choices with tagged unions and counted pointers, but C still must construct a valid graph, manage its lifetimes and obey its pointer contracts. The converter in [native_request/ffi.rs](../../libraries/c/src/ffi/native_request/ffi.rs) traverses generated carriers, builds intermediate values, serializes JSON and enters shared Rust admission. Keeping this path would retain the carrier generator and its header surface, pointer and extent validation, recursive graph handling and conversion copies. It would require rewriting the existing C example and installed conformance input builder. It does not remove JSON parsing from the current implementation.

## Migration boundary

Keep the exported typed and component-based constructors, generated request carriers and header structs while phase 3 consumers use them. For example, Ada and COBOL construct typed inputs, while Go, C++ and Zig use the component-based JSON constructors. Add no features to these transitional entry points. Ticket 0690 removes them after those callers move. Ticket 0692 owns the runtime version check and COBOL cap; this decision changes neither.

C's documented caller and installed session and plan cases use canonical JSON. The find and relate lifetime cases also use full Request JSON. Result packet views and Plan value views remain available; the result representation belongs to 0638.

Retain isolated legacy ABI safety regressions until 0690 because the functions still ship. `LEGACY_INPUT_ABI` in the preview fixture checks typed carrier ownership, malformed tagged and JSON trees, null and aliased outputs, depth and cycles, exact numeric values and COBOL capacity. `SESSION_TYPED` checks copied session, descriptor and reader-failure carriers and closed-intake conversion. `SESSION_NAMED_REFUSALS` checks component view extents, null arguments and thread-local diagnostics. These are migration safety checks, not supported alternatives in the C consumer examples. The normal preview fixture, installed consumer and session conformance use only the chosen input path.
