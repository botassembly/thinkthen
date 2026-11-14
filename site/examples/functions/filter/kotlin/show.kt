import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = File("messages.txt").readLines()
    ThinkThen().use { client ->
        // Only the messages that answer yes.
        // A failed message throws after the kept ones.
        client.filter("Does the customer request a refund?", messages)
            .collect { refund -> println(refund) }
    }
}
