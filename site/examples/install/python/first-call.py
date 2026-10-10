import thinkthen as tt

question = "Does the customer ask for a refund?"

broken = "Please refund my order. It arrived broken."
is_refund = tt.decide(question, broken).value
assert is_refund is True

refund = {"decide": question, "threshold": "0.2:0.8"}
send_back = "I want to send this back."
is_refund = tt.decide(refund, send_back).value
assert is_refund is None

reports = [
    (
        "CSV export fails every time. "
        "Steps: open a report,\n"
        "click Export, pick CSV. "
        "My month-end numbers are stuck.\n"
    ),
    (
        "Steps: open the login page, enter a password, "
        "press Enter. The page spins and "
        "nobody can sign in."
    ),
    (
        "The Pay button on the billing page is a slightly "
        "different blue. No steps, I just noticed it."
    ),
]
triage = tt.annotate(
    tt.QuestionSource(path="form.json"), reports,
).value
assert triage == [
    {"steps": True, "area": "export", "impact": 1.99},
    {"steps": True, "area": "login", "impact": 2.0},
    {"steps": False, "area": "billing", "impact": 0.01},
]
