import thinkthen as tt

is_refund = tt.decide(
    "Does the customer request a refund?",
    "Please refund my order.",
)
print(is_refund)
