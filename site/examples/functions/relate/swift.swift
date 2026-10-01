import Foundation
import ThinkThen

struct Graph: Decodable {
    struct Named: Decodable {
        let name: String
    }
    struct Edge: Decodable {
        let source: Named
        let target: Named
    }
    let edges: [Edge]
}

let tt = try Engine()
let spec = """
    {"version": 1, "relate": {"relations": [
     {"name": "sings", "source": "singer", "target": "song"}
    ]}}
    """
let names = [
    #"{"name": "Paul McCartney", "kind": "singer"}"#,
    #"{"name": "Ringo Starr", "kind": "singer"}"#,
    #"{"name": "Yesterday", "kind": "song"}"#,
    #"{"name": "Octopus's Garden", "kind": "song"}"#,
]
let related = try tt.relate(spec, names)
let whoSings = try JSONDecoder().decode(
    Graph.self,
    from: Data(related.value.utf8)
)
let sings = whoSings.edges.map {
    [$0.source.name, $0.target.name]
}
precondition(sings == [
    ["Paul McCartney", "Yesterday"],
    ["Ringo Starr", "Octopus's Garden"],
])
tt.close()
