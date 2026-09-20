# The Python surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to Python.

## What really good looks like

A Python user judges a library in the first minute. The install is one wheel, the import is instant, and the first example is three lines with nothing to construct. The answer lands in a plain `if` or `match`, and the editor knows its type before anything runs. Ctrl-C works, threads go faster, and the library still holds up on a long column in a notebook.

```python
import thinkthen as tt

if tt.decide("The command only reads files.", command):
    run_it()

# under an event loop, the same call
from thinkthen import aio
ok = await aio.decide("The command only reads files.", command)
```

## Goals

- One compiled module, and nothing heavy imported beside it.
- The lock is released for the whole time Rust waits. Threads and record verbs then run in parallel.
- SIGINT during a slow stub response raises `KeyboardInterrupt`.
- Sync by default, with `thinkthen.aio` beside it under the same names and order.
- A record verb crosses the barrier once per chunk.
- `py.typed` and stubs ship, with `Outcome` an `Enum` and options a `Literal`.
- A verb takes any iterable. A pandas or polars column works, and the library imports neither.

## Anti-goals

- No pydantic, httpx, requests, or vendor SDK. Each costs import time and a resolver conflict.
- No `__bool__` that raises under a single cut. A cut has two outcomes, and refusing the boolean throws away Python's best line.
- No dict tree and no JSON text built in Python for the bare answer. The engine already holds the value.
- No thread pool in Python. The lock serializes it, and the engine already runs requests at the width `jobs` names.
- No pure-Python fallback. It drifts, and the digest must match the other surfaces.

## Where this language wastes time

- **The global interpreter lock.** Held across a network wait, it stops every other thread. Fix: release the lock around each engine call, through `Python::detach` in current PyO3, and retake it only to build the answer.
- **A call per Python object.** A list or any iterable crosses as objects, and conversion gets paid a thousand times. Fix: pull a chunk, cross once, yield lazily. PyO3's `to_str` borrows CPython's UTF-8 buffer, so a `str` needs no copy in. Whether an abi3 build below Python 3.10 falls back to a copying path is unchecked.
- **A column turned back into Python objects.** The Arrow PyCapsule protocol needs nothing installed, so `__arrow_c_array__` and `__arrow_c_stream__` hand a Polars or an Arrow-backed pandas column to Rust with no import of pyarrow. A `String` array is offsets over one contiguous UTF-8 buffer, so Rust reads every value as `&str` with no per-item object. The `arrow` crate's `pyarrow` feature and `pyo3-arrow` both read the capsules. The stream form arrives one record batch at a time, so a column longer than memory still runs. Polars exposes the stream form and not the array form. NumPy is the weak one: a `U` array is fixed-width UTF-32 and costs a conversion, and an object array crosses like a list.
- **A thread per async call.** The easy bridge parks a thread on each Rust future. Fix: `pyo3-async-runtimes`, where the Tokio waker completes the future through `call_soon_threadsafe`.
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, a dictionary-encoded column costs one judgment per distinct value, and a cached answer costs nothing. No `set` and no `unique` in Python. `jobs` is one number for the process, and threads, `aio` tasks, and two callers feed one scheduler. A called question in a loop is serial, and `tt.filter(q, records)` is the bulk form.
- **Fork with a live pool.** `multiprocessing` forks by default, and the child inherits the Rust pool's sockets. Fix: an `os.register_at_fork` hook drops the pool.

## How little code

**PyO3, built and shipped with maturin.** PyO3 builds against CPython's stable ABI, and maturin builds the wheels and the sdist from one manifest.

The shim holds argument conversion, the command's option words as keywords, `Outcome` and the `Literal` aliases, chunking, failure-to-exception mapping, the `replaying` scope, the `aio` module, the fork hook, and the stubs. It holds no threshold math, no JSON, no retry, no digest, no recording format, and no HTTP.

Wheels are abi3. One wheel per platform serves many Python versions. Prebuilt: manylinux and musllinux on x86_64 and aarch64, macOS on arm64 and x86_64, Windows on x86_64, plus an sdist. Whether abi3 covers the free-threaded build is unchecked.

## Tests only this surface needs

- Many threads against a stub backend finish near the time of one call.
- SIGINT during a blocking call raises `KeyboardInterrupt` in the calling thread.
- The import loads no third-party module and meets the budget.
- A thousand records through `filter` cross the barrier once per chunk.
- A forked child does not reuse the parent's pooled connection.
- The stubs pass mypy and pyright strict.
- A Polars column reaches Rust through `__arrow_c_stream__` with pyarrow absent.
- A bench counts rows a second through `filter` over a list and over an Arrow column against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.

## Open questions for the ADR

1. Which abi3 minimum does the wheel target, and does the free-threaded build get its own wheel?
2. Does `aio` restate every verb, or does one generated wrapper cover them all?
3. Does chunk size become a keyword beside `jobs`, or does the engine choose it?
4. After a fork, does the child rebuild the pool silently, or does its first call raise?
5. Does the Arrow path ship in the first release? It puts the `arrow` crate or `pyo3-arrow` in the wheel, and lines and build size are the cost. The no-dependency budget survives either way, because the capsule protocol needs nothing installed in Python.
6. Which Arrow string layouts does the first release accept, and does an Arrow column come back as an Arrow array or as a list?
