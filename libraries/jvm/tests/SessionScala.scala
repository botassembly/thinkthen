import thinkthen.scala.ScalaEngine
import thinkthen.Presence
import thinkthen.Results
import thinkthen.Engine
import thinkthen.Inputs
import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.jdk.CollectionConverters.*
import java.nio.file.{Files, Path}

object SessionScala {
  def main(args: Array[String]): Unit = {
    val engine = new ScalaEngine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0))
    var retainedCall: Engine.OwnedCall = null
    var failure: Engine.SessionFailure = null
    try {
      val question = new Inputs.RequestQuestionText().text("Is it?")
      def input(text: String) = new Inputs.RequestInputText().text(text)
      try { Await.result(engine.decide(question, input("scala-invalid"), new Inputs.RequestOptions().contextNull()).result, 10.seconds); throw new AssertionError("null context admitted") }
      catch { case _: thinkthen.NativeFailure => () }
      val held = engine.decide(question, input("hold-jvm-scala"))
      val limit = System.nanoTime() + 10.seconds.toNanos
      while (!Files.exists(Path.of("barrier/arrived-hold-jvm-scala"))) {
        assert(System.nanoTime() < limit, "provider did not arrive")
        Thread.sleep(5)
      }
      val independent = Await.result(engine.decide(question, input("session-scala-independent")).result, 10.seconds)
      assert(independent.terminal().facts().state() == Presence.State.VALUE)
      val started = System.nanoTime()
      assert(held.cancel())
      try { Await.result(held.result, 2.seconds); throw new AssertionError("Future succeeded after cancellation") }
      catch { case _: java.util.concurrent.CancellationException => () }
      assert(System.nanoTime() - started < 2.seconds.toNanos)
      assert(!Files.exists(Path.of("barrier/release-hold-jvm-scala")))
      assert(independent.terminal().facts().value().requestsSent().intValueExact() == 1)
      val authored = new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionFieldsDecide().decide(new Inputs.AuthoredQuestionText("Is it?")).trueValue(new Inputs.AuthoredCriterion(Map[String, Any]("ok" -> false, "number" -> BigDecimal("7.5"), "values" -> List("nested", null)).asJava)).falseValueNull())
      val original = Map("body" -> "session-scala-nested", "metadata" -> List(Map("present" -> null)))
      val records = new Inputs.RequestInputRecords().items(List(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(original))).asJava)
      val nested = Await.result(engine.decide(authored, records, new Inputs.RequestOptions().field(List("/body").asJava).details(true)).result, 10.seconds)
      val row = nested.packets().asScala.collectFirst { case value: Results.SessionPacketDecideRow => value }.get
      val reading = row.value().value().value().asInstanceOf[java.util.Map[String, Any]]
      assert(reading.get("ok") == false && reading.get("number") == new java.math.BigDecimal("7.5"))
      assert(reading.get("values").asInstanceOf[java.util.List[Any]].get(1) == null)
      val retained = row.value().input().value().asInstanceOf[java.util.Map[String, Any]]
      assert(retained.get("metadata").asInstanceOf[java.util.List[Any]].get(0).asInstanceOf[java.util.Map[String, Any]].containsKey("present"))
      assert(nested.terminal().facts().value().requestsSent().intValueExact() == 1)
      try { Await.result(engine.decide(authored, records, new Inputs.RequestOptions().extension("field", List(7))).result, 10.seconds); throw new AssertionError("invalid field admitted") }
      catch { case _: thinkthen.NativeFailure => () }
      retainedCall = independent
      try { Await.result(engine.decide(question, input("status-401")).result, 10.seconds); throw new AssertionError("provider failure succeeded") }
      catch { case error: Engine.SessionFailure => failure = error }
    } finally engine.close()
    assert(retainedCall.terminal().facts().value().requestsSent().intValueExact() == 1)
    assert(failure.call().terminal().facts().value().requestsSent().intValueExact() == 1)
    assert(failure.failure().error().message().nonEmpty)
    println("INSTALLED_JVM_SCALA_SESSION_PASS")
  }
}
