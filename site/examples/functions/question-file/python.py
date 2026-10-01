import thinkthen as tt

refund = tt.question(file="refund.json")
money_back = (
    "I would like to return this and get my money back."
    "\n"
)
is_refund = tt.decide(refund, money_back).value
assert is_refund is True
