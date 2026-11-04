import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        val topic = client.choose(
            "What is the message about?", message,
            listOf("billing", "shipping", "other"),
        ).answer()
        println(topic)
    }
}
