import dev.thinkthen.kotlin.*
import java.io.File
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val message = File("message.txt").readText()
    ThinkThen().use { client ->
        val names = client.recognize(
            message, listOf("person", "organization"),
        ).answer()
        for (name in names) {
            println("${name.mention.kind}: ${name.mention.span.text}")
        }
    }
}
