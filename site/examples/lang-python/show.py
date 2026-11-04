import thinkthen

refund = thinkthen.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
    threshold=(0.2, 0.8),
)
print(refund.status, refund.answer)
