import pandas as pd
import thinkthen as tt
import thinkthen.pandas  # registers the .thinkthen accessor

questions = tt.AnnotationQuestions({
    "refund": tt.DecideQuestion("Does the customer request a refund?"),
    "topic": tt.ChooseQuestion(
        "What is the message about?", choices=["billing", "shipping", "other"]
    ),
    "urgency": tt.ScoreQuestion(
        "How urgent is the request?", levels=["routine", "urgent"]
    ),
})
messages = pd.read_csv("messages.csv")
forms = messages["text"].thinkthen.annotate(questions)
reviewed = messages[["id"]].join(forms)
print(reviewed.to_string(index=False))
