# ThinkThen for python

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
pip install thinkthen
```

## Example

Configure a model backend before running this example.

```python
import thinkthen as tt

with tt.Engine(cache=False) as engine:
    result = engine.decide("Does this ask for a refund?", "Please refund my order.")
    print(result.value)
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
