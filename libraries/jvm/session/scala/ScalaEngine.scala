package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Results
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call[+T <: OwnedCall](result: Future[T], cancel: () => Boolean)
final class ScalaEngine private (private val engine: Engine) extends ScalaNamedCalls(engine) with AutoCloseable {
  def this(settings: Inputs.EngineSettings) = this(ScalaEngine.native(new Engine(settings, Engine.Surface.SCALA)))
  def this() = this(new Inputs.EngineSettings())
  protected override def run[A,B <: OwnedCall](native: java.util.concurrent.CompletableFuture[A])(convert: A => B): Call[B] = {
    val result = Promise[B]()
    native.whenComplete((value, error) => { try { if (error == null) result.trySuccess(convert(value)) else result.tryFailure(ScalaEngine.failure(error)) } catch { case error: Throwable => result.tryFailure(ScalaEngine.failure(error)) }; () })
    Call(result.future, () => native.cancel(false))
  }
  def plan(request: Inputs.Request): Results.Plan = try engine.plan(request) catch { case error: Throwable => throw ScalaEngine.failure(error) }
  def startSession(request: Inputs.Request): OwnedSession = ScalaEngine.native(new OwnedSession(engine.startSession(request)))
  def execute(request: Inputs.Request): Call[OwnedCall] = run(engine.execute(request))(new OwnedCall(_))
  def execute(request: Inputs.Request, feed: Iterable[Inputs.RequestSessionDescriptor]): Call[OwnedCall] = run(engine.execute(request, feed.asJava))(new OwnedCall(_))
  def usagePersistence(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.usagePersistence()))
  def finishUsageStatus(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.finishUsageStatus()))
  override def close(): Unit = engine.close()
}

type SessionFailure = Engine.SessionFailure
object ScalaEngine {
  private[scala] def native[T](action: => T): T = try action catch { case error: Throwable => throw failure(error) }
  private[scala] def failure(error: Throwable): Throwable = error match {
    case wrapped: java.util.concurrent.CompletionException if wrapped.getCause != null => failure(wrapped.getCause)
    case native: Engine.SessionFailure => native
    case native: thinkthen.NativeFailure => new NativeFailure(native)
    case other => other
  }

}

final class NativeFailure private[scala](error: thinkthen.NativeFailure) extends RuntimeException(error.getMessage, error) {
  val kind: thinkthen.ErrorKind = error.kind()
  val code: Int = error.code()
  val retryable: Boolean = error.retryable()
  val facts: Option[Results.ImmediateFacts] = Option(error.facts())
}

/** One producer and one reader over the shared owner. */
final class OwnedSession private[scala] (private val native: thinkthen.OwnedSession) extends AutoCloseable {
  def tryPush(descriptor: Inputs.RequestSessionDescriptor): Int = ScalaEngine.native(native.tryPush(descriptor))
  private def future[A,B](pending: java.util.concurrent.CompletableFuture[A])(convert: A => B): Future[B] = {
    val result = Promise[B]()
    pending.whenComplete((value, error) => { if (error == null) result.trySuccess(convert(value)) else result.tryFailure(ScalaEngine.failure(error)); () })
    result.future
  }
  def push(descriptor: Inputs.RequestSessionDescriptor): Future[Int] = future(native.push(descriptor))(_.intValue())
  def nextPacket(settlement: Boolean = false): Future[OwnedSession.Read] = future(native.nextPacketAsync(settlement))(read => OwnedSession.Read(Option(read.packet()), read.ended()))
  def finish(failure: Option[Inputs.RequestReaderFailure] = None): Unit = ScalaEngine.native(native.finish(failure.orNull))
  def cancel(): Unit = native.cancel()
  def waitReady(input: Boolean): Future[Unit] = future(native.waitReady(input))(_ => ())
  def tryRead(): OwnedSession.Read = ScalaEngine.native { val read = native.tryRead(); OwnedSession.Read(Option(read.packet()), read.ended()) }
  override def close(): Unit = native.close()
}
object OwnedSession { final case class Read(packet: Option[Results.SessionPacket], ended: Boolean) }
