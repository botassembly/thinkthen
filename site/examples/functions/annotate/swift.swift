import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
struct Triage: Decodable, Equatable {
    let steps: Bool
    let area: String
    let impact: Double
}

let form = try String(
    contentsOfFile: "form.json",
    encoding: .utf8
)
let annotate = """
    {"annotate": \(form),
     "records": ["Steps: click Log in. Nobody gets in."]}
    """
let annotated = try tt.call(annotate)
let triage = try JSONDecoder().decode(
    Reply<[Triage]>.self,
    from: Data(annotated.utf8)
).value
precondition(triage == [
    Triage(steps: true, area: "login", impact: 1.98),
])
tt.close()
