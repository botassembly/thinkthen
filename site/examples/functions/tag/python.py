import thinkthen as tt

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
message = (
    "Love the new dashboard, but export crashes the app,\n"
    "and I was charged twice.\n"
)
fitting_labels = tt.tag(
    question, message, labels=labels
).value
assert fitting_labels == ["praise", "bug", "billing"]
