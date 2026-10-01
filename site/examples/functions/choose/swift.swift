import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "Which team owns this?"
let teams = """
    {"billing": "Invoices, fees, and refunds.",
     "shipping": "Parcels and delivery.",
     "account": "Logins and passwords."}
    """
let texts = [
    "Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.",
    "I cannot reset my password.",
    "My parcel never came, and now "
        + "I cannot log in to track it.",
]
let owners = try texts.map { text in
    let team = try tt.call("""
        {"choose": "\(question)", "options": \(teams),
         "threshold": 0.9, "evidence": "\(text)"}
        """)
    return try JSONDecoder().decode(
        Reply<String?>.self,
        from: Data(team.utf8)
    ).value
}
precondition(
    owners == ["billing", "shipping", "account", nil]
)
tt.close()
