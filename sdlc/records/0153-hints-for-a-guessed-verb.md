# Ticket 0153 build record

## Scope correction, 2026-09-27

The build preflight at `46c87f8f` confirmed that ticket 0146 is on main and the stopped-record failure keeps its record number and boxed cause. It also found that the accepted typed match needs `JsonError`, which the private core JSON module did not re-export. No runtime change was retained from that preflight.

Ticket amendment `0d488e15` adds `core/mod.rs` and a two-line budget for a command-feature-gated crate-private re-export. A fresh read-only Codex Sol Medium reviewer returned ACCEPT. It checked that the exact record-1, typed-JSONL, no-field conditions remain, duplicate-name and nonfinite errors retain their paths, the core remains pure, and no public library surface is added. The coordinator accepts this bounded correction. The ticket checker and diff whitespace check passed. Runtime implementation and final code review remain open.
