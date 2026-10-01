import Foundation
import ThinkThen

struct Facts: Decodable {
    struct Entity: Decodable {
        let text: String
        let kind: String
    }
    let entities: [Entity]
}

let tt = try Engine()
let spec = """
    {"version": 1, "recognize": {"kinds": {
     "person": null,
     "organization": null,
     "place": null}}}
    """
let text = "Maria Chen joined Northwind Freight, "
    + "a company in Chicago."
let recognized = try tt.recognize(spec, text)
let facts = try JSONDecoder().decode(
    Facts.self,
    from: Data(recognized.value.utf8)
)
let names = facts.entities.map { [$0.text, $0.kind] }
precondition(names == [
    ["Maria Chen", "person"],
    ["Northwind Freight", "organization"],
    ["Chicago", "place"],
])
tt.close()
