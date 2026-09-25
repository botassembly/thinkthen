# The engine's WidthActive message still says width

Status: Open. Found by ticket 0108's build and review, 2026-09-25. Owner: the engine.

## What happens

The rename from width to throttle reached the public error. `public/error.rs` says "throttle N is already active for this process; use throttle N or drop the throttle argument". The engine's own `WidthActive` display at `crates/thinkthen/src/engine/mod.rs:266` still says "width N is already active for this process; use width N or drop the width argument". The command prints that internal sentence through `cli/failure.rs`, so a command user reads the old word.

The R surface raises the public error's `Display`, so R prints the throttle sentence. R keeps one engine for each session and refuses a second `tt_engine` with other settings before the engine can raise `WidthActive`.

## Fix

Say throttle in the internal display, or print the public sentence from the command, so the two sentences match.
