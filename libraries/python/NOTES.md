# Python binding notes

Ticket 0105 ported the Python surface from tag `surfaces-wave7-frozen-2026-09-24b` onto the public `thinkthen` API. The tag's own `NOTES.md` stays at the tag as history. This file records what the port decided and why. Ian can overturn each point.

## Shape

- `src/lib.rs` holds the module edge: the six exception classes, the one table from `ErrorKind` to a class, and the one `catch_unwind` site. `Cancelled` has two parents, `KeyboardInterrupt` and `ThinkThenError`. `create_exception!` gives a class one parent, so `Cancelled` is built once with Python's `type()`.
- `src/worker.rs` runs every call on a spawned worker (decision 6, amendment change 2). The worker owns its inputs and never touches Python. The calling thread releases the interpreter and waits in 50 ms ticks. Each tick reads the caller's token, then runs the signal handlers. On a stop it cancels the worker's own token and leaves the worker behind. `mpsc::Receiver` is not `Sync`, so the tick moves the receiver into the detached closure and back out (spike 255).
- `src/input.rs` reads every argument into owned Rust values before the first send: texts, entities, the deadline, and the token.
- `src/asked.rs` holds the question values and what `recognize` and `relate` return.
- `src/engine.rs` holds `_Engine`. Four methods serve the verbs by shape: `ask` for one text, `many` for `decide_many` and `filter`, `order` for `rank` and `find`, and one each for `annotate`, `recognize`, and `relate`. The package names the verb. This kept production Rust under its 1,200-line budget.
- `thinkthen/__init__.py` composes question-file JSON from keywords, so parts and files make one question (decision 8). It holds `tt.Engine` and the module functions, which call the same methods on one lazy value over `default_engine()`.

## Decisions made while porting

- One text verb reads `details`. `decide`, `choose`, `score`, and `tag` each read the value of one `details` call. A question with runtime labels needs no typed `Choice`, and no send is added (decision 8, G5).
- A verb takes a question text too. `tt.decide("Is this a complaint?", text)` builds `{"decide": text}`, as the tag did. Beside a text, `choose` takes `options`, `score` takes `levels`, and `tag` takes `labels`. Beside a built question, those keywords raise `TypeError`, since they would be dropped.
- `rank` and `find` take the question text, since their questions come from `Question::rank` and `Question::find`. Each wraps a record in one `Indexed` evidence type, so the answer carries the input's place. `find` returns `None` when nothing is selected.
- `recognize` and `relate` take keywords or a spec. `kinds` lists kind words or maps each to a description. `relations` maps a name to a `(source, target)` pair, and `either` names the rules that read both ways. A second positional argument is a spec file path or the file's `dict`.
- `details` returns the command's `--details` document as a `dict`.
- `tt.Engine` checks `throttle` itself, so `throttle=300` is a `UsageError` and never an `OverflowError` (amendment change 13). `cache=True` names the default folder, `cache=False` means no cache, and any other value is a folder path.
- `max_requests` refuses `rank` and `find` before any send. A streaming call sends the records under the limit first, as `EngineBuilder::max_requests` documents. The engine test uses `rank` for the zero-send case.
- Each verb makes exactly one engine call, so `deadline` and `max_requests` apply to the whole call. A future verb that loops engine calls over several texts must resolve the deadline once into an instant and pass `deadline_at` to each inner call.
- The Rust unit tests no longer need one thread. Only one test starts a worker, so the live count is its own.

## The Polars door (ticket 0106)

Ticket 0106 ported the tag's `arrow.rs` into `src/arrow/` and the column and frame glue into `src/frame.rs`. Ian can overturn each point.

- A column verb makes one engine call. `decide` and `decide_many` call `decide_many_with` once. `choose`, `score`, and `tag` call `annotate_with` once over a one-question set. A Polars `Series` gets a `Series` back through its own class, and any other Arrow column gets a list.
- Frames are Polars only. `annotate(set, frame, on=)` calls `annotate_with` once and returns the frame with one column per question. The frame's own columns go back as aliases of the caller's batches, never copies.
- `recognize(frame, on=)` calls `recognize_with` once per text in Rust. The deadline is resolved once, at call start, into an instant, and each inner call gets `deadline_at` of it. `max_requests` caps each inner call, not the loop.
- The worker owns the imported batches and reads the strings in place (amendment change 3). It releases them attached to the interpreter, behind the exit gate in `src/arrow/gate.rs` (change 6). A release that finds exit under way leaks the batches on purpose, since a release then could freeze the process. The binding's own output keeps per-batch atomic shares, so any thread may release it.
- The policy check admits `unsafe` only in files named `ffi.rs`. So the door's raw memory code sits in three such files: `src/arrow/ffi.rs` reads a producer, `src/arrow/out/ffi.rs` hands answers back, and `src/arrow/probe/ffi.rs` holds the probe build's lock-bound producer. The ticket had named one file per concern. The other five files hold no `unsafe`.
- The tag's pandas probe and pandas advice are gone (the 2026-09-21 ruling). R3-19 retires with them.
- Fork advice from experiment 228 stands: a warm Polars pool hangs a forked child, so start children with `spawn`, or fork before Polars runs.

### Refused string-view shapes (R7-8)

Each line names the test that proves it. `check.sh` fails when a named test does not exist.

- A table of fewer than three buffers, a null views buffer, or a null sizes buffer: proved by `null_tables_and_row_claims_are_refused`.
- A view that names a data buffer past the table, the sizes buffer included: proved by `a_view_cannot_name_the_sizes_buffer`.
- A view that runs past the size its data buffer declares: proved by `a_view_past_its_declared_length_is_refused`.
- A declared size that is negative or past the 1 GiB cap on one data buffer: proved by `a_declared_data_extent_past_the_cap_is_refused_and_a_big_slice_borrows`.
- A sizes buffer shorter than its count, against unreadable memory: proved by `a_sizes_buffer_shorter_than_its_count_is_refused`.
- A view whose bytes run past readable memory, though its declared size allows them: proved by `test_extents_past_readable_memory_are_refused`.
- A buffer or a view read past the end of a mapped file: proved by `a_file_mapping_past_the_end_of_its_file_is_unreadable`.

### Waivers kept

- An extent that runs into another readable allocation reads that memory. The readable-memory check sees mapped pages, not allocations. Lever: a producer that lies about its sizes.
- Off Linux, `mincore` sees only unmapped pages, so a guard page reads as readable.
- A producer that unmaps its memory during a call stays a recorded risk. The snapshot is taken once per call.

## Tests

Each engine call runs in a child Python with its own loopback backend and cache folder (amendment change 5). `tests/conftest.py` builds the child's environment: it copies the parent's, deletes `THINKTHEN_API_KEY`, then sets a fake key beside the loopback address (change 14). A pytest session check fails when the parent holds a key.

- `test_surface.py`: the registration check, the question builder, the result shapes of both spellings, the relate input forms, and the fork test.
- `test_inputs.py`: the pandas and Arrow refusals, whole-list reading, and the deadline rule.
- `test_door.py`: column and list parity, the door's refusals with zero sends, frame round trips, recognize on a frame, `polars` left unimported, and the slide sample.
- `test_arrow_safety.py`: malformed columns against unreadable memory, releases on refusal, the door's output and moved children, and the address proof.
- `test_release.py`: attached release after the caller leaves, and the exit-freeze test of change 6. `arrow_c.py` holds the hand-built producers.
- `test_column_timing.py`: one deadline and one token per column, throttle equality, the in-flight count, and the recognize loop's one deadline.
- `test_stopping.py`: the caller's token, Ctrl-C on single, batch, and Polars column calls, and a handler's `SystemExit`.
- `test_engine.py`: the `tt.Engine` settings.
- `test_secrecy.py`: the child environment, the fake key at a loopback listener, and every message and `repr`.
- `conformance.py` runs every case in `conformance/cases.json`, and each typed, `decide_many`, and `annotate` case again over a Polars column or frame. It reports pass, fail, or not run with a reason, and the three counts sum to the file's count.
- `examples.py` runs `examples.json` against the generic arm.

## Gate

`check.sh` needs Python 3.12 or later, `uv`, and `maturin`, and exits 77 when one is missing. Its venv lives at `~/.cache/thinkthen-toolchains/python/<hash of the checkout path>/`, installed offline from uv's cache with hashes required. `build-wheel.sh` builds the release wheel without the `probe` feature and checks its contents.
