import thinkthen as tt

question = "Does the customer ask for a refund?"

broken = "Please refund my order. It arrived broken."
assert tt.decide(question, broken) is True

refund = tt.question(decide=question, threshold=(0.2, 0.8))
send_back = "I want to send this back."
assert tt.decide(refund, send_back) is None

reports = [
    "CSV export fails. Steps: click Export.",
    "The login page spins and nobody can sign in.",
    "The Pay button on the billing page is too blue.",
]
assert tt.annotate("form.json", reports) == [
    {"steps": True, "area": "export", "impact": 1.94},
    {"steps": False, "area": "login", "impact": 2.0},
    {"steps": False, "area": "billing", "impact": 0.06},
]
