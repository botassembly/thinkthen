# ThinkThen for Python

Install Python 3.10 or later and run `pip install thinkthen`. The wheel includes the native engine.

The example uses the stub backend and needs no key or service.

```python
import thinkthen

with thinkthen.Client(backend="stub") as client:
    row = client.decide(
        "Does the customer ask for money back?",
        "Please refund my order.",
        threshold=(0.2, 0.8),
    )
    print(row.status, row.answer)
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/python/thinkthen/__init__.py).
