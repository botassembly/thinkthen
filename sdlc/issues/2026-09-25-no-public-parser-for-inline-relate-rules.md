# No public parser for inline relate rules

Status: Open

Filed by ticket 0111 on 2026-09-25.

## What happens

The command reads inline relate rules as `NAME` or `NAME=SOURCE:TARGET` in `inline_rule` at `crates/thinkthen/src/core/relate_file.rs:295`. The public API does not export that parser. The PostgreSQL binding's `thinkthen_relate(query, rules text[])` takes the same spellings, so `databases/postgresql/src/relate.rs` copies the function.

## Why it matters

Two copies of one grammar can drift. A later change to the command's spelling would leave the SQL surface reading the old one.

## What would fix it

Export a constructor such as `RelationRule::parse_inline(text)` that returns the command's rule and its usage error. The bindings then call it and drop their copies.
