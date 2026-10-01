import thinkthen as tt

reports = [
    "Steps: click Export. It is very slow.",
    "Steps: click Log in. Nobody gets in.",
    "The Pay button on billing is too blue.",
]
triage = tt.annotate("form.json", reports).value
assert triage == [
    {"steps": True, "area": "export", "impact": 1.04},
    {"steps": True, "area": "login", "impact": 1.98},
    {"steps": False, "area": "billing", "impact": 0.09},
]
