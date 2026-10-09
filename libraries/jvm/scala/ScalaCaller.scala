import thinkthen.Door
import java.nio.charset.StandardCharsets
import scala.concurrent.{ExecutionContext, Future, Promise}

/** Scala facade over the same Java FFM door. Await before closing the engine. */
final class ScalaFacade(engine: Door)(using ExecutionContext) {
  def decide(question: String, evidence: String): Door.TypedResult[Door.Answer] =
    engine.decide(question, evidence.getBytes(StandardCharsets.UTF_8))
  def files(question: String, paths: java.util.List[String]): String = engine.files(question, paths)
  def call(request: String): String = engine.call(request)
  final class RunningDecision(question: String, evidence: String) extends AutoCloseable {
    private val token = engine.token()
    private val promise = Promise[Door.TypedResult[Door.Answer]]()
    private val caller = Thread.ofPlatform().start(() =>
      try promise.success(engine.decide(question, evidence.getBytes(StandardCharsets.UTF_8), -1L, token))
      catch { case ex: Throwable => promise.failure(ex) }
    )
    def future: Future[Door.TypedResult[Door.Answer]] = promise.future
    def cancel(): Unit = token.fire()
    override def close(): Unit = { caller.join(); token.close() }
  }
  def decideAsync(question: String, evidence: String): RunningDecision = new RunningDecision(question, evidence)
}
