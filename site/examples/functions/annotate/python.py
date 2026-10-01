import thinkthen as tt

report = "Steps: click Log in. Nobody gets in."
triage = tt.annotate("form.json", [report]).value
assert triage == [
    {"steps": True, "area": "login", "impact": 1.98},
]
