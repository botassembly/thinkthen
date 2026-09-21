# Shared conformance cases

`cases.json` is the language-neutral behavior contract for the command, the Rust library, later language bindings, and database extensions. Every surface reads the same cases. The file fixes one canonical backend URL, question grammar, exact System One request bytes, decoded answers, bare values, detailed request identities, host-neutral bulk results, and the six public error kinds.

A successful exchange has one of two provenance values. `captured` names a committed public recording whose request and response match the embedded exchange exactly. `synthetic_contract` says the exchange was written against the accepted wire contract. Fault cases name deterministic injection points. They are schema contracts until the private engine runner lands with the one-crate merge.

The mixed `annotate` case asks `decide`, `choose`, `score`, and `tag` together in one request. The two-group case fixes aggregate request identity in question-set group order.

`rank` expectations name zero-based input indexes in output order beside their yes probabilities. `find` expectations name the selected zero-based input index, or `null`, and list every candidate probability in input order. Its `none` candidate appears last with a null index. A host maps these indexes back to its own container.

The pure-core integration test validates this file offline through the production question grammar, request encoder, response decoder, digest, answer rules, ranking, and find selector. It does not call a command or a network service.
