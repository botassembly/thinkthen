import thinkthen.scala.ScalaEngine
import thinkthen.Presence
import thinkthen.scala.{OwnedCall, SessionFailure}
import thinkthen.Results
import thinkthen.Inputs
import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.jdk.CollectionConverters.*
import java.nio.file.{Files, Path}

object SessionScala {
  def main(args: Array[String]): Unit = {
    val authoredNames = List("zebra", "mango", "apple", "tango", "berry").foldLeft(new Inputs.Ordered[Inputs.AuthoredDescription]())((values, name) => values.entry(name, new Inputs.AuthoredDescription(name)))
    thinkthen.AuthoredOrderCases.check(new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionAuthoredChoose().choose(new Inputs.AuthoredQuestionText("Which?")).options(new Inputs.AuthoredOptions(authoredNames))), "options")
    thinkthen.AuthoredOrderCases.check(new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionRecognizeDefinition().recognize(new Inputs.RecognizeQuestion().kinds(authoredNames))), "kinds")
    val engine = new ScalaEngine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0))
    var retainedCall: OwnedCall = null
    var failure: SessionFailure = null
    try {
      val question = new Inputs.RequestQuestionText().text("Is it?")
      def input(text: String) = new Inputs.RequestInputText().text(text)
      try { Await.result(engine.decide(question, input("scala-invalid"), new Inputs.RequestOptions().contextNull()).result, 10.seconds); throw new AssertionError("null context admitted") }
      catch { case _: thinkthen.scala.NativeFailure => () }
      val plan = engine.plan(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(input("scala-plan"))))
      assert(plan.requests().signum() > 0)
      val held = engine.decide(question, input("hold-jvm-scala"))
      val limit = System.nanoTime() + 10.seconds.toNanos
      while (!Files.exists(Path.of("barrier/arrived-hold-jvm-scala"))) {
        assert(System.nanoTime() < limit, "provider did not arrive")
        Thread.sleep(5)
      }
      val feedRequest = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(new Inputs.RequestInputFeed().name("input")))
      val descriptor = new Inputs.RequestSessionDescriptor().item(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-scala-independent")))
      val independent = Await.result(engine.execute(request = feedRequest, feed = List(descriptor)).result, 10.seconds)
      assert(independent.terminal().facts().state() == Presence.State.VALUE)
      val started = System.nanoTime()
      assert(held.cancel())
      try { Await.result(held.result, 2.seconds); throw new AssertionError("Future succeeded after cancellation") }
      catch { case _: java.util.concurrent.CancellationException => () }
      assert(System.nanoTime() - started < 2.seconds.toNanos)
      assert(!Files.exists(Path.of("barrier/release-hold-jvm-scala")))
      assert(independent.terminal().facts().value().requestsSent().intValueExact() == 1)
      val authored = new Inputs.RequestQuestionDefinition().value(new Inputs.RequestDefinitionAuthoredDecide().decide(new Inputs.AuthoredQuestionText("Is it?")).trueValue(new Inputs.AuthoredCriterion(Map[String, Any]("ok" -> false, "number" -> new java.math.BigDecimal("7.5"), "values" -> List("nested", null).asJava).asJava)).falseValueNull())
      val original = Map("body" -> "session-scala-nested", "metadata" -> List(Map("present" -> null).asJava).asJava)
      val records = new Inputs.RequestInputRecords().items(List(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(original.asJava))).asJava)
      val nested = Await.result(engine.decide(authored, records, new Inputs.RequestOptions().field(List("/body").asJava)).result, 10.seconds)
      val row = nested.packets().collectFirst { case value: Results.SessionPacketDecideRow => value }.get
      val reading = row.value().value().value().asInstanceOf[java.util.Map[String, Any]]
      assert(reading.get("ok") == false && reading.get("number") == new java.math.BigDecimal("7.5"))
      assert(reading.get("values").asInstanceOf[java.util.List[Any]].get(1) == null)
      val retained = row.value().input().value().asInstanceOf[java.util.Map[String, Any]]
      assert(retained.get("metadata").asInstanceOf[java.util.List[Any]].get(0).asInstanceOf[java.util.Map[String, Any]].containsKey("present"))
      assert(nested.terminal().facts().value().requestsSent().intValueExact() == 1)
      assert(nested.decide().contains(true), "normalized answer remains separate from authored JSON")
      try { Await.result(engine.decide(authored, records, new Inputs.RequestOptions().extension("field", List(7).asJava)).result, 10.seconds); throw new AssertionError("invalid field admitted") }
      catch { case _: thinkthen.scala.NativeFailure => () }
      retainedCall = independent
      try { Await.result(engine.decide(question, input("status-401")).result, 10.seconds); throw new AssertionError("provider failure succeeded") }
      catch { case error: SessionFailure => failure = error }
      ScalaExamples.run(engine)
    } finally engine.close()
    assert(retainedCall.terminal().facts().value().requestsSent().intValueExact() == 1)
    assert(failure.call().terminal().facts().value().requestsSent().intValueExact() == 1)
    assert(failure.failure().error().message().nonEmpty)
    println("INSTALLED_JVM_SCALA_SESSION_PASS")
  }
}
