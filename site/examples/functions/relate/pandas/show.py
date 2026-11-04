import pandas as pd
import thinkthen as tt
import thinkthen.pandas  # adds the .thinkthen accessor

messages = pd.DataFrame(
    {"message": ["Morgan works for Example Group.\nPlease refund my order."]}
)
rules = [
    tt.RelationRule(
        source="person", target="organization", relation="works_for"
    )
]
relations = messages["message"].thinkthen.relate(rules=rules)
for relation in relations.explode():
    print(relation.source.text, relation.relation, relation.target.text)
