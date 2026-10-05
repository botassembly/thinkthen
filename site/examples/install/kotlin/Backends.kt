import thinkthen.Door
import thinkthen.Door.Outcome

fun main() {
    val settings = listOf(
        "{\"backend\":\"typesafe\"}",
        "{\"backend\":\"liquid\"}",
        "{\"backend\":\"ollama\",\"base_url\":\"http://localhost:11535/v1\"}",
    )
    for (setting in settings) {
        Door(setting).use { engine ->
            val tt = KotlinFacade(engine)
            val question =
                "Does the customer ask for a refund?"
            val brokenIsRefund = tt.decide(
                question,
                "Please refund my order. It arrived broken.",
            )
            val thanksIsRefund = tt.decide(
                question,
                "Thanks for the quick help yesterday!",
            )
            val broken = Door.outcome(brokenIsRefund.value())
            val thanks = Door.outcome(thanksIsRefund.value())
            check(broken == Outcome.YES)
            check(thanks == Outcome.NO)
        }
    }
}
