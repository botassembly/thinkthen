import dev.thinkthen.kotlin.*
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = listOf(
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    )
    ThinkThen().use { client ->
        client.filter(
            "Does the customer request a refund?", messages,
        ).collect { refund -> println(refund) }
    }
}
