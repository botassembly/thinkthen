import java.io.File
import thinkthen.Door
import thinkthen.Door.Outcome

fun main() {
    Door().use { engine ->
        val tt = KotlinFacade(engine)
        val refund = File("refund.json").readText()
        val text = "I would like to return this " +
            "and get my money back.\n"
        val isRefund = tt.decide(refund, text).value()
        check(Door.outcome(isRefund) == Outcome.YES)
    }
}
