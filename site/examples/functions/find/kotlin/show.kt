import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        // null when no line answers the question.
        val action = client.find(
            "Which line states the requested action?", message,
        ).answer()
        println(action?.text ?: "no line answers")
    }
}
