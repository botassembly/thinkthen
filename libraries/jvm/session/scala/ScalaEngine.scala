package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Values
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[OwnedCall], cancel: () => Boolean)
final class ScalaEngine private (private val engine: Engine) extends AutoCloseable {
  def this(settings: Map[String, Any]) = this(new Engine(ScalaEngine.javaMap(settings), Engine.Surface.SCALA))
  def this() = this(Map.empty[String, Any])
  def this(settings: Inputs.EngineSettings) = this(new Engine(ScalaEngine.transport(settings), Engine.Surface.SCALA))
  private def javaMap[K](value: scala.collection.Map[K, Any]): java.util.Map[K, Any] = ScalaEngine.javaMap(value)
  private def run(native: java.util.concurrent.CompletableFuture[Engine.OwnedCall]): Call = {
    val result = Promise[OwnedCall]()
    native.whenComplete((value, error) => { try { if (error == null) result.trySuccess(OwnedCall.read(value)) else result.tryFailure(ScalaEngine.failure(error)) } catch { case error: Throwable => result.tryFailure(ScalaEngine.failure(error)) }; () })
    Call(result.future, () => native.cancel(false))
  }
  def decide(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.decide(javaMap(question), javaMap(input), javaMap(options)))
  def choose(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.choose(javaMap(question), javaMap(input), javaMap(options)))
  def tag(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.tag(javaMap(question), javaMap(input), javaMap(options)))
  def score(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.score(javaMap(question), javaMap(input), javaMap(options)))
  def filter(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.filter(javaMap(question), javaMap(input), javaMap(options)))
  def rank(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.rank(javaMap(question), javaMap(input), javaMap(options)))
  def find(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.find(javaMap(question), javaMap(input), javaMap(options)))
  def annotate(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.annotate(javaMap(question), javaMap(input), javaMap(options)))
  def recognize(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.recognize(javaMap(question), javaMap(input), javaMap(options)))
  def relate(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.relate(javaMap(question), javaMap(input), javaMap(options)))
  def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = decide(question, input, null)
  def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.decide(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = choose(question, input, null)
  def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.choose(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = tag(question, input, null)
  def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.tag(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = score(question, input, null)
  def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.score(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = filter(question, input, null)
  def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.filter(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = rank(question, input, null)
  def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.rank(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = find(question, input, null)
  def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.find(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = annotate(question, input, null)
  def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.annotate(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = recognize(question, input, null)
  def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.recognize(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = relate(question, input, null)
  def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.relate(ScalaEngine.transport(question), ScalaEngine.transport(input), if (options == null) null else ScalaEngine.transport(options)))
  def plan(request: Inputs.Request): Results.Plan = try Results.Plan.read(engine.plan(ScalaEngine.transport(request)).json()) catch { case error: Throwable => throw ScalaEngine.failure(error) }
  def execute(request: Inputs.Request): Call = run(engine.execute(ScalaEngine.transport(request)))
  override def close(): Unit = engine.close()
}

final case class OwnedCall(packets: List[Results.SessionPacket], terminal: Results.SessionPacketTerminal)
object OwnedCall {
  private[scala] def read(value: Engine.OwnedCall): OwnedCall = OwnedCall(value.packets().asScala.toList.map(packet => Results.SessionPacket.read(packet.json())), Results.SessionPacketTerminal.read(value.terminal().json()))
}
final class SessionFailure(val call: OwnedCall) extends RuntimeException(call.terminal.failure.get.error.message) {
  def failure: Results.CallError = call.terminal.failure.get
}
object ScalaEngine {
  private def failure(error: Throwable): Throwable = error match {
    case native: Engine.SessionFailure => new SessionFailure(OwnedCall.read(native.call()))
    case native: thinkthen.NativeFailure => new NativeFailure(native)
    case other => other
  }
  private def transport(value: Values.Value): java.util.Map[String, Any] = javaValue(value.json()).asInstanceOf[java.util.Map[String, Any]]
  private def javaMap[K](value: scala.collection.Map[K, Any]): java.util.Map[K, Any] =
    value.iterator.map { case (key, item) => key -> javaValue(item) }.toMap.asJava
  private def javaValue(value: Any): Any = value match {
    case map: scala.collection.Map[?, ?] => javaMap(map)
    case map: java.util.Map[?, ?] => javaMap(map.asScala)
    case items: java.lang.Iterable[?] => items.asScala.iterator.map(javaValue).toList.asJava
    case number: scala.math.BigDecimal => number.bigDecimal
    case number: scala.math.BigInt => number.bigInteger
    case items: Iterable[?] => items.iterator.map(javaValue).toList.asJava
    case scalar => scalar
  }
}

final class NativeFailure private[scala](error: thinkthen.NativeFailure) extends RuntimeException(error.getMessage, error) {
  val code: Int = error.code()
  val retryable: Boolean = error.retryable()
  val facts: Option[Results.Facts] = Option(error.facts()).map(Results.Facts.read)
}
