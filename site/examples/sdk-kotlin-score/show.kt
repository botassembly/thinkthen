import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val urgency = client.score(
            "How urgent is the request?",
            "Please refund my order.",
            listOf("routine", "urgent"),
        ).answer()
        println(urgency)
    }
}
