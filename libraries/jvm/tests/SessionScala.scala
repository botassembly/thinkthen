import thinkthen.scala.ScalaEngine
import thinkthen.Presence
import thinkthen.scala.{Results, OwnedCall, SessionFailure}
import thinkthen.Inputs
import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.jdk.CollectionConverters.*
import java.nio.file.{Files, Path}

object SessionScala {
  def main(args: Array[String]): Unit = {
    val missing = Results.AnnotationValueDecision.read(Map("kind" -> "decision").asJava)
    assert(missing.presence("value") == Presence.State.MISSING)
    try { missing.value; throw new AssertionError("required nullable member was missing") } catch { case _: IllegalArgumentException => () }
    val nil = Results.AnnotationValueDecision.read(Map("kind" -> "decision", "value" -> null, "future" -> false).asJava)
    assert(nil.value.isEmpty && nil.presence("value") == Presence.State.NULL)
    assert(thinkthen.Values.`object`(nil.json).get("future") == false)
    assert(Results.AnnotationValueDecision.read(Map("kind" -> "decision", "value" -> false).asJava).value.contains(false))
    val optional = Results.SessionPacketTerminal.read(Map("kind" -> "terminal", "facts" -> null).asJava)
    assert(optional.facts.isEmpty && optional.presence("facts") == Presence.State.NULL)
    assert(Results.AtomicBoolean.read(Map.empty[String, Any].asJava).input.isEmpty)
    assert(Results.AtomicBoolean.read(Map("input" -> null).asJava).input.contains(None))
    assert(Results.AtomicBoolean.read(Map("input" -> false).asJava).input.contains(Some(false)))
    val engine = new ScalaEngine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0))
    var retainedCall: OwnedCall = null
    var failure: SessionFailure = null
    try {
      val question = new Inputs.RequestQuestionText().text("Is it?")
      def input(text: String) = new Inputs.RequestInputText().text(text)
      try { Await.result(engine.decide(question, input("scala-invalid"), new Inputs.RequestOptions().contextNull()).result, 10.seconds); throw new AssertionError("null context admitted") }
      catch { case _: thinkthen.scala.NativeFailure => () }
      val plan = engine.plan(new Inputs.Request().call(new Inputs.RequestCallDecide().question(question).input(input("scala-plan"))))
      assert(plan.requests > 0)
      val held = engine.decide(question, input("hold-jvm-scala"))
      val limit = System.nanoTime() + 10.seconds.toNanos
      while (!Files.exists(Path.of("barrier/arrived-hold-jvm-scala"))) {
        assert(System.nanoTime() < limit, "provider did not arrive")
        Thread.sleep(5)
      }
      val independent = Await.result(engine.decide(question, input("session-scala-independent")).result, 10.seconds)
      assert(independent.terminal.presence("facts") == Presence.State.VALUE)
      val started = System.nanoTime()
      assert(held.cancel())
      try { Await.result(held.result, 2.seconds); throw new AssertionError("Future succeeded after cancellation") }
      catch { case _: java.util.concurrent.CancellationException => () }
      assert(System.nanoTime() - started < 2.seconds.toNanos)
      assert(!Files.exists(Path.of("barrier/release-hold-jvm-scala")))
      assert(independent.terminal.facts.get.requestsSent.toInt == 1)
      val authored = new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionFieldsDecide().decide(new Inputs.AuthoredQuestionText("Is it?")).trueValue(new Inputs.AuthoredCriterion(Map[String, Any]("ok" -> false, "number" -> BigDecimal("7.5"), "values" -> List("nested", null)).asJava)).falseValueNull())
      val original = Map("body" -> "session-scala-nested", "metadata" -> List(Map("present" -> null)))
      val records = new Inputs.RequestInputRecords().items(List(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(original))).asJava)
      val nested = Await.result(engine.decide(authored, records, new Inputs.RequestOptions().field(List("/body").asJava).details(true)).result, 10.seconds)
      val row = nested.packets.collectFirst { case value: Results.SessionPacketDecideRow => value }.get
      val reading = row.value.value.json.asInstanceOf[java.util.Map[String, Any]]
      assert(reading.get("ok") == false && reading.get("number") == new java.math.BigDecimal("7.5"))
      assert(reading.get("values").asInstanceOf[java.util.List[Any]].get(1) == null)
      val retained = row.value.input.get.get.asInstanceOf[java.util.Map[String, Any]]
      assert(retained.get("metadata").asInstanceOf[java.util.List[Any]].get(0).asInstanceOf[java.util.Map[String, Any]].containsKey("present"))
      assert(nested.terminal.facts.get.requestsSent.toInt == 1)
      try { Await.result(engine.decide(authored, records, new Inputs.RequestOptions().extension("field", List(7))).result, 10.seconds); throw new AssertionError("invalid field admitted") }
      catch { case _: thinkthen.scala.NativeFailure => () }
      retainedCall = independent
      try { Await.result(engine.decide(question, input("status-401")).result, 10.seconds); throw new AssertionError("provider failure succeeded") }
      catch { case error: SessionFailure => failure = error }
    } finally engine.close()
    assert(retainedCall.terminal.facts.get.requestsSent.toInt == 1)
    assert(failure.call.terminal.facts.get.requestsSent.toInt == 1)
    assert(failure.failure.error.message.nonEmpty)
    println("INSTALLED_JVM_SCALA_SESSION_PASS")
  }
}
