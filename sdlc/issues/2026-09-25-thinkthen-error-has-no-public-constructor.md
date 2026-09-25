# `thinkthen::Error` has no public constructor

Status: Open

Filed by ticket 0111 on 2026-09-25.

## What happens

A binding cannot make a `thinkthen::Error` of its own. The PostgreSQL binding refuses bare text, a spent request total, an over-cap file, and a cancelled wait before or after any engine call. Each refusal needs the same kind, message, and retry signal as an engine error, so the binding keeps its own `Refusal` type beside `Error` and converts one into the other.

## Why it matters

Each binding writes the same small type, and each copy maps kinds to host errors twice.

## What would fix it

Add public constructors such as `Error::usage(message)` and `Error::local(message)`, or document the pattern each binding follows. The owner of the public API decides.
