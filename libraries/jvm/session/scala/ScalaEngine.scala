package thinkthen.scala
import thinkthen.Engine
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[Engine.OwnedCall], cancel: () => Boolean)
final class ScalaEngine(settings: Map[String, Any] = Map.empty) extends AutoCloseable {
  private val engine = new Engine(settings.asJava)
  private def run(native: java.util.concurrent.CompletableFuture[Engine.OwnedCall]): Call = {
    val result = Promise[Engine.OwnedCall]()
    native.whenComplete((value, error) => { if (error == null) result.trySuccess(value) else result.tryFailure(error); () })
    Call(result.future, () => native.cancel(false))
  }
  def decide(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.decide(question.asJava, input.asJava, options.asJava))
  def choose(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.choose(question.asJava, input.asJava, options.asJava))
  def tag(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.tag(question.asJava, input.asJava, options.asJava))
  def score(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.score(question.asJava, input.asJava, options.asJava))
  def filter(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.filter(question.asJava, input.asJava, options.asJava))
  def rank(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.rank(question.asJava, input.asJava, options.asJava))
  def find(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.find(question.asJava, input.asJava, options.asJava))
  def annotate(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.annotate(question.asJava, input.asJava, options.asJava))
  def recognize(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.recognize(question.asJava, input.asJava, options.asJava))
  def relate(question: Map[String, Any], input: Map[String, Any], options: Map[String, Any] = Map.empty): Call = run(engine.relate(question.asJava, input.asJava, options.asJava))
  override def close(): Unit = engine.close()
}
