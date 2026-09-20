# The C surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to C.

## What really good looks like

Nobody writes C here. Rust exports the functions, `cbindgen` writes one header, and the first reader of that header is writing a binding for another language. He wants a header he skims in a minute, one rule for memory, a length on every string, and a stated answer on threads. Zig reads the header directly, and Java 22 and later call it with no glue code.

```c
#include <thinkthen.h>

thinkthen_outcome d;
int rc = thinkthen_decide(NULL, "The command only reads files.", cmd, cmd_len, NULL, &d);
if (rc != THINKTHEN_OK) return 4;
if (d == THINKTHEN_YES) run_it();
else if (d == THINKTHEN_NO) refuse();
else ask_a_person();
```

## Goals

- **The JSON door is the base.** `thinkthen_call` takes a request as UTF-8 text and returns the answer as UTF-8 text. Every verb rides it.
- **Three typed calls sit beside it:** `thinkthen_decide`, `thinkthen_choose`, `thinkthen_score`. The cost is two paths and three frozen signatures.
- **One memory rule.** The library allocates every buffer it returns and takes it back through `thinkthen_free`. Every string is UTF-8 with an explicit length.
- **One opaque handle holds the pooled connection,** and a null handle means the common case. The header states thread safety at the top.
- **Failure is a returned code plus a message call.** No `errno`, no thread-local last error. `thinkthen_abi_version` returns an integer checked at load.
- **Blocking only in version one.** No callback and no pollable handle, because a batch already runs many requests at once inside Rust.

## Anti-goals

- **No panic crosses the boundary.** Unwinding into C is undefined. Every function catches at the edge.
- **No process-wide state.** No `atexit`, no signal handler, no global logger. A binding runtime may load two copies in one process.
- **No struct with public fields.** A field added later changes the size and breaks compiled callers.
- **No terminator carries meaning,** and no enum stands for anything open. Codes and labels grow. The outcome alone is closed.
- **No hand-written header.** It drifts from the exports with nothing to catch it.

## Where this language wastes time

- **A client per call.** Opening a handle builds an async runtime and a secure connection, so a binding holds one per process.
- **A terminator hunt.** `strlen` reads the whole buffer, and a length removes that read and the copy behind it.
- **One call per record.** The barrier is cheap and the JSON encode is not. C has one wide container, and a record verb takes it: an array of pointers and an array of lengths, with a caller-owned array of answers to fill. `thinkthen_decide_many(h, question, const char *const *items, const size_t *lens, size_t n, thinkthen_outcome *out)`. `libpq` passes parameters in that same shape, so the form is already familiar. The library allocates nothing and the caller frees nothing.
- **Hand escaping the request.** A binding uses its own JSON writer.
- **Why the array call matters past C.** Every language that binds this header reaches the engine through it. With the JSON door alone, each binding builds request text per record in its own language, and the batching lives outside Rust. The array call hands them full width with no encoder.
- **Work the binding must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No binding compares or sorts records first. The handle holds the pool and the scheduler, `jobs` is set on it once, and a binding opens one handle per process. The typed single calls are serial, the array call is the bulk form, and the header says so above the single form.

## How little code

`cbindgen` writes `thinkthen.h` from a thin `thinkthen-ffi` crate. It reads the exports, so the header cannot drift. That crate is never published, because every public name is `thinkthen`. Only the libraries and the header ship.

The shim holds those functions, the panic catch, the length and UTF-8 checks, the allocator behind `thinkthen_free`, the handle, and the code table. It holds no rule and no retry. A linker script and a module file keep every symbol under one prefix.

The crate builds a `cdylib` and a `staticlib`, and a release ships an archive per platform with both, the header, and a `thinkthen.pc` file. Prebuilt for Linux x86_64 and aarch64, macOS arm64 and x86_64, and Windows x86_64.

## Tests only this surface needs

- A small C runner feeds every conformance case through `thinkthen_call` under replay and compares the bytes, then repeats each single judgment through the typed calls.
- A memory checker calls every path, failures included, and ends at zero.
- A forced panic returns a code and the process lives.
- One handle called from many threads gives the single-threaded answers.
- A check compiles the header as C99, as C++, and through Zig.
- A bench counts rows a second through the array call against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.
- The array call with a short output array is refused, and the memory checker stays at zero.

## Open questions for the ADR

1. Do the typed calls ship in version one, or does the JSON door ship alone first?
2. Is the request text its own versioned schema, or the command's shape?
3. Does a null handle open a default client per loaded copy, or must every caller open one? Two handles in one process would be two widths, and the shared rule says the width is one number.
4. Do the array calls ship in version one? They cost three more frozen signatures. Leaving them out pushes per-record JSON building into every language that binds C.
5. How does the array form report a per-record failure? A parallel array of codes, a sentinel in the outcome array, and a single failed call are the candidates.
