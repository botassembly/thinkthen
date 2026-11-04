import thinkthen as tt

retry = tt.RetryPolicy(max_retries=5)
with tt.Client(retry=retry) as client:
    is_refund = client.decide(
        "Does the customer request a refund?",
        "Please refund my order.",
    )
print(is_refund)
