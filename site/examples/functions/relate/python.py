import thinkthen as tt

entities = [
    ("gateway", "service"),
    ("billing", "service"),
]
edges = tt.Engine(model="local-1").relate(
    entities,
    relations={"calls": ("service", "service")},
    threshold=0.5,
).value
pairs = [
    (edge.relation, edge.source.name, edge.target.name)
    for edge in edges
]
assert pairs == [("calls", "gateway", "billing")]
