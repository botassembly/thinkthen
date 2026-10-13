package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import thinkthen.Results
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[OwnedCall], cancel: () => Boolean)
final class ScalaEngine private (private val engine: Engine) extends ScalaNamedCalls(engine) with AutoCloseable {
  def this(settings: Inputs.EngineSettings) = this(ScalaEngine.native(new Engine(settings, Engine.Surface.SCALA)))
  def this() = this(new Inputs.EngineSettings())
  protected override def run(native: java.util.concurrent.CompletableFuture[Engine.OwnedCall]): Call = {
    val result = Promise[OwnedCall]()
    native.whenComplete((value, error) => { try { if (error == null) result.trySuccess(value) else result.tryFailure(ScalaEngine.failure(error)) } catch { case error: Throwable => result.tryFailure(ScalaEngine.failure(error)) }; () })
    Call(result.future, () => native.cancel(false))
  }
  def plan(request: Inputs.Request): Results.Plan = try engine.plan(request) catch { case error: Throwable => throw ScalaEngine.failure(error) }
  def startSession(request: Inputs.Request): OwnedSession = ScalaEngine.native(new OwnedSession(engine.startSession(request)))
  def execute(request: Inputs.Request): Call = run(engine.execute(request))
  def execute(request: Inputs.Request, feed: java.lang.Iterable[Inputs.RequestSessionDescriptor]): Call = run(engine.execute(request, feed))
  def usagePersistence(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.usagePersistence()))
  def finishUsageStatus(): UsagePersistence = ScalaEngine.native(UsagePersistence.read(engine.finishUsageStatus()))
  override def close(): Unit = engine.close()
}

type OwnedCall = Engine.OwnedCall
type SessionFailure = Engine.SessionFailure
object ScalaEngine {
  private[scala] def native[T](action: => T): T = try action catch { case error: Throwable => throw failure(error) }
  private def failure(error: Throwable): Throwable = error match {
    case wrapped: java.util.concurrent.CompletionException if wrapped.getCause != null => failure(wrapped.getCause)
    case native: Engine.SessionFailure => native
    case native: thinkthen.NativeFailure => new NativeFailure(native)
    case other => other
  }

}

final class NativeFailure private[scala](error: thinkthen.NativeFailure) extends RuntimeException(error.getMessage, error) {
  val code: Int = error.code()
  val retryable: Boolean = error.retryable()
  val facts: Option[Any] = Option(error.facts())
}

/** One producer and one reader over the shared owner. */
final class OwnedSession private[scala] (private val native: thinkthen.OwnedSession) extends AutoCloseable {
  def tryPush(descriptor: Inputs.RequestSessionDescriptor): Int = ScalaEngine.native(native.tryPush(descriptor))
  def push(descriptor: Inputs.RequestSessionDescriptor): java.util.concurrent.CompletableFuture[Integer] = native.push(descriptor)
  def nextPacket(settlement: Boolean = false): Future[OwnedSession.Read] = {
    val result = Promise[OwnedSession.Read]()
    native.nextPacketAsync(settlement).whenComplete((read, error) => {
      if (error == null) result.trySuccess(OwnedSession.Read(Option(read.packet()), read.ended())) else result.tryFailure(error)
      ()
    })
    result.future
  }
  def finish(failure: Inputs.RequestReaderFailure = null): Unit = ScalaEngine.native(native.finish(if (failure == null) null else failure))
  def cancel(): Unit = native.cancel()
  def waitReady(input: Boolean): java.util.concurrent.CompletableFuture[Void] = native.waitReady(input)
  def tryRead(): OwnedSession.Read = ScalaEngine.native { val read = native.tryRead(); OwnedSession.Read(Option(read.packet()), read.ended()) }
  override def close(): Unit = native.close()
}
object OwnedSession { final case class Read(packet: Option[Results.SessionPacket], ended: Boolean) }
