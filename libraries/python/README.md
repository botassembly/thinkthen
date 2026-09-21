# The Python surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the shim:

```python
import thinkthen as tt

tt.decide("Does the customer ask for a refund?", text)  # True
refund = tt.question(decide="...", threshold=(0.2, 0.8))
tt.decide(refund, "I was charged twice. Can you fix this?")  # None
complaints = tt.filter("Is this a complaint?", reviews)
df = tt.annotate("form.json", df, on="body")
```

`None` is "not sure". A list or a data frame column crosses once. The bulk
spelling is `decide_many`. The shim binds `thinkthen-contract` and never the
engine beneath it.
