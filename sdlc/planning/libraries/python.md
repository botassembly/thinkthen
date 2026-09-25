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

## As built (ticket 0105)

The binding is `libraries/python`, the crate `thinkthen-python` over the public API (ADR 0047). `libraries/python/NOTES.md` records the port's decisions. The module functions read the environment's engine, and `tt.Engine` holds the ADR 0017 section 5 settings.

```python
import thinkthen as tt

tt.decide("Is this a complaint?", text)
refund = tt.question(decide="Does the customer ask for a refund?", threshold=(0.2, 0.8))
tt.decide_many(refund, texts)

engine = tt.Engine(throttle=8, cache=False)
engine.decide_many(refund, texts)
```

Where the build differs from the goals below:

- Ctrl-C comes from a worker thread, not the engine's poll. Every call runs on a detachable worker, and the calling thread checks signals and the caller's token every 50 ms. A stop raises `Cancelled` within one tick for a single send and for a batch. The tests hold the reply on the backend's held arm and measure under 100 ms.
- `details` is its own function, and it returns the command's `--details` document as a `dict`. No verb takes `details=True`.
- `thinkthen.aio` is not built. Ticket 0105 excludes async forms.
- A pandas object is refused (ADR 0017, 2026-09-21). A Polars or Arrow column is refused until ticket 0106 opens the Arrow door.
- The wheel claims Python 3.10 through `abi3-py310`. The gate runs on 3.12 or later, because the test pins need it. Release wheels own a run on each claimed Python.

## Goals

- One compiled module, and nothing heavy imported beside it.
- The lock is released for the whole time Rust waits. Threads and record verbs then run in parallel. Experiment 211 measured 100 Python threads calling at once holding the process gate at 32 in flight.
- SIGINT during a batch raises `KeyboardInterrupt` in about a fifth of a second, with nothing served after the return. The engine runs a poll callback on the calling thread every 50–100 ms (ADR 0017 section 2); the closure re-takes the interpreter lock and checks signals. Measured 0.207–0.209 s after the signal, stub frozen at 32 requests (211). The 205 finding this retires: a whole-list crossing was deaf 8.07 s, a chunked one stopped in 0.23 s.
- Sync by default. `thinkthen.aio` may restate the verbs, but the engine is blocking (ADR 0017 section 2), so `aio` is a wrapper over a call that already releases the lock, never a second runtime. The 205 finding that forced `pyo3-async-runtimes` retires with the engine's runtime.
- A record verb crosses the barrier once. Chunking is a memory lever, not a responsiveness lever: the interrupt comes from the poll, not the chunk.
- A band answers with `None` for unsure beside `True` and `False` (ADR 0017 pick 3), and the band example leads with the `is None` check.
- Every verb takes `details=True`; the details object is load-bearing for conformance, carrying the model and the digest.
- The first slot takes plain question text or a question value; a leading `{` means question-file JSON, and plain text wraps as `{"decide": text}`.
- `py.typed` and stubs ship, with options a `Literal`.
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
- **A thread per async call.** The easy bridge parks a thread on each Rust future. PyO3 0.29 names the floor: the async bridge is the built-in `experimental-async` coroutine, `Python::detach` (was `allow_threads`) and `Python::attach` (was `with_gil`). `pyo3-async-runtimes` is deleted; it is superseded and its `future_into_py` shape no longer matches (205).
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, a dictionary-encoded column costs one judgment per distinct value, and a cached answer costs nothing. No `set` and no `unique` in Python. `jobs` is one number for the process, and threads, `aio` tasks, and two callers feed one scheduler. A called question in a loop is serial, and `tt.filter(q, records)` is the bulk form.
- **Fork with a live pool.** `multiprocessing` forks by default. The engine stamps its process ID and rebuilds the pool and the gate on a mismatch, taking no inherited lock, so a forked child answers on its first call (ADR 0017 section 2; 211 measured a child answering in about 353 ms under `fork`, and ten forks during live 32-wide batches all answering). An `os.register_at_fork` hook is not needed and cannot reach engine state anyway.

## How little code

**PyO3, built and shipped with maturin.** PyO3 builds against CPython's stable ABI, and maturin builds the wheels and the sdist from one manifest.

The shim holds argument conversion, the command's option words as keywords, the empty-value mapping for unsure, `decide_many`, chunking, failure-to-exception mapping with `retryable` on every error and the deadline kind its own exception, the `replaying` scope, the `aio` wrapper, and the stubs. It holds no threshold math, no JSON, no retry, no digest, no recording format, and no HTTP. The line ceiling is about 400 code lines; 330 was the honest floor for errors, laziness, `aio`, and details without the Arrow door (205).

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

1. Which abi3 minimum does the wheel target, and does the free-threaded build get its own wheel? The poll shape assumes CPython's signal check on the main thread; a `cp313t` wheel is a CI decision, unchecked.
2. Does `aio` restate every verb, or does one generated wrapper cover them all?
3. Answered by ADR 0017: no chunk for the sake of Ctrl-C. The poll carries the interrupt, and chunking stays a memory lever.
4. Answered by experiment 211: the child rebuilds silently and answers. Ten forks during live batches, every child answering in about 353 ms.
5. Answered by measurement (205, round two): the Arrow door ships hand-rolled over the PyCapsule protocol, +15.4 KB with no new crates, buffer-address equality proven, `pyarrow` never loaded at import (0.58–0.66 ms unchanged). The `arrow`-crate route would cost +139 KB and 169 more crates for a saving inside noise.
6. Which Arrow string layouts does the first release accept, and does an Arrow column come back as an Arrow array or as a list?
7. Should `UsageError` also subclass `ValueError` for ecosystem catchability? It costs a few lines under `create_exception!`'s single parent (205).
