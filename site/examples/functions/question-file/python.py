import thinkthen as tt

refund = tt.QuestionSource(path="refund.json")
money_back = (
    "I would like to return this and get my money back."
    "\n"
)
is_refund = tt.decide(refund, money_back).value
assert is_refund == "The customer asks for money back."
