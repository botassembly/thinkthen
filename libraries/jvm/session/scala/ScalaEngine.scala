package thinkthen.scala
import thinkthen.Engine
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[Engine.OwnedCall], cancel: () => Boolean)
final class ScalaEngine(settings: Map[String, Any] = Map.empty) extends AutoCloseable {
  private def javaMap[K](value: scala.collection.Map[K, Any]): java.util.Map[K, Any] =
    value.iterator.map { case (key, item) => key -> javaValue(item) }.toMap.asJava
  private def javaValue(value: Any): Any = value match {
    case map: scala.collection.Map[?, ?] => javaMap(map)
    case items: Iterable[?] => items.iterator.map(javaValue).toList.asJava
    case scalar => scalar
  }
  private val engine = new Engine(javaMap(settings))
  private def run(native: java.util.concurrent.CompletableFuture[Engine.OwnedCall]): Call = {
    val result = Promise[Engine.OwnedCall]()
    native.whenComplete((value, error) => { if (error == null) result.trySuccess(value) else result.tryFailure(error); () })
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
  override def close(): Unit = engine.close()
}
