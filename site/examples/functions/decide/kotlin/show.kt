import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        // answer() throws UnsureException when not sure,
        // ThinkThenException on failure.
        val isRefund = client.decide(
            "Does the customer request a refund?", message,
        ).answer()
        println(isRefund)
    }
}
