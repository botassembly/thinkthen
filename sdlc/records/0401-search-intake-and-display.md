# 0401: Shared intake, search display and rank sets

All four slices landed: ordered named inputs and line windows across seven commands; filter/rank line numbers, scores and bounded neighbor display; the independent find display adapter; CLI/Rust rank question sets. Focused boundary/API regressions, full Linux tests/specification, documentation replay and lint passed at their landings.

Intake never infers file intent from label spelling. Windows preserve physical lines, never cross files and refuse JSONL/table/pointer conflicts. Display neighbors remain local context, leave request/store identity unchanged and use bounded owned snapshots. Filter retains its completed prefix on a late failure; rank discards held output; find judges one complete candidate set, preserves none and complete probabilities, and changes no requests for display.

Rank sets independently rank each member, visit members in authored turns and suppress repeated original positions without refilling a turn. Top is applied after merging; details attribute the selecting member’s probability/identity. One-member wire parity and individual-member recordings/cache reuse remain. Any failed member refuses the whole result; preflight, secrecy, budgets, cancellation/worker joins, preview packing and backend selection retain their separate cases.

Left: SQL sets belong to 0417 later and foreign binding sets to 0418 later; no cross-surface set support or paragraph windows are implied. These decisions replace unreferenced design/build-stage notes without changing public behavior.
