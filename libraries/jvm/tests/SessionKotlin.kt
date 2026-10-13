import thinkthen.kotlin.KotlinEngine
import thinkthen.Presence
import thinkthen.kotlin.OwnedCall
import thinkthen.kotlin.SessionFailure
import thinkthen.Results
import thinkthen.Inputs
import kotlinx.coroutines.*
import java.nio.file.Files
import java.nio.file.Path

fun main() = runBlocking {
    val missing = Results.AnnotationValueDecision(mapOf("kind" to "decision"))
    check(missing.value().state() == Presence.State.MISSING)
    try { missing.value().value(); error("required nullable member was missing") } catch (_: IllegalStateException) { }
    val nil = Results.AnnotationValueDecision(mapOf("kind" to "decision", "value" to null, "future" to false))
    check(nil.value().state() == Presence.State.NULL && nil.value().state() == Presence.State.NULL)
    check(thinkthen.Values.`object`(nil.json())["future"] == false)
    check(Results.AnnotationValueDecision(mapOf("kind" to "decision", "value" to false)).value().value() == false)
    val optional = Results.SessionPacketTerminal(mapOf("kind" to "terminal", "facts" to null))
    check(optional.facts().state() == Presence.State.NULL && optional.facts().state() == Presence.State.NULL)
    lateinit var retained: OwnedCall
    lateinit var failure: SessionFailure
    KotlinEngine(Inputs.EngineSettings().cache(Inputs.CacheDocument(false)).maxRetries(0)).use { engine ->
        val question = Inputs.RequestQuestionText().text("Is it?")
        fun input(text: String) = Inputs.RequestInputText().text(text)
        try { engine.decide(question, input("kotlin-invalid"), Inputs.RequestOptions().contextNull()); error("null context admitted") }
        catch (_: thinkthen.kotlin.NativeFailure) { }
        val plan = engine.plan(Inputs.Request().call(Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(input("kotlin-plan"))))
        check(plan.requests().signum() > 0)
        val held = launch { engine.decide(question, input("hold-jvm-kotlin")) }
        withTimeout(10000) { while (!Files.exists(Path.of("barrier/arrived-hold-jvm-kotlin"))) delay(5) }
        val independent = withTimeout(10000) { engine.decide(question, input("session-kotlin-independent")) }
        check(independent.terminal().facts().state() == Presence.State.VALUE)
        withTimeout(2000) { held.cancelAndJoin() }
        check(!Files.exists(Path.of("barrier/release-hold-jvm-kotlin")))
        retained = independent
        try { engine.decide(question, input("status-401")); error("provider failure succeeded") }
        catch (error: SessionFailure) { failure = error }
    }
    check(retained.terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.call().terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.failure().error().message().isNotEmpty())
    println("INSTALLED_JVM_KOTLIN_SESSION_PASS")
}
