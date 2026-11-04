import thinkthen as tt

urgency = tt.score(
    "How urgent is the request?",
    "Please refund my order.",
    levels=["routine", "urgent"],
)
print(urgency)
