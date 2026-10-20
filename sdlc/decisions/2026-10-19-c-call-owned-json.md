# Return owned canonical JSON from the blocking C call

Ticket 0638 authorizes the result choice before the core freeze. Ian can overturn it.

`thinkthen_run_call` wraps the public Rust `Engine::call` and returns one independent result owner containing the existing serialized `Call<RequestValue>`. C reads counted UTF-8 through `thinkthen_call_result_json` and frees the owner after its last reader. C's continuing call examples and installed call cases use this representation alone. The JSON grammar and semantic rules belong to Rust. Counted input follows the [input decision](2026-10-19-c-input-json.md).

Owned JSON costs one Rust serialization, a result buffer, and host JSON parsing. It uses the same complete call serializer as Rust consumers and gives hosts their ordinary JSON values without reproducing hundreds of nested layouts, discriminators, optional fields, and graph lifetimes. The owner is immutable and independent of engine and calling thread lifetimes. Foreign producers supply the existing canonical `RequestItem` JSON grammar to `RequestFeed::new`; the callback buffer lasts through the next callback or call return. The Rust feed owns scheduling and producer admission.

Generated views avoid host JSON parsing and expose typed fields, but currently build their graph by parsing serialized packets into another allocation tree. They retain the complete packet graph, generated converters and per-language declarations, pointer ownership rules, and a large header. They would also require a new converter for the blocking call's complete result. Their conversion and maintenance cost exceeds the saved host parse for the intended thin bindings.

The blocking call returns a distinct owned error with safe text, existing status kind, retryability, and canonical native Facts JSON. It uses the common panic containment helper without recording legacy per-thread diagnostics. All immutable result and error reads may move to another thread. Async hosts dedicate one worker to the blocking call and use its cancellation owner across threads. Producers run synchronously on the call's thread, including callback reentry into independent guarded exports; callbacks must contain host exceptions before returning the failure status.

Keep all still-shipping session, typed, view and thread-local error exports for the phase 3 hosts. Their ABI safety regressions remain required. Ticket 0690 removes those paths after migration. This decision changes no version or COBOL capacity.
