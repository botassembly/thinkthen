import thinkthen.Door
import thinkthen.Door.Outcome

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val question =
            "Does the customer ask for a refund?"
        val text =
            "Please refund my order. It arrived broken."
        val isRefund = tt.decide(question, text).value()
        check(Door.outcome(isRefund) == Outcome.YES)
    }
}
