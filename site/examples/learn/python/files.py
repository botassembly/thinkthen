import json
import thinkthen as tt

engine = tt.Engine(model="local-1")
lines = tt.read_files("documents", unit="line")
documents = tt.read_files("documents", unit="file")
refunds = engine.filter(
    "Does this line describe a refund?", lines).details
billing = engine.rank(
    "Does this document discuss a billing dispute?",
    documents).details
policy = engine.find(
    "Which line gives the refund policy?", lines).details
contracts = engine.decide(
    "Does this document contain a support contract?",
    documents).details
categories = engine.choose(
    "Which category fits this document?", documents,
    options=["billing", "support"]).details
labels = engine.tag(
    "Which labels apply?", documents,
    labels=["refund", "contract"]).details
urgency = engine.score(
    "How urgent is this document?", documents,
    levels=["a", "b"]).details
document_answers = engine.annotate(
    "questions.json", documents).details
names = engine.recognize(
    documents, kinds=["person", "organization"]).details
connections = engine.relate(
    documents, relations={"connected": ("*", "*")}).value
fields = ("input", "source", "value", "candidates")
for function, rows in [
    ("filter", refunds), ("rank", billing),
    ("find", policy), ("decide", contracts),
    ("choose", categories), ("tag", labels),
    ("score", urgency), ("annotate", document_answers),
    ("recognize", names),
]:
    print(function)
    for row in rows:
        data = row.to_dict()
        shown = {key: data[key] for key in fields
                 if key in data}
        print(json.dumps(shown, ensure_ascii=False))
print("relate")
for edge in connections:
    print(json.dumps(edge.to_dict(), ensure_ascii=False))
windows = tt.read_files(
    ["documents/01-policy.txt", "documents/01-policy.txt"],
    unit="window", window=2)
window_refunds = engine.filter(
    "Does this line describe a refund?", windows).details
print("windows")
for row in window_refunds:
    data = row.to_dict()
    shown = {key: data[key] for key in fields
             if key in data}
    print(json.dumps(shown, ensure_ascii=False))
