import polars as pl
import thinkthen as tt

reports = pl.DataFrame({
    "body": ["Steps: click Log in. Nobody gets in."],
})
triage = tt.annotate("form.json", reports, on="body").value
assert triage["steps"].to_list() == [True]
assert triage["area"].to_list() == ["login"]
assert triage["impact"].to_list() == [1.98]
