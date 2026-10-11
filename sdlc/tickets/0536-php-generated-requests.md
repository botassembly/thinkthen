# 0536: Build PHP requests from generated input types

Status: OPEN.

Milestone: 0.2

## Outcome

PHP builds each request from generated input types and serializes it with one encoder. PHP code no longer concatenates request JSON or maps functions to input kinds by hand.

## Evidence

- Starts from: gap 2 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md). `libraries/php/src/session/operation.php:20-34` picks `units`, `entities` or `records` per function and concatenates the request string. PHP is a C-interface host, so the owned JSON session from 0503 stays its transport. [The binding guide](../../libraries/BINDING-AUTHOR.md) asks for generated declarations and mechanical conversions from Rust.
- Keeps: every named call, result class, failure class and cleanup behavior that 0526 qualified, including the destructor freeing a session while batches are open.
- Changes: generate the PHP request input types from the same graph the other C-interface hosts use. `libraries/csharp/src/RequestInputs.g.cs` and `libraries/dart/lib/src/session/inputs_generated.dart` are the models. Replace the concatenation and the kind mapping with them. Lower the PHP ceiling.
- Proof: the routine installed PHP checks pass.
- Defers: nothing.
