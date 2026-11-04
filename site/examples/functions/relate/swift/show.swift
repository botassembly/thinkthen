import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let worksFor = RelationRule(
    source: "person", target: "organization", relation: "works_for")
let client = try Client()
let relations = try await client.relate(
    rules: [worksFor], record: .text(message)).get()
for relation in relations {
    print("\(relation.source.text) --\(relation.relation)--> "
        + "\(relation.target.text)")
}
