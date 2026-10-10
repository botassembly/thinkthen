package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[OwnedCall], cancel: () => Boolean)
final class ScalaEngine private (private val engine: Engine) extends AutoCloseable {
  def this(settings: Inputs.EngineSettings) = this(ScalaEngine.native(new Engine(ScalaEngine.transport(settings), Engine.Surface.SCALA)))
  def this() = this(new Inputs.EngineSettings())
  private def run(native: java.util.concurrent.CompletableFuture[Engine.OwnedCall]): Call = {
    val result = Promise[OwnedCall]()
    native.whenComplete((value, error) => { try { if (error == null) result.trySuccess(OwnedCall.read(value)) else result.tryFailure(ScalaEngine.failure(error)) } catch { case error: Throwable => result.tryFailure(ScalaEngine.failure(error)) }; () })
    Call(result.future, () => native.cancel(false))
  }
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
  def startSession(request: Inputs.Request): OwnedSession = ScalaEngine.native(new OwnedSession(engine.startSession(ScalaEngine.transport(request))))
  def execute(request: Inputs.Request): Call = run(engine.execute(ScalaEngine.transport(request)))
  def usagePersistence(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.usagePersistence()))
  def finishUsageStatus(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.finishUsageStatus()))
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
  private[scala] def native[T](action: => T): T = try action catch { case error: Throwable => throw failure(error) }
  private def failure(error: Throwable): Throwable = error match {
    case native: Engine.SessionFailure => new SessionFailure(OwnedCall.read(native.call()))
    case native: thinkthen.NativeFailure => new NativeFailure(native)
    case other => other
  }
  private[scala] def transport(value: Inputs.EngineSettings): Inputs.EngineSettings = Inputs.EngineSettings.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.Request): Inputs.Request = Inputs.Request.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.RequestQuestion): Inputs.RequestQuestion = Inputs.RequestQuestion.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.RequestInput): Inputs.RequestInput = Inputs.RequestInput.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.RequestOptions): Inputs.RequestOptions = Inputs.RequestOptions.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.RequestSessionDescriptor): Inputs.RequestSessionDescriptor = Inputs.RequestSessionDescriptor.fromJson(javaValue(value.json()))
  private[scala] def transport(value: Inputs.RequestReaderFailure): Inputs.RequestReaderFailure = Inputs.RequestReaderFailure.fromJson(javaValue(value.json()))
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

/** One producer and one reader over the shared owner. */
final class OwnedSession private[scala] (private val native: thinkthen.OwnedSession) extends AutoCloseable {
  def tryPush(descriptor: Inputs.RequestSessionDescriptor): Int = ScalaEngine.native(native.tryPush(ScalaEngine.transport(descriptor)))
  def finish(failure: Inputs.RequestReaderFailure = null): Unit = ScalaEngine.native(native.finish(if (failure == null) null else ScalaEngine.transport(failure)))
  def cancel(): Unit = native.cancel()
  def tryRead(): OwnedSession.Read = ScalaEngine.native { val read = native.tryRead(); OwnedSession.Read(Option(read.packet()).map(packet => Results.SessionPacket.read(packet.json())), read.ended()) }
  override def close(): Unit = native.close()
}
object OwnedSession { final case class Read(packet: Option[Results.SessionPacket], ended: Boolean) }
