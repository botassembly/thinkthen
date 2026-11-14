import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val refundCheck = client.decide("Does the customer request a refund?", "")
        println(refundCheck)
    }
}
