import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        val topics = client.tag(
            "Which topics appear?", message,
            listOf("billing", "shipping"),
        ).answer()
        println(topics)
    }
}
