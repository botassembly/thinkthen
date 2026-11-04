import thinkthen as tt

rules = [
    tt.RelationRule(source="person", target="organization",
                    relation="works_for")
]
text = "Morgan works for Example Group.\nPlease refund my order.\n"
relations = tt.relate(text, rules=rules)
for relation in relations:
    print(relation.source.kind, repr(relation.source.text),
          relation.relation,
          relation.target.kind, repr(relation.target.text),
          relation.probability)
# Every name relate recognized, related or not.
if relations.detail is not None:
    for name in relations.detail.names:
        print("name:", name)
