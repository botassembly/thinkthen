import polars as pl
import thinkthen as tt

reports = pl.DataFrame({
    "body": [
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
            "The Pay button on the billing page "
            "is a slightly "
            "different blue. No steps, I just noticed it."
        ),
    ],
})
triage = tt.annotate("form.json", reports, on="body").value
print(triage.select("steps", "area", "impact"))
