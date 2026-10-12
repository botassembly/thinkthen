# 0550: Fix the binding defects from the adversarial review

Status: OPEN.

Milestone: 0.2

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision d4f576fb9ffe72557c357c900e544f7d7450aee0, accept

## Outcome

A caller in every language sees its own producer's exception and Rust's admission message unchanged. Every binding assembles aggregate rows by one rule, treats a closed push as a stop signal and keeps the native retry signal. No dead admission code ships.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), bindings section, defects B1 to B8.
- Keeps: the session ABI, every installed case and its expected result.
- Changes: one fix for each binding defect below.
  - **Producer exceptions.** Add a reader-failure kind in Rust for a producer that raised. Python, Ruby, JavaScript, PHP and R raise the original exception as the cause of the terminal error. Today four report a generic invalid-input error and R reports an io failure (`python/thinkthen/_calls.py:220`, `ruby/lib/thinkthen/session.rb:216`, `typescript/native.js:85`, `php/src/session/operation.php:107`, `r/thinkthen/R/requests.R:117`).
  - **Admission messages.** Give request decoding a typed cause in Rust, and delete the message-prefix matching:
    - `crates/thinkthen/src/public/request.rs:107`
    - `typescript/src/request_node.rs:41`
    - `ruby/src/ffi/request.rs:86`
    - `python/src/inputs.rs:30`
    - `c/src/ffi/session/ffi.rs:97`
  - **Aggregate rows.** Python and Ruby use the single add rule that JavaScript and PHP use, and their recognize branch goes (`_calls.py:206-212`, `session.rb:178`). Rust sends one aggregate packet per call except for recognize, so the rows are the same today. The change removes the second rule.
  - **C++.** A closed push stops feeding and does not throw (`cpp/include/thinkthen/client.hpp:52`).
  - **C#.** Immediate failures carry the native retry flag and facts (`csharp/src/NativeSession.cs:16`).
  - **PHP.**
    - An operation keeps its client alive, so `(new Client)->start(...)` returns a live operation.
    - Find sends units and relate sends entities, as every other binding does (`operation.php:28`).
    - The methods `usage_persistence` and `finish_usage_status` take camelCase names. The PHP README maps the old names to the new ones.
  - **Dead code.** Delete `typescript/_requests.js` and `_requests.d.ts`. Nothing loads them.
  - **Usage counters.** A poisoned lock marks the usage as failed, and the held panic survives (`engine/usage.rs:219`, `public/options/observer.rs:57`, `public/options.rs:569`). Keep `Engine::finish_usage_status` and delete `finish_usage` (`public/engine.rs:236`).
- Proof: installed checks, as follows.
  - An installed regression in each affected language covers producer exceptions and admission messages.
  - Installed regressions cover the C++ closed push, the C# retry facts, the PHP operation lifetime and the PHP find and relate inputs.
  - Rust tests cover the usage counters.
  - Routine checks pass.
- Defers: nothing.

PM amendment, 2026-10-11. The C# fix may add two C exports: `thinkthen_session_error_retryable()` and `thinkthen_session_error_facts_json()`. They read the calling thread's last immediate session failure, the same failure that `thinkthen_session_error_message()` reports, and reuse the existing facts JSON. The borrowed facts stay valid until the next immediate session failure on that thread, and successful calls leave them in place. The constructor and session error slots stay independent, as the installed `session.c:205,236` checks pin. The header documents both exports, and an installed C check covers them. Ian's [efficiency approvals](../decisions/2026-10-11-efficiency-approvals.md) cover additive exports before publication.

## Progress

- 2026-10-11 started
- 2026-10-11 next: Session diagnostic accessors await a PM ABI ruling because constructor-slot reuse breaks the retained installed C lifetime contract; other binding repairs continue.
- 2026-10-11 next: Committed binding fixes passed fresh review, and the PM approved the remaining session-error accessor delta with header documentation and installed C coverage.
- 2026-10-11 next: Approved thread-local session diagnostic accessors are implemented and reviewed, with installed C and managed checks passing; whole-ticket checks and landing await the approved dependency order.
- 2026-10-11 next: Exhaustive combined-source measurement confirms C Rust matches 32161 but C fixtures measure 3418 against unchanged 3340; the bounded survey preserves all named-export and lifetime assertions and finds only 23 safe shared-helper lines, leaving 55 excess lines that require a limit ruling rather than weakened checks.
- 2026-10-12 next: Canonical Ada declaration repair d9c6a7887 passes generator, header, compiler ABI and unchanged archive selftest; final combined e52ac9593 has six measured source-limit failures—C fixtures 3418/3340, Java 4161/4071, Kotlin 1661/1660, Scala 1675/1674, Ada declarations 8653/8638 and Ada Python 551/550—with all assertions retained and no ceiling raised.
