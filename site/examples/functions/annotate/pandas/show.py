import pandas as pd
import thinkthen as tt
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {
        "message": [
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.",
        ]
    }
)
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
forms = messages["message"].thinkthen.annotate(questions)
print(messages.join(forms))
