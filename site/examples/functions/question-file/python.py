import thinkthen as tt

refund = tt.question(file="refund.json")
messages = [
    "I would like to return this and get my money back.",
    "I want to send this back.",
]
is_refund = [
    tt.decide(refund, message + "\n").value
    for message in messages
]
assert is_refund == [True, None]
