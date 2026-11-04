import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        val urgency = client.score(
            "How urgent is the request?", message,
            listOf("routine", "urgent"),
        ).answer()
        println(urgency)
    }
}
