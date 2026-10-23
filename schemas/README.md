# Call contract

`client.schema.json` describes the settings admitted once by `Client::from_json`. `request.schema.json` describes the JSON accepted by `SdkRequest::from_json`. `row.schema.json` describes each serialized `Row`. Both are JSON Schema draft 2020-12 data derived from the actual Rust types. Regenerate the data from the repository root with `cargo run -p thinkthen --example schema`, through the normal build job. The gate compares published data with the types and validates shared requests and actual rows offline. No schema resolver accesses HTTP or files.

A call carries only function, tagged question, function-specific options and initial records. Client settings carry mode, adapter, address, key source, model and batch. Records may be omitted when the caller will stream them after opening a call.

An inline question is `{"type":"inline","value":"Ready?"}`. Recognize uses a kind array as its value, relate uses a rule array, and annotate uses a toolbox object. A named toolbox question uses `{"type":"toolbox","value":{"questions":{"ready":{"decide":"Ready?"}}},"name":"ready"}`. A reference uses `{"type":"reference","name":"ready"}`. A library never interprets inline text as a filename. Each function has one schema alternative with its own inline value and options. Semantic admission supplies stable errors for invalid thresholds, duplicate labels and invalid toolbox definitions.

Client settings select embedded mode by default. `adapter: "stub"` needs no credentials or network. `key_source` optionally names the selected backend's credential environment variable. Proxy mode and reference execution return `usage.question` until the proxy transport exists.

Answered and unsure rows carry `detail.source` with the caller-visible question identity, backend and served model. Optional `detail.probabilities` holds the function's backend detail. Failed rows have no answering source.
