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

## Tests

Each engine call runs in a child Python with its own loopback backend and cache folder (amendment change 5). `tests/conftest.py` builds the child's environment: it copies the parent's, deletes `THINKTHEN_API_KEY`, then sets a fake key beside the loopback address (change 14). A pytest session check fails when the parent holds a key.

- `test_surface.py`: the registration check, the question builder, the result shapes of both spellings, the relate input forms, and the fork test.
- `test_inputs.py`: the pandas and Arrow refusals, whole-list reading, and the deadline rule.
- `test_stopping.py`: the caller's token, Ctrl-C on single and batch calls, and a handler's `SystemExit`.
- `test_engine.py`: the `tt.Engine` settings.
- `test_secrecy.py`: the child environment, the fake key at a loopback listener, and every message and `repr`.
- `conformance.py` runs every case in `conformance/cases.json`. It reports pass, fail, or not run with a reason, and the three counts sum to the file's count.
- `examples.py` runs `examples.json` against the generic arm.

## Gate

`check.sh` needs Python 3.12 or later, `uv`, and `maturin`, and exits 77 when one is missing. Its venv lives at `~/.cache/thinkthen-toolchains/python/<hash of the checkout path>/`, installed offline from uv's cache with hashes required. `build-wheel.sh` builds the release wheel without the `probe` feature and checks its contents.
