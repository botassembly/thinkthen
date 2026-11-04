import thinkthen as tt

message = "Morgan works for Example Group.\nPlease refund my order."
action = tt.find(
    "Which line states the requested action?", message
)
print(action)
