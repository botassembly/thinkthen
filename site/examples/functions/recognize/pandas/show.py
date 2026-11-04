import pandas as pd
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame({"message": ["Morgan works for Example Group."]})
names = messages["message"].thinkthen.recognize(
    kinds=["person", "organization"]
)
for name in names.explode():
    print(name.kind, name.text)
