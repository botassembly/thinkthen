import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val isRefund = client.decide(
            "Does the customer request a refund?",
            "I was charged twice for one invoice.",
            threshold = Threshold.band(0.2, 0.8),
        )
        try {
            if (isRefund.answer()) println("Send to refunds.")
            else println("Leave in the inbox.")
        } catch (notSure: UnsureException) {
            println("Not sure. Ask a person.")
        }
    }
}
