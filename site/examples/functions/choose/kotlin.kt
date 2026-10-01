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
        val text =
            "Please refund the extra fee on my invoice."
        val owner = Json.parseObject(tt.call(
            """{"choose": ${Json.quote(question)},
               "options": $teams,
               "evidence": ${Json.quote(text)}}"""
        ))["value"]
        check(owner == "billing")
    }
}
