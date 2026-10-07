import thinkthen.Door
import scala.concurrent.ExecutionContext

@main def scalaTypeCase(request: String): Unit = {
  if(request=="native") {val engine=new ScalaComplete(System.getenv("TT_NATIVE_SETTINGS"));try {val call=NativeChecks.run(engine);assert(call.rows().get(0).common().answer().value().probability().value()==.9);assert(call.facts().value().callId().value().length==64);assert(call.rows().get(0).common().position().value().firstLine().value()==1L)}finally engine.close();println("{\"native\":\"pass\"}");return}
  if (request == "carriers") { scalaCarrierChecks(); return }
  given ExecutionContext = ExecutionContext.global
  val door = new Door()
  try println(new ScalaFacade(door).call(request))
  catch { case failure: Door.NativeFailure =>
    println(s"{\"failed\":{\"kind\":\"${failure.failure.kind().name.toLowerCase}\",\"code\":${failure.failure.code()}}}")
  } finally door.close()
}


// Fixture decoding/static APIs; no native result or parity receipt.
private def scalaCarrierChecks(): Unit = {
  import thinkthen.Complete.*
  import thinkthen.Requests
  val question = Requests.QuestionInput.questionFile("β.json")
  val source = Requests.InputSource.files(new FileSource(java.util.List.of("β.png", "β.png"), SourceUnit.IMAGE_FILE, 0L))
  val controls = new CallControls(OptionalValue.absent(), OptionalValue.absent(), false, true)
  val requests = List(
    ScalaRequests.decide(question, source, controls),
    ScalaRequests.choose(question, source, controls),
    ScalaRequests.tag(question, source, controls),
    ScalaRequests.score(question, source, controls),
    ScalaRequests.filter(question, source, controls),
    ScalaRequests.rank(question, source, controls),
    ScalaRequests.find(question, source, controls),
    ScalaRequests.annotate(question, source, controls),
    ScalaRequests.recognize(question, source, controls),
    ScalaRequests.relate(question, source, controls)
  )
  assert(requests.map(_.function()) == Function.values().toList)
  assert(source.files().value().paths().size() == 2 && !question.question().present())
  val answer = thinkthen.CompleteReaders.atomic("""{"kind":"choice","pick":"β","probabilities":{"β":0.7,"a":0.3},"confidence":0}""")
  assert(answer.probabilities().get(0).name() == "β" && answer.confidence().present() && answer.confidence().value() == 0.0)
  val span = new NameSpan(1L, 3L, OptionalValue.of(java.util.List.of[Probability]()), OptionalValue.absent())
  assert(span.kinds().present() && !span.edges().present() && span.end() == 3L)
  println("{\"carriers\":\"pass\"}")
}
