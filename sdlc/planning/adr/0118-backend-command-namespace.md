# ADR 0118: Put backend checking under backends

Status: accepted under Ian's 2026-10-05 approval of ticket 0416; amended by reviewed ticket 0440.

## Decision

Use `thinkthen backends check` as the canonical backend diagnostic. Keep the published `thinkthen check` spelling as a hidden compatibility alias. Route both to the existing check implementation with identical options, four request bodies, eight report rows, address/key resolution, no-send plans, secrecy, timeouts, diagnostics and exits. Help advertises `backends` and its `check` subcommand.

Reserve `questions`, `items`, `answers`, `checks`, `datasets`, `setups`, `findings`, `people` and `search` as top-level command names. They remain unrecognized commands with exit 2. Implement no proxy, resource command or new judging function.

`thinkthen runs audit` and `thinkthen runs diff` activate the previously reserved `runs` noun under ticket 0440. Keep published top-level `audit` and `diff` as hidden compatibility aliases. Both forms route before setup to the existing offline analysis, with identical options, outputs, diagnostics and exit codes. This introduces no judging function or business policy.

Current examples use the canonical spelling. Historical run evidence retains the spelling it executed. Ian can overturn the names or compatibility policy.
