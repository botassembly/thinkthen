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
let text = "My parcel went to the wrong address."
let choose = """
    {"choose": \(try json(question)),
     "options": \(teams),
     "evidence": \(try json(text))}
    """
let reply = try tt.call(choose)
let team = try JSONDecoder().decode(
    Reply<String?>.self,
    from: Data(reply.utf8)
).value
precondition(team == "shipping")
tt.close()
