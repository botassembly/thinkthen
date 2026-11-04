import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message =
        "Morgan works for Example Group.\nPlease refund my order."
    ThinkThen().use { client ->
        val action = client.find(
            "Which line states the requested action?", message,
        ).answer()
        println(action?.text ?: "no line answers")
    }
}
