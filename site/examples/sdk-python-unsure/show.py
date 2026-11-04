import thinkthen as tt

is_refund = tt.decide(
    "Does the customer request a refund?",
    "I was charged twice for one invoice.",
    threshold=(0.2, 0.8),
)
try:
    if is_refund:
        print("Send to refunds.")
    else:
        print("Leave in the inbox.")
except tt.Unsure:
    print("Not sure. Ask a person.")
