import thinkthen.kotlin.KotlinEngine
import thinkthen.Presence
import thinkthen.Engine
import thinkthen.Inputs
import kotlinx.coroutines.*
import java.nio.file.Files
import java.nio.file.Path

fun main() = runBlocking {
    lateinit var retained: Engine.OwnedCall
    lateinit var failure: Engine.SessionFailure
    KotlinEngine(Inputs.EngineSettings().cache(Inputs.CacheDocument(false)).maxRetries(0)).use { engine ->
        val question = Inputs.RequestQuestionText().text("Is it?")
        fun input(text: String) = Inputs.RequestInputText().text(text)
        val held = launch { engine.decide(question, input("hold-jvm-kotlin")) }
        withTimeout(10000) { while (!Files.exists(Path.of("barrier/arrived-hold-jvm-kotlin"))) delay(5) }
        val independent = withTimeout(10000) { engine.decide(question, input("session-kotlin-independent")) }
        check(independent.terminal().facts().state() == Presence.State.VALUE)
        withTimeout(2000) { held.cancelAndJoin() }
        check(!Files.exists(Path.of("barrier/release-hold-jvm-kotlin")))
        retained = independent
        try { engine.decide(question, input("status-401")); error("provider failure succeeded") }
        catch (error: Engine.SessionFailure) { failure = error }
    }
    check(retained.terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.call().terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.failure().error().message().isNotEmpty())
    println("INSTALLED_JVM_KOTLIN_SESSION_PASS")
}
