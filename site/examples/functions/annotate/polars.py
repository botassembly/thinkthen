import polars as pl
import thinkthen as tt

reports = pl.DataFrame({
    "body": [
        "Steps: click Export. It is very slow.",
        "Steps: click Log in. Nobody gets in.",
        "The Pay button on billing is too blue.",
    ],
})
triage = tt.annotate("form.json", reports, on="body").value
print(triage.select("steps", "area", "impact"))
