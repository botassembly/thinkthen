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
     "PER": "Part of a person's name.",
     "ORG": "Part of the name of an organization: \
    a company, band, team, agency, government body, \
    or media outlet.",
     "LOC": "Part of the name of a place: a country, \
    region, city, or geographic feature.",
     "MISC": "Part of another named entity: a \
    nationality, an event, a product, or the name of \
    a creative work."}}}
    """
let text = "Maria Chen joined Northwind Freight "
    + "in Chicago last spring."
let recognized = try tt.recognize(spec, text)
let facts = try JSONDecoder().decode(
    Facts.self,
    from: Data(recognized.value.utf8)
)
let names = facts.entities.map { [$0.text, $0.kind] }
precondition(names == [
    ["Maria Chen", "PER"],
    ["Northwind Freight", "ORG"],
    ["Chicago", "LOC"],
])
tt.close()
