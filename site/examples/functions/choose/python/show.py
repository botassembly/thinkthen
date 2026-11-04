import thinkthen as tt

topic = tt.choose(
    "What is the message about?",
    "Please refund my order.",
    choices=["billing", "shipping", "other"],
)
print(topic)
