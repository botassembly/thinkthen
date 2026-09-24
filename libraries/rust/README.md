# Rust examples

One runnable program for each `thinkthen` function, plus the deck's Rust slide. Each program depends on `thinkthen` the way an outside user does: by path, with default features off, through the public API alone.

```sh
export THINKTHEN_API_KEY=...
cargo run --example decide
```

Every program builds its engine with `Engine::from_env()`, so it reads `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE` as the command does.

## Width is per loaded copy

The engine's width is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads two copies, such as a Rust program and a native extension built from another surface, can run up to twice the width (ADR 0047 item 5).

## Checks

A bare `cargo test` builds every example, then runs `tests/examples.rs`. `cargo test --test examples` alone would run stale programs. It starts a loopback backend for each program, runs the program in its own process, and compares its output with the `.txt` file beside it. The loopback backend's generic arm answers every question by a fixed rule: the first option, level, label, or yes gets 0.9. The pinned text shows that rule's answers. A model would answer differently.

`check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It runs the formatter, Clippy, and the tests. It then runs the slide against the rung's own loopback backend. That run proves the port the rung hands every surface.
