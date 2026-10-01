import thinkthen as tt

question = "Does the customer ask for a refund?"
broken = "Please refund my order. It arrived broken."
thanks = "Thanks for the quick help yesterday!"
broken_is_refund = tt.decide(question, broken).value
thanks_is_refund = tt.decide(question, thanks).value
assert broken_is_refund is True
assert thanks_is_refund is False
