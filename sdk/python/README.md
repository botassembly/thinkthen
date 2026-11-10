# Python SDK

The Python SDK asks questions about text from your Python code. Its first function, decide, answers a yes-or-no question with yes (`True`), no (`False`) or not sure.

Install it with Python 3.10 or later:

```sh
pip install thinkthen
```

The calls read your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```python
import thinkthen as tt

is_refund = tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
)
print(is_refund)
```

Output:

```text
True
```

`True` means yes: the message asks for a refund. `is_refund` stands for its answer, so `if is_refund:` works.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. Using a not-sure answer raises `tt.Unsure`, and a message ThinkThen could not judge raises `tt.RecordError`, so neither one passes for no. Send both to a person.

```python
import thinkthen as tt

is_refund = tt.decide(
    "Does the customer request a refund?",
    "I was charged twice for one invoice.",
    threshold=(0.2, 0.8),
)
try:
    if is_refund:
        print("Send to refunds.")
    else:
        print("Leave in the inbox.")
except tt.Unsure:
    print("Not sure. Ask a person.")
```

Output:

```text
Not sure. Ask a person.
```

## Next

The [Python SDK page](https://thinkthen.dev/sdks/python/) shows all ten functions, with pandas and Polars, async calls with `thinkthen.aio`, and when to make a client of your own. The [Python guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/python.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/python/thinkthen/__init__.py) cover every option.

### Inspect client usage

```python
from thinkthen import Client

with Client(backend="stub") as client:
    is_ready = client.decide("Ready?", "alpha")
print("Ready" if is_ready else "Not ready")
snapshot = client.usage
assert snapshot.attempts == 0
```

`client.usage` (also on `AsyncClient`) is an immutable cumulative snapshot for that explicit client, available after close. Input and output tokens each carry a reported sum and a count of finished attempts whose usage is unknown. Active attempts and calls remain separate. This is not a price or invoice total; use a separate client for isolated accounting.

## Request allowance

```python
from thinkthen import Client

client = Client(max_requests=100)
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

### Local response storage

`Client(response_store=tt.ResponseStore(path, "record", namespace, model_generation))` explicitly enables local recording; use `"cache"` for eligible reuse or `"replay"` for offline playback without credentials. `AsyncClient` accepts the same option. Optional `max_bytes`, `max_entries`, `max_age` (seconds), and `busy_timeout_ms` use engine defaults when omitted. `client.settings.response_store` returns admitted values. Usage snapshots include `cache_hits` and `replay_hits`.

Storage is disabled unless configured. Local recordings retain question and record text; use a private local path and a nonsecret namespace/model-generation label. The shared engine owns limits, identity and concurrency policy. Replay misses fail locally rather than making a model call.
