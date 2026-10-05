import json
from dataclasses import asdict
import thinkthen as tt

engine = tt.Engine(model="local-1")
lines = tt.read_files("documents", unit="line")
documents = tt.read_files("documents", unit="file")
refunds = engine.filter(
    "Does this line describe a refund?", lines).value
billing = engine.rank(
    "Does this document discuss a billing dispute?",
    documents).value
policy = engine.find(
    "Which line gives the refund policy?", lines).value
contracts = engine.decide(
    "Does this document contain a support contract?",
    documents).value
categories = engine.choose(
    "Which category fits this document?", documents,
    options=["billing", "support"]).value
labels = engine.tag(
    "Which labels apply?", documents,
    labels=["refund", "contract"]).value
urgency = engine.score(
    "How urgent is this document?", documents,
    levels=["a", "b"]).value
document_answers = engine.annotate(
    "questions.json", documents).value
names = engine.recognize(
    documents, kinds=["person", "organization"]).value
connections = engine.relate(
    documents, relations={"connected": ("*", "*")}).value
for function, rows in [
    ("filter", refunds), ("rank", billing),
    ("find", [policy]), ("decide", contracts),
    ("choose", categories), ("tag", labels),
    ("score", urgency), ("annotate", document_answers),
    ("recognize", names),
]:
    print(function)
    for row in rows:
        print(json.dumps(asdict(row), ensure_ascii=False))
print("relate")
for edge in connections:
    edge["source"] = asdict(edge["source"])
    edge["target"] = asdict(edge["target"])
    print(json.dumps(edge, ensure_ascii=False))
windows = tt.read_files(
    ["documents/01-policy.txt", "documents/01-policy.txt"],
    unit="window", window=2)
window_refunds = engine.filter(
    "Does this line describe a refund?", windows).value
print("windows")
for row in window_refunds:
    print(json.dumps(asdict(row), ensure_ascii=False))
