import thinkthen as tt

message = "Morgan works for Example Group.\nPlease refund my order."
rules = [
    tt.RelationRule(
        source="person", target="organization", relation="works_for"
    )
]
relations = tt.relate(message, rules=rules)
for relation in relations.answer:
    print(relation.source.text, relation.relation, relation.target.text)
