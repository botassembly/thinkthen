# The Python surface

`import thinkthen as tt` calls the real engine through the public `thinkthen` crate. The binding is the unpublished crate `thinkthen-python` in its own Cargo workspace (ADR 0047). The design page is `sdlc/planning/libraries/python.md`.

```python
import thinkthen as tt

tt.decide("Does the customer ask for a refund?", text)  # True, False, or None
refund = tt.question(decide="Does the customer ask for a refund?", threshold=(0.2, 0.8))
tt.decide(refund, "I was charged twice. Can you fix this?")
complaints = tt.filter("Is this a complaint?", reviews)
rows = tt.annotate("form.json", reviews)  # one dict per review

engine = tt.Engine(throttle=8, cache=False)
engine.decide_many(refund, reviews)
```

`None` means "not sure". A call reads one `str`, or a list, tuple, or other iterable of `str`, whole before its first request. A pandas object is refused, since the Python data frame is Polars. A Polars or Arrow column is refused until ticket 0106 opens that door. `examples.json` holds one checked example for each function.

The module functions use one engine that the environment configures: `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`, and the rest. `tt.Engine` takes `base_url`, `model`, `throttle`, `max_requests`, `cache`, and `cache_bytes` as keywords and reads the environment for the rest. The key comes only from `THINKTHEN_API_KEY`. The throttle is the number of requests in flight. It is one per loaded copy of this package: the first explicit throttle sets it, and a second, different one raises `UsageError`.

Every verb takes `deadline`, in seconds from the call, and `token`, a `CancelToken` any thread can set. No deadline is spelled ``deadline=None`` or ``deadline=-1`` (ADR 0041). Zero is a spent deadline: the call sends nothing and raises `DeadlineError`. Any other negative, a bool, and a non-number raise `UsageError`.

Every call runs on its own worker thread. Ctrl-C or the caller's token stops the wait within 50 ms and raises `Cancelled`, a subclass of both `KeyboardInterrupt` and `ThinkThenError`. No new request starts after a stop, and a request already sent ends on its own. A signal handler's own error, such as `SystemExit`, passes through unchanged.

`check.sh` is this folder's gate. It needs Python 3.12 or later, `uv`, and `maturin`, and it installs the pinned test packages offline from uv's cache. A missing piece reports "not run". `build-wheel.sh` builds the release wheel and checks its contents. `NOTES.md` records the port's decisions.
