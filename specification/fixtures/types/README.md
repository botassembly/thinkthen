# Type contract fixtures

`corpus.json` names each result definition, a C JSON door request, an independent expected result or error, and the structural verdict. A `case_id` points to the offline backend's existing shared case. Case 41 carries the non-BMP offset expectation and reuses its captured exchanges without copying them here.

`self-test` runs from the test rung. It validates the result schema, which a Rust unit test generates, and the `doorRequest` definition of the question-file schema with `python3` and the installed test-only `jsonschema` package, as checked by the install rung. It then builds the local C door and conformance backend under the shared heavy lock, loads the platform's Cargo shared-library artifact, calls the real C JSON door through `ctypes`, and compares its output with the corpus. A malformed result shape is checked only as a schema rejection; a valid request must also pass the real door. The test runs with a fake key and local loopback address. It uses a fresh temporary cache for each case and cleans up the backend process.

Later port integrations run this corpus through their public bindings. The existing shared conformance suite still owns the complete request and response checks for each case, including error branches and exact bytes. This corpus pins distinct type boundaries rather than copying that suite.
