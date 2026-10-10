# R binding design

The [binding author guide](../BINDING-AUTHOR.md#r) owns the language contract. The [package README](README.md) documents the named calls and native results. The [canonical Request specification](../../specification/request.schema.json) owns admission.

R converts vectors, lists and data frames, maps missing atomic slots separately from native indexes, and supplies guarded interruption and once-only cleanup. The Rust Request boundary owns question grammar, reading, limits, cache keys and result facts. Generated result conversions own field names and presence. R supplies printing and ordinary access to those values.

A source package carries the Rust engine and pinned registry dependencies. Installation compiles its native library offline. An explicit installer-owned Cargo output directory can be reused across installs without entering the package.
