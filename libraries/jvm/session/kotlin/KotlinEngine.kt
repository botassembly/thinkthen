package thinkthen.kotlin
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Results
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.ensureActive
import java.util.concurrent.CompletableFuture
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
/** Coroutine facade over the shared owned JVM sessions. */
class KotlinEngine private constructor(private val engine: Engine) : KotlinNamedCalls(engine), AutoCloseable {
    constructor(settings: Inputs.EngineSettings = Inputs.EngineSettings()) : this(nativeResult { Engine(settings, Engine.Surface.KOTLIN) })
    protected override suspend fun <T> await(start: () -> CompletableFuture<T>): T = awaitNative(start)
    fun plan(request: Inputs.Request): Results.Plan = try { engine.plan(request) } catch (error: Throwable) { throw convertFailure(error) }
    fun startSession(request: Inputs.Request): OwnedSession = nativeResult { OwnedSession(engine.startSession(request)) }
    suspend fun execute(request: Inputs.Request): OwnedCall = OwnedCall(await { engine.execute(request) })
    suspend fun execute(request: Inputs.Request, feed: Iterable<Inputs.RequestSessionDescriptor>): OwnedCall = OwnedCall(await { engine.execute(request, feed) })
    fun usagePersistence(): UsagePersistence = nativeResult { UsagePersistence.read(engine.usagePersistence()) }
    fun finishUsageStatus(): UsagePersistence = nativeResult { UsagePersistence.read(engine.finishUsageStatus()) }
    override fun close() = engine.close()
}

typealias SessionFailure = Engine.SessionFailure
private suspend fun <T> awaitNative(start: () -> CompletableFuture<T>): T {
    currentCoroutineContext().ensureActive()
    val pending = start()
    return suspendCancellableCoroutine { continuation ->
        continuation.invokeOnCancellation { pending.cancel(false) }
        pending.whenComplete { value, error -> if (error == null) continuation.resume(value) else continuation.resumeWithException(convertFailure(error)) }
    }
}
private fun convertFailure(error: Throwable): Throwable = when (error) {
    is java.util.concurrent.CompletionException -> error.cause?.let { convertFailure(it) } ?: error
    is Engine.SessionFailure -> error
    is thinkthen.NativeFailure -> NativeFailure(error)
    else -> error
}
class NativeFailure internal constructor(error: thinkthen.NativeFailure) : RuntimeException(error.message, error) {
    val kind: thinkthen.ErrorKind = error.kind()
    val code: Int = error.code()
    val retryable: Boolean = error.retryable()
    val facts: Results.ImmediateFacts? = error.facts()
}
private fun <T> nativeResult(action: () -> T): T = try { action() } catch (error: Throwable) { throw convertFailure(error) }

/** One producer and one reader; pending reads remain distinct from end. */
class OwnedSession internal constructor(private val native: thinkthen.OwnedSession) : AutoCloseable {
    fun tryPush(descriptor: Inputs.RequestSessionDescriptor): Int = nativeResult { native.tryPush(descriptor) }
    suspend fun push(descriptor: Inputs.RequestSessionDescriptor): Int = awaitNative { native.push(descriptor) }
    suspend fun nextPacket(settlement: Boolean = false): Read = awaitNative { native.nextPacketAsync(settlement) }.let { Read(it.packet(), it.ended()) }
    fun finish(failure: Inputs.RequestReaderFailure? = null) = nativeResult { native.finish(failure) }
    fun cancel() = native.cancel()
    suspend fun waitReady(input: Boolean) { awaitNative { native.waitReady(input) } }
    fun tryRead(): Read = nativeResult { val read = native.tryRead(); Read(read.packet(), read.ended()) }
    override fun close() = native.close()
    data class Read(val packet: Results.SessionPacket?, val ended: Boolean)
}
