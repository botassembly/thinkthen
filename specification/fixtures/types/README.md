# Type contract fixtures

`corpus.json` names result definitions, independent expected results and structural verdicts. Shared host conformance consumers also read its requests and case references. Case 41 retains independent scalar, UTF-16 and R offset expectations against the captured source text.

`self-test` validates the generated result schema and question-file request schema with the installed test-only `jsonschema` package. It checks duplicate names, nullable and false values, failure shapes, detail fields and complete result schemas. It executes no C library or provider.

Current native C execution belongs to the owned session checks in `libraries/c/tests/installed-session.py` and `installed_cases.py`. Those checks run the canonical Request interface and compare typed packet views with native packets. They cover the ten functions, errors, cancellation, images, declarations, offsets, context, cache, replay and provenance. The focused request preview door check separately pins preview bytes and zero requests without a key.

The shared host conformance suite owns complete request and response checks for each public case, including error branches and exact bytes. Its consumers retain this corpus unchanged.
