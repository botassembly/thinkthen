# The public `Entity` Debug prints its kind

Status: Open.

Filed on 2026-09-25 by ticket 0127, found while it marked the second relate entity's kind in the secrecy sweep. No key was used, and no request left the machine.

## What happens

`Entity`'s `Debug` in `crates/thinkthen/src/public/relate.rs:145` withholds the name and prints `kind` in clear. The core `RelationEntity` `Debug` in `crates/thinkthen/src/core/relation.rs:19` withholds both. A kind is the caller's text, so a library user's `{:?}` of an entity or of anything that holds one can print it. The secrecy sweep covers the command's diagnostics, and none of them formats a public `Entity`, so no test fails today.

## What would fix it

Withhold `kind` the way the core type does: print its length through `Withheld`.

## Done when

A `Debug` line of a public `Entity` built with a marked kind holds no marker, and a test pins it.
