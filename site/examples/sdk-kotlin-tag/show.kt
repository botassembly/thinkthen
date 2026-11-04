import dev.thinkthen.kotlin.*
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    ThinkThen().use { client ->
        val topics = client.tag(
            "Which topics appear?",
            "I was charged twice and my parcel is late again.",
            listOf("billing", "shipping"),
        ).answer()
        println(topics)
    }
}
