import thinkthen.Door
import thinkthen.Json
import java.nio.charset.StandardCharsets
import java.nio.file.{Files, Path}
import scala.concurrent.{Await, ExecutionContext, Future, Promise}
import scala.concurrent.duration.*

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
@main def scalaCaller(): Unit = {
  given ExecutionContext = ExecutionContext.global
  val engine = new Door()
  try {
    val facade = new ScalaFacade(engine)
    assert(facade.decide("Is it?", "scala-direct").value().outcome() == 1)
    val running = facade.decideAsync("Is it?", "scala-future")
    try assert(Await.result(running.future, 20.seconds).value().probability() == .9)
    finally running.close()
    val token = engine.token()
    try {
      token.fire()
      try { engine.decide("Is it?", "scala-never-sent".getBytes(StandardCharsets.UTF_8), -1L, token); throw new AssertionError("fired token worked") }
      catch { case ex: Door.NativeFailure => assert(ex.failure.code() == 5) }
    } finally token.close()
    assert(Json.parseObject(facade.call("{\"decide\":\"Is it?\",\"evidence\":\"scala-json\"}")).get("value") == java.lang.Boolean.TRUE)
    val barrier = Path.of(System.getenv("TT_BARRIER_DIR"))
    val held = facade.decideAsync("Is it?", "hold-scala")
    try {
      try {
        val arrived = barrier.resolve("arrived-hold-scala")
        var count = 0
        while (!Files.exists(arrived) && count < 2000) { Thread.sleep(5); count += 1 }
        assert(Files.exists(arrived), "held Scala call did not arrive")
        held.cancel()
        held.cancel()
        Thread.sleep(250)
      } finally Files.createFile(barrier.resolve("release-hold-scala"))
      try { Await.result(held.future, 20.seconds); throw new AssertionError("held Scala call returned an Answer after cancellation") }
      catch { case ex: Door.NativeFailure => assert(ex.failure.code() == 5, s"Scala cancellation code: ${ex.failure.code()}") }
      println("SCALA_HELD_SCALAR_CANCELLED_PASS")
    } finally held.close()
    assert(facade.decide("Is it?", "scala-recovery").value().outcome() == 1)
    println("SCALA_PASS")
  } finally engine.close()
}
