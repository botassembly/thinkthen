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
