.question |= (.verb = "relate" | .threshold = .relation_threshold)
| .value = [.value.relations[] | {relation, source: {name: .source.text, kind: .source.kind}, target: {name: .target.text, kind: .target.kind}, probability}]
