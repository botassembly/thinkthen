# 0629: Add one Rust function that runs a call from start to final result

Status: OPEN.

Milestone: 0.2.1

Phase: 1

Depends on: 0612, 0628

Reviews: revision de79dcf, accept

## Outcome

- The public Rust library offers one function that runs a named call to completion. It takes a function name, question, input, options, an optional pull-style producer for fed input, and a cancellation handle. It returns the settled call (from 0612) or a typed error.
- Rust owns these rules and their error kinds and text: one producer, one reader, closed engine, missing final packet, and a producer failure kept as the cause. The usage error for a fed input with no producer moves into Rust.
- The direct Rust bridges (Python, TypeScript, Ruby, R, Polars and the MCP server) call this function and keep no loop of their own. The MCP server moves in 0639; each language's migration is a phase 3 ticket.
- The Rust library's own examples use this function.

### Proof

- On the M5 through pm job, the installed Rust library consumer runs all ten functions through the new function against the loopback backend: plain input, fed input, a producer that fails, cancellation while a provider reply is held, and deadline expiry.
- Errors and their text match `conformance/cases.json` error kinds.

## Evidence

Every binding writes its own loop to run a call: start a session, push input from a producer, read packets, enforce one reader and one producer, handle closed and cancelled states, notice a missing final packet, and chain the producer's error. Examples: Go `libraries/go/session.go:86-280`, Swift `libraries/swift/Sources/ThinkThen/OwnedSession.swift:15-292`, Dart `libraries/dart/lib/src/session/client.dart:40-140`, Ruby `libraries/ruby/lib/thinkthen/session.rb:100-296`, C# `libraries/csharp/src/OwnedSession.cs:24-111`, PHP `libraries/php/src/session/operation.php:7-101`. The copies already disagree on error text: "Closed session", "client is closed", "operation is closed", and Dart's `StateError('Native owner is closed')`. Go also checks a nil producer itself (`session.go:96`), and so does Ruby (`session.rb:107,128`). All of these rules belong in Rust.

## Progress

- 2026-10-19 started
- 2026-10-19 next: Implementationafb307004 is pushed with core/control and all-ten plain-fed checks passing in job1792423680791-41766, but jobs1792423680791-41766 and1792424137376-24355 also exposed example fixes and omitted invocation prerequisites, so a fresh developer will run the existing complete Rust entrypoint before installed-proof acceptance.
