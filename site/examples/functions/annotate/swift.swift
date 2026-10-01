import Foundation
import ThinkThen

struct Triage: Decodable, Equatable {
    let steps: Bool
    let area: String
    let impact: Double
}

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

func json(_ value: some Encodable) throws -> String {
    let data = try JSONEncoder().encode(value)
    return String(decoding: data, as: UTF8.self)
}

let tt = try Engine()
let form = try String(
    contentsOfFile: "form.json",
    encoding: .utf8
)
let reports = ["Steps: click Log in. Nobody gets in."]
let annotate = """
    {"annotate": \(form),
     "records": \(try json(reports))}
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
