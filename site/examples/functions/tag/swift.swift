import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "Which labels fit this message?"
let tag = """
    {"tag": "\(question)",
     "labels": ["praise", "bug", "billing"],
     "evidence": "Love the new dashboard, but export \
    crashes the app,\\nand I was charged twice.\\n"}
    """
let labels = try tt.call(tag)
let fittingLabels = try JSONDecoder().decode(
    Reply<[String]>.self,
    from: Data(labels.utf8)
).value
precondition(fittingLabels == ["praise", "bug", "billing"])
tt.close()
