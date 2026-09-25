# The C surface: goals and anti-goals

This page predates the door that shipped. Ticket 0094 landed the door at `libraries/c`, and `libraries/c/DESIGN.md` holds its design. Where the two differ, DESIGN.md holds.

Shared rules live in [README.md](README.md). This page holds only what is particular to C.

## What really good looks like

Nobody writes C here. Rust exports the functions, `cbindgen` writes one header, and the first reader of that header is writing a binding for another language. He wants a header he skims in a minute, one rule for memory, a length on every string, and a stated answer on threads. Zig reads the header directly, and Java 22 and later call it with no glue code.

```c
#include <thinkthen.h>

thinkthen_outcome d;
int rc = thinkthen_decide("The command only reads files.", 34, cmd, cmd_len, NULL, &d);
if (rc != THINKTHEN_OK) return 4;
if (d == THINKTHEN_YES) run_it();
else if (d == THINKTHEN_NO) refuse();
else ask_a_person();
```

No handle exists in the first release. The engine holds one process-wide pool behind a width gate and starts it on first use; a handle owning nothing only confuses, and one arrives when a second backend exists, not before (205). Every string carries a length: the typed call takes the question, its length, the evidence, its length, an optional probability, an optional error slot, and the outcome — seven arguments, not six.

## Goals

- **The JSON door is the base.** `thinkthen_call` takes a request as UTF-8 text and returns the answer as UTF-8 text. Every verb rides it.
- **Three typed calls sit beside it:** `thinkthen_decide`, `thinkthen_choose`, `thinkthen_score`. The cost is two paths and three frozen signatures.
- **The bulk form is `thinkthen_decide_many`** (ADR 0017 pick 8): an array of pointers and an array of lengths, with a caller-owned array of answers to fill, exactly `libpq`'s parameter shape. It ships beside the door not for speed — measured 12 to 19 percent of a null-backend bulk call at 1,000 to 10,000 records, inside noise at 100,000, about 0.1 percent of a width-limited wire run — but because pointer-and-length hosts (Zig, Java FFM, Go) consume records without building JSON at all. One export, about 80 lines, one kept-index buffer.
- **One memory rule.** The library allocates every buffer it returns and takes it back through `thinkthen_free`. Every string is UTF-8 with an explicit length, and a trailing zero is appended for convenience; the length stays authoritative (205).
- **Failure is a returned code plus an optional error slot** on the typed call, holding the failure as JSON text and freed by `thinkthen_free`. No `errno` and no thread-local last error, and a static-phrase call cannot carry the engine's message (205).
- **`thinkthen_abi_version` returns an integer checked at load.**
- **Blocking only in the first release.** No callback and no pollable handle, because a batch already runs many requests at once inside Rust.

## Anti-goals

- **No panic crosses the boundary.** Unwinding into C is undefined. Every function catches at the edge.
- **No process-wide state.** No `atexit`, no signal handler, no global logger. A binding runtime may load two copies in one process.
- **No struct with public fields.** A field added later changes the size and breaks compiled callers.
- **No terminator carries meaning,** and no enum stands for anything open. Codes and labels grow. The outcome alone is closed.
- **No hand-written header.** It drifts from the exports with nothing to catch it.

## Where this language wastes time

- **A client per call.** Opening a handle builds a connection, so the engine holds the pool process-wide; 211 measured the gate holding 100 concurrent calls to 32 in flight.
- **A terminator hunt.** `strlen` reads the whole buffer, and a length removes that read and the copy behind it.
- **One call per record.** The barrier is cheap and the JSON encode is not. C has one wide container, and `thinkthen_decide_many` takes it. The library allocates nothing and the caller frees nothing.
- **Hand escaping the request.** A binding uses its own JSON writer.
- **Why the array call matters past C.** Every language that binds this header reaches the engine through it. With the JSON door alone, each binding builds request text per record in its own language, and the batching lives outside Rust. The array call hands them full width with no encoder.
- **Work the binding must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No binding compares or sorts records first. The engine holds the pool and the scheduler, `jobs` is one number for the process, and the typed single calls are serial while the array call is the bulk form. The header says so above the single form.

## How little code

`cbindgen` writes `thinkthen.h` from a thin `thinkthen-ffi` crate. It reads the exports, so the header cannot drift. That crate is never published, because every public name is `thinkthen`. Only the libraries and the header ship. The engine exports no C symbol of its own (ADR 0017 section 8, step 2); the C binding owns every exported name, so the one-prefix promise holds on day one and the engine's internal doors are compiled out of the shipping crate (205).

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

1. Do the typed calls ship in the first release, or does the JSON door ship alone first?
2. Is the request text its own versioned schema, or the command's shape?
3. Answered by 205: no handle at all. The engine's process-wide pool and gate serve every caller in the process, and the width stays one number for the process, as the shared rule says.
4. Answered: the array call ships, as `thinkthen_decide_many`, one export beside the door.
5. How does the array form report a per-record failure? A parallel array of codes, a sentinel in the outcome array, and a single failed call are the candidates.
