import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let choose = """
    {"choose": "Which team owns this?",
     "options": {
      "billing": "Invoices, fees, and refunds.",
      "shipping": "Parcels and delivery.",
      "account": "Logins and passwords."},
     "evidence": "Please refund the extra fee on my \
    invoice."}
    """
let team = try tt.call(choose)
let owner = try JSONDecoder().decode(
    Reply<String?>.self,
    from: Data(team.utf8)
).value
precondition(owner == "billing")
tt.close()
