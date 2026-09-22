# Typed description builders in every library

Status: open, for the library team. Filed 2026-09-22 by the product side. Ian can overturn it.

## The problem

The question file will take a JSON object wherever it takes a description today (`2026-09-21-the-question-file-cannot-carry-typesafes-structured-fields.md`, rulings 1, 2, and 8). In the shell that is an object in a file that is already JSON. In a library a user should not build that object by hand from a dictionary of strings. The compiler or the runtime should type it.

## What the libraries add

One description type per language, with the three recommended keys, all optional, plus a way to pass any other keys the model may read:

- Rust: `Description { what, not_for, examples }` with a builder, and `Question::choose(q).option("billing", desc)`.
- Python: a `Description` dataclass, or a plain `dict` accepted where a string is accepted today. Both work; the dataclass is documented.
- TypeScript: an interface `Description` on the options object, string or object per option.
- Ruby, R, C, and the databases: the plain form of that language (a hash, a named list, a JSON string in C and SQL). No builder where the language has no types.

Every surface serializes the object to compact JSON with keys in the written order, the same bytes the shell sends, so the cache key matches across surfaces.

## What must stay true

- A string stays valid everywhere. Nothing that works today changes.
- The library never reorders or normalizes the object. It goes through unchanged.
- The digest across surfaces is the same for the same object.

## Order

With the public library shapes (A7), after the widening ticket lands in the engine, because the builders serialize to the grammar that ticket defines.
