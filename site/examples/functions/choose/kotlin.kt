import thinkthen.Door
import thinkthen.Json

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question = "Which team owns this?"
        val teams = """
            {"billing": "Invoices, fees, and refunds.",
             "shipping": "Parcels and delivery.",
             "account": "Logins and passwords."}
        """.trimIndent()
        val parcel =
            "My parcel went to the wrong address."
        val team = Json.parseObject(tt.call(
            """{"choose": ${Json.quote(question)},
               "options": $teams,
               "evidence": ${Json.quote(parcel)}}"""
        ))["value"]
        check(team == "shipping")
    }
}
