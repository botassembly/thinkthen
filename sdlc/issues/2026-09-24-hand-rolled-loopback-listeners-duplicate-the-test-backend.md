# Hand-rolled loopback listeners duplicate the test backend

Status: Open

Ticket 0092 moved the loopback listener into `conformance/backend`, so every surface's tests can share one backend. Its code review (`sdlc/records/0092-code-review.md`) found listeners that still bypass that backend.

1. `serve_once` in `crates/thinkthen/src/cli/conformance_tests/command.rs` answers one request on its own `TcpListener` for the counters case. It is 16 lines. Moving it waited because 0076 is building in `src/cli`.
2. Ticket 0076 adds two more hand-rolled listeners in `crates/thinkthen/src/engine/deadline_tests.rs`, still uncommitted at review time.

The goal is one shared test backend. `serve_once` should use `conformance_backend::Listener`, or the case arm of `Backend`. The deadline tests should use the backend's held arm (`/arm/held`, released by `Backend::release`) or `Listener` with `Canned::after_release`. A test that needs a new `Canned` builder edits `conformance/backend`. That folder must then be in the ticket's `opens`.

## Found by 0117

3. A third listener sits in `crates/thinkthen/src/cli/edge/deadline_tests.rs` (`spent`, line 38 on main at 0117's design review). It only needs a listener that counts connections.

0117 added the delay arm, `round`, and `wait` and left these listeners alone. It kept out of `crates` while other tickets built there. None of them needs a 0117 arm. `serve_once` needs one scripted reply, which `Listener::serving` gives. The engine deadline server needs a 503 with a wait, an answer, a hold, an arrival signal, and a check for a later connection. `Canned::status(..).asking(..)`, `Canned::after_release`, `Listener::answering_with_events`, and `Listener::connections` cover these. The two spent-deadline tests need `Listener::connections`. The transport tests in `src/engine/http.rs` keep their raw sockets, because they test the bytes on the wire. The move is a test refactor with its own ticket, and each moved test must still turn red on its old plant.
