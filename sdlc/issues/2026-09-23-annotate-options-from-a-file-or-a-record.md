# Annotate options from a file or a record

Status: Open. Checked 2026-09-25: `specification/annotate.md` still keeps options inline.

Ian asked on 2026-09-23 how annotate picks an album when the album list lives in the user's data. Today it cannot, short of generating the question set.

## What exists

- `choose` takes options as arguments, from a question file, or per record with `--options POINTER` under `--jsonl` (`specification/choose.md`, ADR 0009 item 4).
- An `annotate` question set holds each `choose` question's options inline in the file (`specification/annotate.md`). No entry takes options from a file or from the record.

So "annotate each song with its album, singer, and mood" needs a script that writes the album list into the question set first.

## The ask

1. A `choose` or `tag` entry in a question set may take `"options": "@albums.txt"`. The file holds one label per line, or `label<TAB>description`. The path resolves relative to the question set.
2. An entry may take `"options": "/candidates"`, a pointer into each record, as `choose --options` already does. Under `--jsonl` only, with the same rules and refusals.
3. Options are read and expanded before the digest is computed. The digest names the actual options, so a changed album list is a changed question. The cache misses only when the list really changed.
4. Libraries take a plain list (`options=albums`). The typed-class form cannot hold a list known only at run time, so the builder form carries it. SQL takes a list expression, such as `(SELECT list(name) FROM albums)` in DuckDB.

## Why it matters

Relate between two kinds is exactly this call. Per song, one `choose` over the people and one over the albums, with the options drawn from the entity list (`sdlc/planning/relate-design.md`, the planner). Build the options source once and relate's planner and annotate share it.

Found by the product side. Ian can overturn any line.
