import pandas as pd
import thinkthen as tt

reports = pd.DataFrame({
    "body": ["Steps: click Log in. Nobody gets in."],
})
triage = tt.annotate("form.json", reports, on="body").value
assert triage["steps"].tolist() == [True]
assert triage["area"].tolist() == ["login"]
assert triage["impact"].tolist() == [1.98]
