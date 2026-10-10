package thinkthen.scala
import thinkthen.Engine
import thinkthen.Inputs
import scala.concurrent.{Future, Promise}
import scala.jdk.CollectionConverters.*
/** A Scala Future and explicit cancellation over the shared JVM session. */
final case class Call(result: Future[Engine.OwnedCall], cancel: () => Boolean)
final class ScalaEngine private (private val engine: Engine) extends AutoCloseable {
  def this(settings: Map[String, Any]) = this(new Engine(ScalaEngine.javaMap(settings), Engine.Surface.SCALA))
  def this() = this(Map.empty[String, Any])
  def this(settings: Inputs.EngineSettings) = this(new Engine(settings, Engine.Surface.SCALA))
  private def javaMap[K](value: scala.collection.Map[K, Any]): java.util.Map[K, Any] = ScalaEngine.javaMap(value)
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
  def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = decide(question, input, null)
  def decide(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.decide(question, input, options))
  def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = choose(question, input, null)
  def choose(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.choose(question, input, options))
  def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = tag(question, input, null)
  def tag(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.tag(question, input, options))
  def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = score(question, input, null)
  def score(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.score(question, input, options))
  def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = filter(question, input, null)
  def filter(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.filter(question, input, options))
  def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = rank(question, input, null)
  def rank(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.rank(question, input, options))
  def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = find(question, input, null)
  def find(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.find(question, input, options))
  def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = annotate(question, input, null)
  def annotate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.annotate(question, input, options))
  def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = recognize(question, input, null)
  def recognize(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.recognize(question, input, options))
  def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput): Call = relate(question, input, null)
  def relate(question: Inputs.RequestQuestion, input: Inputs.RequestInput, options: Inputs.RequestOptions): Call = run(engine.relate(question, input, options))
  override def close(): Unit = engine.close()
}

object ScalaEngine {
  private def javaMap[K](value: scala.collection.Map[K, Any]): java.util.Map[K, Any] =
    value.iterator.map { case (key, item) => key -> javaValue(item) }.toMap.asJava
  private def javaValue(value: Any): Any = value match {
    case map: scala.collection.Map[?, ?] => javaMap(map)
    case items: Iterable[?] => items.iterator.map(javaValue).toList.asJava
    case scalar => scalar
  }
}
