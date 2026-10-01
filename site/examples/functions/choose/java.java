import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "Which team owns this?";
        var teams = """
            {"billing": "Invoices, fees, and refunds.",
             "shipping": "Parcels and delivery.",
             "account": "Logins and passwords."}""";
        var text =
            "Please refund the extra fee on my invoice.";
        var owner = Json.parseObject(tt.call(
            "{\"choose\": " + Json.quote(question)
            + ", \"options\": " + teams
            + ", \"evidence\": " + Json.quote(text) + "}"))
            .get("value");
        assert owner.equals("billing");
    }
}
