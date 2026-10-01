import pandas as pd
import thinkthen as tt

reports = pd.DataFrame({
    "body": [
        "Steps: click Export. It is very slow.",
        "Steps: click Log in. Nobody gets in.",
        "The Pay button on billing is too blue.",
    ],
})
triage = tt.annotate("form.json", reports, on="body").value
print(triage[["steps", "area", "impact"]])
