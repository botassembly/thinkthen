# ThinkThen for Python

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

```sh
pip install thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```python
import thinkthen as tt

with tt.Engine() as engine:
    result = engine.decide("Does this ask for a refund?", "Please refund my order.")
    print(result.value)
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
