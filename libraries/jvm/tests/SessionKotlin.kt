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
    val authored = listOf("zebra", "mango", "apple", "tango", "berry").fold(Inputs.Ordered<Inputs.AuthoredDescription>()) { values, name -> values.entry(name, Inputs.AuthoredDescription(name)) }
    thinkthen.AuthoredOrderCases.check(Inputs.RequestQuestionDefinition().value(Inputs.RequestDefinitionAuthoredChoose().choose(Inputs.AuthoredQuestionText("Which?")).options(Inputs.AuthoredOptions(authored))), "options")
    thinkthen.AuthoredOrderCases.check(Inputs.RequestQuestionDefinition().value(Inputs.RequestDefinitionRecognizeDefinition().recognize(Inputs.RecognizeQuestion().kinds(authored))), "kinds")
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
        check(independent.value == true && independent.probability == java.math.BigDecimal("0.9"))
        check(independent.results.size == 1 && independent.facts.requestsSent().intValueExact() == 1)
        withTimeout(2000) { held.cancelAndJoin() }
        check(!Files.exists(Path.of("barrier/release-hold-jvm-kotlin")))
        retained = independent
        try { engine.decide(question, input("status-401")); error("provider failure succeeded") }
        catch (error: SessionFailure) { failure = error }
        runKotlinExamples(engine)
    }
    check(retained.terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.call().terminal().facts().value().requestsSent().intValueExact() == 1)
    check(failure.failure().error().message().isNotEmpty())
    println("INSTALLED_JVM_KOTLIN_SESSION_PASS")
}
