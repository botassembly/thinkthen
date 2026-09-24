# The C door

`libthinkthen` exports the engine to any language that can call a C library. `include/thinkthen.h` declares its 19 functions, and [DESIGN.md](DESIGN.md) gives the ownership, threading, and error rules behind them.

```sh
cargo build --release
cc -std=c11 -I include examples/slide.c -L target/release -lthinkthen_c -o slide
```

A release renames the built files `libthinkthen.so` and `libthinkthen.a`, with the soname `libthinkthen.so.0`. `thinkthen_engine_new` reads `THINKTHEN_BASE_URL`, `THINKTHEN_API_KEY`, and `THINKTHEN_CACHE` as the command does.

## Width is per loaded copy

The engine's width is the limit on requests in flight at once. It holds per loaded copy of the engine. A process that loads two copies, such as a C host and a native extension built from another surface, can run up to twice the width (ADR 0047 item 5).

## Checks

`cargo test` builds the door and compiles each C program under AddressSanitizer against it, linked through the header and the soname as a host links it. `tests/door/main.rs` checks the soname and the exported symbols, the examples' pinned output, and the C rows in `tests/c/`. `tests/door/cases.rs` feeds every applicable shared case through `tests/c/driver.c`. `tests/door/bytes.rs` holds the door's bare values to the command's bytes.

The loopback backend's generic arm answers every question by a fixed rule: the first option, level, label, or yes gets 0.9. The pinned text shows that rule's answers. A model would answer differently.

`check.sh` is this surface's entry in the surface rung, `sdlc/scripts/surfaces`. It exits 77 when no C compiler is found. It runs the formatter, Clippy, and the tests, and then runs the slide against the rung's own loopback backend.
