import thinkthen as tt

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
message = (
    "Love the new dashboard, "
    "but export crashes the app, "
    "and I was charged twice."
)
tags = tt.tag(question, message, labels=labels)
assert tags == ["praise", "bug", "billing"]
