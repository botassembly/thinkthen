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

Decided 2026-09-22 by the product side: this issue is the handoff. The library team picks it up from main when the `surfaces` branch next takes main, at A7. No separate message goes out, and no library work starts on it before the widening ticket lands.

## Second item, added 2026-09-22: the annotate form as a typed class

Ian asked, from a post comparing Jev's criteria objects with a `Literal` type on a generative model's output, where annotate stands. Ruling: a library user may declare an annotate form as a typed class in languages that have types. The field's type sets the question kind (a boolean is decide, an enumeration is choose, an ordered scale is score), and the field's description carries the instructions, string or JSON. Every field comes back with its value and its probability. The class is sugar over the question set: it serializes to the same compact JSON the question file holds, so the digest is identical to the file form and the cache is shared. Nothing is generated and nothing is validated after the fact, because the model only picks. Python gets a dataclass or Pydantic model, TypeScript an interface with a description map, Rust a derive; Ruby, R, C, and the databases use the question file. Designed with the public library shapes (A7). Ian can overturn it.

**Convention to match, added 2026-09-22.** Pydantic AI already ships this shape for Jev (https://pydantic.dev/docs/ai/models/typesafe/): a `bool` field is a yes-or-no question, a `Literal` or a string `Enum` with member docstrings is a pick-one, an `IntEnum` with member docstrings is a scale, `list[Literal]` is one yes-or-no per label, and a nested model is many questions in one request. Our Python form uses the same mapping so a reader of their docs recognizes ours: `bool` is decide, `Literal` or `Enum` is choose, an ordered `IntEnum` is score, `list[Literal]` is tag, a nested class is an annotate form. Two things stay ours: a description may be a JSON object as well as a docstring, and every field comes back as value, probability, and "not sure" at the band rather than a details dict. Ian can overturn the convention match.
