import thinkthen as tt

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
is_refund = tt.decide(question, broken).value
assert is_refund is True
