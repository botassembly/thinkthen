# 0038: Accept System One rounding at one hundredth

Ticket 0038 stops decimal-rounded System One distributions from failing paid runs. The generic distribution rule stays strict. System One alone accepts a total within `0.01 + member count × f64::EPSILON` of one, which includes decimal `0.99` and `1.01` after binary parsing and addition while refusing `0.98`, `1.02`, `0.85`, `1.15`, zero, and three ones.

## Evidence and implementation

The first phase made a refusal report its computed total, member count, and active tolerance without repeating untrusted text. Its one authorized probe ran 50 independent 17-option requests with retries and caching disabled. Forty answered and ten were refused. Every refusal reported total `0.9900000000000001`, 17 members, and tolerance `3.774758283725532e-15`. The ledger moved from 18,440,118 to 18,524,118 charged tokens. Bare output retained no actual usage totals, and no second paid run occurred.

The live result exposed the missing detail in an earlier `0.01` experiment. Binary `abs(0.99 - 1.0)` is `0.010000000000000009`, so a direct comparison can refuse a decimal one-hundredth boundary. The final System One allowance adds the existing member-count epsilon to that decimal hundredth. Another adapter continues to use the strict generic constructor unless its own evidence supports an exception.

The implementation preserves every reported probability. Choice order and cuts keep using those values. Score keeps dividing its weighted sum by the measured accepted total, so its value stays inside the named scale. Requests, digests, recordings, and successful detailed output do not change.

ADR 0019 now carries an in-place amendment with the original strict decision and the later System One evidence. The backend specification, resolved issue, main plan, and prospective plan agree. The correction pass is complete. Cache locking is next, followed by `find`, page 16 and transforms, the ADR 0017 and engine work, release preparation, six language libraries, and three database extensions.

## Review and validation

Phase-one design review rejected one draft until it separated measurement from behavior, recorded all known refusals, fixed one 50-call reservation, deferred layer ownership, and required round-trip numbers. Phase-one code review caught that default retries could turn 50 iterations into 150 requests; the final probe passes `--max-retries 0` and its fake binary requires it.

Phase-two design review accepted the adapter-specific rule and required an amendment to the decided ADR instead of replacing its history. Phase-two code review accepted the implementation and then caught the missing 50 calls in the main live table. The final plan records the calls, distinguishes unknown actual usage from the 84,000-token charge, and leaves the measured input total unchanged. The same reviewer accepted the remediation.

All four repository rungs pass on the final tree. The Rust suites pass 400 tests with no failure or ignored test, and the free probe self-test passes its three cases. All committed recordings replay, and all 18 green how-tos pass. Lint checks the exact 21,709-line ceiling. `git diff --check` passes.

The Rust ceiling rises by 155 lines for a safe public diagnostic, adapter and generic boundary tests, compiled secrecy proof, and the small tolerance seam. The implementation reuses the existing distribution, adapter error, score normalization, listener, and gate paths. No dependency or second response parser was added.
