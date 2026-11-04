library(thinkthen)

action <- tt_find("Morgan works for Example Group.\nPlease refund my order.",
                  question = "Which line states the requested action?")
print(action$text)
