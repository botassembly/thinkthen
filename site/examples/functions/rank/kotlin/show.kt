import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val messages = File("messages.txt").readLines()
    ThinkThen().use { client ->
        // Every message, most urgent first.
        // answer() is its rank, starting at 1.
        client.rank(
            "How urgent is the request?", messages,
            listOf("routine", "urgent"),
        ).collect { ranked -> println("${ranked.answer()} ${ranked.item}") }
    }
}
