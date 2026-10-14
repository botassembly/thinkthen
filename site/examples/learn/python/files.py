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
for function, rows in [
    ("filter", refunds), ("rank", billing),
    ("find", policy), ("decide", contracts),
    ("choose", categories), ("tag", labels),
    ("score", urgency), ("annotate", document_answers),
    ("recognize", names),
]:
    print(function)
    for row in rows:
        source = getattr(row, "source", None)
        if source is not None:
            print(source.file, source.first_line, source.last_line)
        if function == "recognize":
            print([(entity.text, entity.kind) for entity in row.value.entities])
        elif function == "annotate":
            print(row.value)
        else:
            print(row.value)
        if function == "find":
            for candidate in row.candidates:
                source = candidate.source
                print(candidate.index, source.file, source.first_line, source.last_line, candidate.probability)
print("relate")
for edge in connections:
    print(edge.relation, edge.source.name, edge.target.name, edge.probability)
windows = tt.read_files(
    ["documents/01-policy.txt", "documents/01-policy.txt"],
    unit="window", window=2)
window_refunds = engine.filter(
    "Does this line describe a refund?", windows).details
print("windows")
for row in window_refunds:
    print(row.source.file, row.source.first_line, row.source.last_line, row.value)
