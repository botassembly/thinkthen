# Related projects

Written 2026-09-19 from a reading of five public repositories. It answers one question for Ian: does anything that already exists do what `thinkthen` does, or stand in the way of libraries with the same grammar? Nothing does.

| Project | What it is | Publisher | License | What it has of ours |
| --- | --- | --- | --- | --- |
| `typesafe-sdk` on PyPI | The vendor's Python client. `TypeSafeClient` and an async twin, `system_one(state, questions)`, the three question types, retries, one error class per status | The vendor | MIT | Nothing beyond the call itself |
| `@typesafe-ai/sdk` on npm | The vendor's TypeScript client, with `noul()`, `choice()`, `score()`, and `models.list()` | The vendor | MIT | Nothing beyond the call itself |
| `typesafe-ai` on crates.io | A community Rust crate: request builders, an async client, and a blocking client. Two example programs: a semantic grep, and a checker that runs a folder of saved questions over a pull request with a dry run | One community author | Apache-2.0 | Examples alone touch `find`, a question set, and a dry run. The crate has none of them |
| `typesafe-rs` | A community Rust client that mirrors the vendor's clients, with a mock server and a conformance suite of scripted HTTP exchanges. Its roadmap names rate-limit layers, other backends, and WebAssembly as a stretch | One community author | MIT or Apache-2.0 | Nothing. Its conformance suite checks retries and headers and never a judgment |
| The vendor's agent skill | One `SKILL.md` of about 150 lines that teaches a coding agent to compose the three question types and to read the vendor's live pages | The vendor | MIT | Nothing |

No project has a gate by exit code, a band with an unresolved middle, `filter`, `rank`, `annotate`, question files, record and replay for a user's own tests, detailed rows, or evals. No license blocks reuse.

## What the reading adds to the interface audit

- The service also sends `retry-after-ms` beside `retry-after`. Ticket 0013 honors the wait, and it reads both.
- `GET /v1/models` lists the models with a name, a description, and a release date. `roadmap.md` holds a `models` listing as an idea.
- Every response carries `x-typesafe-request-id`. The tool keeps it out of rows and recordings, because it changes on every call.
- The community client's own product page lists the same unknowns as ours: the ceiling on questions in one request, and the size limit of a request. Tickets 0015 and 0017 measure both.

## Names

The vendor's words are Noul, Choice, and Score in every language. The registries hold `typesafe-sdk`, `@typesafe-ai/sdk`, `typesafe-ai`, and `typesafe-rs`. The name `thinkthen` appears in none of these projects. Nobody has checked the registries themselves.
