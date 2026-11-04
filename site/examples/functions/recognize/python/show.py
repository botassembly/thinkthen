import thinkthen as tt

names = tt.recognize(
    "Morgan works for Example Group.",
    kinds=["person", "organization"],
)
for name in names:
    print(name.kind, name.text)
