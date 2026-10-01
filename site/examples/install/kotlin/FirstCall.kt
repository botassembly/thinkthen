import thinkthen.Door
import thinkthen.Door.Outcome

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question =
            "Does the customer ask for a refund?"
        var isRefund = tt.decide(
            question,
            "Please refund my order. It arrived broken.",
        )
        check(Door.outcome(isRefund.value()) == Outcome.YES)

        val refund = """
            {"decide": "$question",
             "threshold": "0.2:0.8"}
        """.trimIndent()
        isRefund = tt.decide(
            refund,
            "I want to send this back.",
        )
        val outcome = Door.outcome(isRefund.value())
        check(outcome == Outcome.NOT_SURE)
    }
}
