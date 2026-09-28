import thinkthen as tt

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
message = (
    "Love the new dashboard, "
    "but export crashes the app, "
    "and I was charged twice."
)
fitting_labels = tt.tag(question, message, labels=labels)
assert fitting_labels == ["praise", "bug", "billing"]
