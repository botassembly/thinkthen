# ThinkThen for Python

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
pip install thinkthen
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```python
import thinkthen as tt
refunds = ["Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!",
           "I want to send this back."]
messages = ["Please refund the extra fee on my invoice.", "My parcel went to the wrong address.",
            "I cannot reset my password."]
teams = {"billing": "Invoices, fees, and refunds.", "shipping": "Parcels and delivery.",
         "account": "Logins and passwords."}
with tt.Engine() as engine:
    for text in refunds:
        refund = engine.decide("Does the customer ask for a refund?", text).value
        print(refund)
    for text in messages:
        team = engine.choose("Which team owns this?", text, options=teams).value
        print(team)
```

```text
True
False
True
billing
shipping
account
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
