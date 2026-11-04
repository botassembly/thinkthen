import thinkthen as tt

topics = tt.tag(
    "Which topics appear?",
    "I was charged twice and my parcel is late again.",
    labels=["billing", "shipping"],
)
print(list(topics))
