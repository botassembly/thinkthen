# Generated contracts

The public Rust types generate `client.schema.json`, `request.schema.json` and `row.schema.json`. `Settings::from_json` admits client settings; `Request::from_json` admits calls. The engine's `Code` registry generates every error name, ABI integer, family and record flag in `contract.json`.

The settings schema includes backend, model, address, secret, batch and bounded retry settings. A request holds its tagged question, optional name and records. The row schema describes each function's typed answer and optional metadata. The [0.2 rulings](../sdlc/planning/api-0-2-0/rulings.md) describe the surface conventions.

`abi.json` and the generated headers describe native exports and ownership. `thinkthen_code_family` exposes the registry's family as a process-lifetime NUL-terminated UTF-8 string. It returns NULL for unknown values, including zero; callers never free it. Existing code names and integer values remain unchanged.

Run the schema and header examples through a build job to regenerate these files. `scripts/contract-schema-check` compares generated output and checks documented code names. No contract fixture resolves outside network resources.
