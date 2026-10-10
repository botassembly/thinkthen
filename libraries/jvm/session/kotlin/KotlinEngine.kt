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
    constructor(settings: Inputs.EngineSettings = Inputs.EngineSettings()) : this(nativeResult { Engine(settings, Engine.Surface.KOTLIN) })
    private suspend fun await(start: () -> CompletableFuture<Engine.OwnedCall>): OwnedCall {
        currentCoroutineContext().ensureActive()
        val call = start()
        return suspendCancellableCoroutine { continuation ->
            continuation.invokeOnCancellation { call.cancel(false) }
            call.whenComplete { value, error ->
                if (error != null) continuation.resumeWithException(convertFailure(error))
                else try { continuation.resume(OwnedCall.read(value)) } catch (error: Throwable) { continuation.resumeWithException(convertFailure(error)) }
            }
        }
    }
    suspend fun decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.decide(question, input, options) }
    suspend fun choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.choose(question, input, options) }
    suspend fun tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.tag(question, input, options) }
    suspend fun score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.score(question, input, options) }
    suspend fun filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.filter(question, input, options) }
    suspend fun rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.rank(question, input, options) }
    suspend fun find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.find(question, input, options) }
    suspend fun annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.annotate(question, input, options) }
    suspend fun recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.recognize(question, input, options) }
    suspend fun relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions? = null): OwnedCall = await { engine.relate(question, input, options) }
    fun plan(request: Inputs.Request): Results.Plan = try { Results.Plan.read(engine.plan(request).json()) } catch (error: Throwable) { throw convertFailure(error) }
    fun startSession(request: Inputs.Request): OwnedSession = nativeResult { OwnedSession(engine.startSession(request)) }
    suspend fun execute(request: Inputs.Request): OwnedCall = await { engine.execute(request) }
    fun usagePersistence(): UsagePersistence = nativeResult { UsagePersistence.read(engine.usagePersistence()) }
    fun finishUsageStatus(): UsagePersistence = nativeResult { UsagePersistence.read(engine.finishUsageStatus()) }
    override fun close() = engine.close()
}

/** All packets are copied owned native-language views, independent of engine lifetime. */
data class OwnedCall(val packets: List<Results.SessionPacket>, val terminal: Results.SessionPacketTerminal) {
    companion object { internal fun read(value: Engine.OwnedCall) = OwnedCall(value.packets().map { Results.SessionPacket.read(it.json()) }, Results.SessionPacketTerminal.read(value.terminal().json())) }
}
class SessionFailure(val call: OwnedCall) : RuntimeException(call.terminal.failure!!.error.message) {
    val failure: Results.CallError get() = call.terminal.failure!!
}
private fun convertFailure(error: Throwable): Throwable = when (error) {
    is Engine.SessionFailure -> SessionFailure(OwnedCall.read(error.call()))
    is thinkthen.NativeFailure -> NativeFailure(error)
    else -> error
}
class NativeFailure internal constructor(error: thinkthen.NativeFailure) : RuntimeException(error.message, error) {
    val code: Int = error.code()
    val retryable: Boolean = error.retryable()
    val facts: Results.Facts? = error.facts()?.let { Results.Facts.read(it) }
}
private fun <T> nativeResult(action: () -> T): T = try { action() } catch (error: Throwable) { throw convertFailure(error) }

/** One producer and one reader; pending reads remain distinct from end. */
class OwnedSession internal constructor(private val native: thinkthen.OwnedSession) : AutoCloseable {
    fun tryPush(descriptor: Inputs.RequestSessionDescriptor): Int = nativeResult { native.tryPush(descriptor) }
    fun finish(failure: Inputs.RequestReaderFailure? = null) = nativeResult { native.finish(failure) }
    fun cancel() = native.cancel()
    fun tryRead(): Read = nativeResult { val read = native.tryRead(); Read(read.packet()?.let { Results.SessionPacket.read(it.json()) }, read.ended()) }
    override fun close() = native.close()
    data class Read(val packet: Results.SessionPacket?, val ended: Boolean)
}
