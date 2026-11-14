import pandas as pd
import thinkthen as tt
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame({"message": ["Please refund my order.", None]})
try:
    messages["is_refund"] = messages["message"].thinkthen.decide(
        "Does the customer request a refund?"
    )
except tt.Failed as failure:
    outcomes = failure.outcomes
    print(outcomes[["status", "answer", "error_code"]])
