import polars as pl
import thinkthen as tt

question = "Which labels fit this message?"
labels = ["praise", "bug", "billing"]
messages = pl.DataFrame({
    "body": [
        (
            "Love the new dashboard, but export "
            "crashes the app,\n"
            "and I was charged twice.\n"
        ),
    ],
})
fitting_labels = tt.tag(
    question, messages["body"], labels=labels
).value
messages = messages.with_columns(labels=fitting_labels)
print(messages.select("labels"))
