# 0550: Fix the binding defects from the adversarial review

Status: OPEN.

Milestone: 0.2

## Outcome

A caller in every language sees its own producer's exception and Rust's admission message unchanged. Every binding assembles aggregate rows by one rule, treats a closed push as a stop signal and keeps the native retry signal. No dead admission code ships.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), bindings section, defects B1 to B8.
- Keeps: the session ABI, every installed case and its expected result.
- Changes:
  - **Producer exceptions.** Add a reader-failure kind in Rust for a producer that raised. Python, Ruby, JavaScript, PHP and R raise the original exception as the cause of the terminal error. Today four report a generic invalid-input error and R reports an io failure (`python/thinkthen/_calls.py:220`, `ruby/lib/thinkthen/session.rb:216`, `typescript/native.js:85`, `php/src/session/operation.php:107`, `r/thinkthen/R/requests.R:117`).
  - **Admission messages.** Give request decoding a typed cause in Rust, and delete the message-prefix matching:
    - `crates/thinkthen/src/public/request.rs:107`
    - `typescript/src/request_node.rs:41`
    - `ruby/src/ffi/request.rs:86`
    - `python/src/inputs.rs:30`
    - `c/src/ffi/session/ffi.rs:97`
  - **Aggregate rows.** Python and Ruby always add aggregate rows, as JavaScript and PHP do, and their recognize branch goes (`_calls.py:206-212`, `session.rb:178`). Rust sends one aggregate packet per call except for recognize, so the rows are the same today. The change removes the second rule.
  - **C++.** A closed push stops feeding and does not throw (`cpp/include/thinkthen/client.hpp:52`).
  - **C#.** Immediate failures carry the native retry flag and facts (`csharp/src/NativeSession.cs:16`).
  - **PHP.**
    - An operation keeps its client alive, so `(new Client)->start(...)` returns a live operation.
    - Find sends units and relate sends entities, as every other binding does (`operation.php:28`).
    - The methods `usage_persistence` and `finish_usage_status` take camelCase names. The PHP README maps the old names to the new ones.
  - **Dead code.** Delete `typescript/_requests.js` and `_requests.d.ts`. Nothing loads them.
  - **Usage counters.** A poisoned lock marks the usage as failed, and the held panic survives (`engine/usage.rs:219`, `public/options/observer.rs:57`, `public/options.rs:569`). Keep `Engine::finish_usage_status` and delete `finish_usage` (`public/engine.rs:236`).
- Proof:
  - An installed regression in each affected language covers producer exceptions and admission messages.
  - Installed regressions cover the C++ closed push, the C# retry facts, the PHP operation lifetime and the PHP find and relate inputs.
  - Rust tests cover the usage counters.
  - Routine checks pass.
- Defers: nothing.
