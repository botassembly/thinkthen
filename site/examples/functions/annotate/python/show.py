import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
questions = {
    "refund": tt.DecideQuestion("Does the customer request a refund?"),
    "topic": tt.ChooseQuestion(
        "What is the message about?",
        choices=["billing", "shipping", "other"],
    ),
    "urgency": tt.ScoreQuestion(
        "How urgent is the request?",
        levels=["routine", "urgent"],
    ),
}
forms = tt.annotate_stream(questions, messages).to_list()
for form in forms:
    entries = form.raise_if_failed().answer
    print(entries["refund"], entries["topic"], entries["urgency"])
