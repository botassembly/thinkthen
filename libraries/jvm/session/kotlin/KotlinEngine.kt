package thinkthen.kotlin
import thinkthen.Engine
import thinkthen.Inputs
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.ensureActive
import java.util.concurrent.CompletableFuture
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
/** Coroutine facade over the shared owned JVM sessions. */
class KotlinEngine private constructor(private val engine: Engine) : AutoCloseable {
    constructor(settings: Map<String, Any?> = emptyMap()) : this(Engine(settings, Engine.Surface.KOTLIN))
    constructor(settings: Inputs.EngineSettings) : this(Engine(settings, Engine.Surface.KOTLIN))
    private suspend fun await(start: () -> CompletableFuture<Engine.OwnedCall>): Engine.OwnedCall {
        currentCoroutineContext().ensureActive()
        val call = start()
        return suspendCancellableCoroutine { continuation ->
            continuation.invokeOnCancellation { call.cancel(false) }
            call.whenComplete { value, error ->
                if (error != null) continuation.resumeWithException(error)
                else continuation.resume(value)
            }
        }
    }
    suspend fun decide(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.decide(question, input, options) }
    suspend fun choose(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.choose(question, input, options) }
    suspend fun tag(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.tag(question, input, options) }
    suspend fun score(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.score(question, input, options) }
    suspend fun filter(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.filter(question, input, options) }
    suspend fun rank(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.rank(question, input, options) }
    suspend fun find(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.find(question, input, options) }
    suspend fun annotate(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.annotate(question, input, options) }
    suspend fun recognize(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.recognize(question, input, options) }
    suspend fun relate(question: Map<String, Any?>, input: Map<String, Any?>, options: Map<String, Any?>? = null): Engine.OwnedCall = await { engine.relate(question, input, options) }
    suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.decide(question, input, options) }
    suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.choose(question, input, options) }
    suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.tag(question, input, options) }
    suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.score(question, input, options) }
    suspend fun filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.filter(question, input, options) }
    suspend fun rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.rank(question, input, options) }
    suspend fun find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.find(question, input, options) }
    suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.annotate(question, input, options) }
    suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.recognize(question, input, options) }
    suspend fun relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): Engine.OwnedCall = await { engine.relate(question, input, options) }
    override fun close() = engine.close()
}
