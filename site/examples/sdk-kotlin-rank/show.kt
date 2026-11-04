import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = listOf(
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    )
    ThinkThen().use { client ->
        client.rank(
            "How urgent is the request?", messages,
            listOf("routine", "urgent"),
        ).collect { ranked ->
            val message = messages[ranked.position.toInt() - 1]
            println("${ranked.answer()} $message")
        }
    }
}
