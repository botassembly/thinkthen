import polars as pl
import thinkthen as tt

reports = pl.DataFrame({
    "body": [
        (
            "Export crashes. "
            "Steps: open a report, click Export."
        ),
        "The login page spins and nobody can sign in.",
        "The Pay button on the billing page is too blue.",
    ],
})
triage = tt.annotate("form.json", reports, on="body").value
assert triage.drop("body").rows() == [
    (True, "export", 1.99),
    (False, "login", 2.0),
    (False, "billing", 0.07),
]
