import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
refund_requests = list(
    tt.filter("Does the customer request a refund?", messages)
)
for refund_request in refund_requests:
    print(refund_request)
