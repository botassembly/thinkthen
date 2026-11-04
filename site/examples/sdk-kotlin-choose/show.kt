import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val topic = client.choose(
            "What is the message about?",
            "Please refund my order.",
            listOf("billing", "shipping", "other"),
        ).answer()
        println(topic)
    }
}
