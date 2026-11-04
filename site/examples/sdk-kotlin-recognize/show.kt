import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val names = client.recognize(
            "Morgan works for Example Group.",
            listOf("person", "organization"),
        ).answer()
        for (name in names) {
            val mention = name.mention
            println("${mention.kind} ${mention.span.text}")
        }
    }
}
