import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "Which team owns this?";
        var teams = """
            {"billing": "Invoices, fees, and refunds.",
             "shipping": "Parcels and delivery.",
             "account": "Logins and passwords."}""";
        var parcel =
            "My parcel went to the wrong address.";
        var team = Json.parseObject(tt.call(
            "{\"choose\": " + Json.quote(question)
            + ", \"options\": " + teams
            + ", \"evidence\": " + Json.quote(parcel)
            + "}"))
            .get("value");
        assert team.equals("shipping");
    }
}
