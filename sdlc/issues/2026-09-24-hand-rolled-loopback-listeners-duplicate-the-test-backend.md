# Hand-rolled loopback listeners duplicate the test backend

Status: Open

Ticket 0092 moved the loopback listener into `conformance/backend`, so every surface's tests can share one backend. Its code review (`sdlc/records/0092-code-review.md`) found listeners that still bypass that backend.

1. `serve_once` in `crates/thinkthen/src/cli/conformance_tests/command.rs` answers one request on its own `TcpListener` for the counters case. It is 16 lines. Moving it waited because 0076 is building in `src/cli`.
2. Ticket 0076 adds two more hand-rolled listeners in `crates/thinkthen/src/engine/deadline_tests.rs`, still uncommitted at review time.

The goal is one shared test backend. `serve_once` should use `conformance_backend::Listener`, or the case arm of `Backend`. The deadline tests should use the backend's held arm (`/arm/held`, released by `Backend::release`) or `Listener` with `Canned::after_release`. A test that needs a new `Canned` builder edits `conformance/backend`. That folder must then be in the ticket's `opens`.
