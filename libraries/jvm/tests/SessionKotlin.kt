import thinkthen.kotlin.KotlinEngine
import thinkthen.Presence
import kotlinx.coroutines.*
import java.nio.file.Files
import java.nio.file.Path

fun main() = runBlocking {
    KotlinEngine(mapOf("cache" to false, "max_retries" to 0)).use { engine ->
        val question = mapOf("kind" to "text", "text" to "Is it?")
        fun input(text: String): Map<String, Any?> = mapOf("kind" to "text", "text" to text)
        val held = launch { engine.decide(question, input("hold-jvm-kotlin")) }
        withTimeout(10000) { while (!Files.exists(Path.of("barrier/arrived-hold-jvm-kotlin"))) delay(5) }
        val independent = withTimeout(10000) { engine.decide(question, input("session-kotlin-independent")) }
        check(independent.terminal().facts().state() == Presence.State.VALUE)
        withTimeout(2000) { held.cancelAndJoin() }
        check(!Files.exists(Path.of("barrier/release-hold-jvm-kotlin")))
        check(independent.terminal().facts().value().requestsSent().intValueExact() == 1)
    }
    println("INSTALLED_JVM_KOTLIN_SESSION_PASS")
}
