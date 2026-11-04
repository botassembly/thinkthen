import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {"message": ["I was charged twice and my parcel is late again."]}
)
messages["topics"] = messages["message"].thinkthen.tag(
    "Which topics appear?",
    labels=["billing", "shipping"],
)
print(messages[["topics"]])
