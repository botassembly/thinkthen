import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

func json(_ value: some Encodable) throws -> String {
    let data = try JSONEncoder().encode(value)
    return String(decoding: data, as: UTF8.self)
}

let tt = try Engine()
let question = "Which team owns this?"
let teams = """
    {"billing": "Invoices, fees, and refunds.",
     "shipping": "Parcels and delivery.",
     "account": "Logins and passwords."}
    """
let text = "Please refund the extra fee on my invoice."
let choose = """
    {"choose": \(try json(question)),
     "options": \(teams),
     "evidence": \(try json(text))}
    """
let team = try tt.call(choose)
let owner = try JSONDecoder().decode(
    Reply<String?>.self,
    from: Data(team.utf8)
).value
precondition(owner == "billing")
tt.close()
