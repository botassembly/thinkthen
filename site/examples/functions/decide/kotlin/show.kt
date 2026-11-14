import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        // A scalar failure throws FailedException at the call.
        // Reading an unsure answer throws UnsureException.
        val isRefund = client.decide(
            "Does the customer request a refund?", message,
        ).answer()
        println(isRefund)
    }
}
