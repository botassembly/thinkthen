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

- **The global interpreter lock.** Held across a network wait, it stops every other thread. Fix: wrap each engine call in `Python::allow_threads` and retake the lock only to build the answer.
- **Copying `str` and `bytes` across the barrier.** Fix: borrow the Python buffer, and build each returned string once.
- **A call per record.** Conversion and error setup get paid a thousand times. Fix: pull a chunk, cross once, yield lazily.
- **A thread per async call.** The easy bridge parks a thread on each Rust future. Fix: `pyo3-async-runtimes`, where the Tokio waker completes the future through `call_soon_threadsafe`.
- **Fork with a live pool.** `multiprocessing` forks by default, and the child inherits the Rust pool's sockets. Fix: an `os.register_at_fork` hook drops the pool.

## How little code

**PyO3, built and shipped with maturin.** PyO3 builds against CPython's stable ABI, and maturin builds the wheels and the sdist from one manifest.

The shim holds argument conversion, the command's option words as keywords, `Outcome` and the `Literal` aliases, chunking, failure-to-exception mapping, the `replaying` scope, the `aio` module, the fork hook, and the stubs. It holds no threshold math, no JSON, no retry, no digest, no recording format, and no HTTP.

Wheels are abi3. One wheel per platform serves many Python versions, and no user needs a Rust toolchain. Prebuilt: manylinux and musllinux on x86_64 and aarch64, macOS on arm64 and x86_64, Windows on x86_64, plus an sdist. Whether abi3 covers the free-threaded build is unchecked.

## Tests only this surface needs

- Many threads against a stub backend finish near the time of one call.
- SIGINT during a blocking call raises `KeyboardInterrupt` in the calling thread.
- The import loads no third-party module and meets the budget.
- A thousand records through `filter` cross the barrier once per chunk.
- A forked child does not reuse the parent's pooled connection.
- The stubs pass mypy and pyright strict.

## Open questions for the ADR

1. Which abi3 minimum does the wheel target, and does the free-threaded build get its own wheel?
2. Does `aio` restate every verb, or does one generated wrapper cover them all?
3. Does chunk size become a keyword beside `jobs`, or does the engine choose it?
4. After a fork, does the child rebuild the pool silently, or does its first call raise?
