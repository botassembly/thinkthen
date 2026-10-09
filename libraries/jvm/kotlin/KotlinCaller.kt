import thinkthen.Door
import java.nio.charset.StandardCharsets
import java.util.concurrent.atomic.AtomicReference

/** Kotlin facade over the same Java FFM door. Caller joins before engine close. */
class KotlinFacade(private val engine: Door) {
    fun decide(question: String, evidence: String): Door.TypedResult<Door.Answer> = engine.decide(question, evidence.toByteArray(StandardCharsets.UTF_8))
    fun files(question: String, paths: List<String>): String = engine.files(question, paths)
    fun call(request: String): String = engine.call(request)
    inner class RunningDecision(question: String, evidence: String): AutoCloseable {
        private val token = engine.token()
        private val result = AtomicReference<Door.TypedResult<Door.Answer>>()
        private val failure = AtomicReference<Throwable>()
        private val caller = Thread.ofPlatform().start {
            try { result.set(engine.decide(question, evidence.toByteArray(StandardCharsets.UTF_8), -1, token)) }
            catch (ex: Throwable) { failure.set(ex) }
        }
        fun cancel() = token.fire()
        fun await(): Door.TypedResult<Door.Answer> {
            caller.join()
            failure.get()?.let { throw it }
            return result.get()
        }
        override fun close() { caller.join(); token.close() }
    }
    fun decideAsync(question: String, evidence: String) = RunningDecision(question, evidence)
}
