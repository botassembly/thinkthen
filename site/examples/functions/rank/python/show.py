import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
by_urgency = tt.rank(
    "How urgent is the request?",
    messages,
    levels=["routine", "urgent"],
).to_list()
for rank in by_urgency:
    print(rank, messages[rank.position - 1])
