import examples.SampleInputs as S
import thinkthen.scala.ScalaEngine
import scala.concurrent.Await
import scala.concurrent.duration.*
object ScalaExamples {
  def run(engine: ScalaEngine): List[Any] = {
    val answers = List(
    Await.result(engine.decide(question = S.decide(), input = S.input("session-decide"), options = Some(S.options())).result, 10.seconds).value,
    Await.result(engine.choose(S.choose(), S.input("session-choose"), S.options()).result, 10.seconds).value,
    Await.result(engine.tag(S.tag(), S.input("session-tag"), S.options()).result, 10.seconds).value,
    Await.result(engine.score(S.score(), S.input("session-score"), S.options()).result, 10.seconds).value,
    Await.result(engine.filter(S.decide(), S.input("session-filter"), S.options()).result, 10.seconds).value,
    Await.result(engine.rank(S.decide(), S.input("session-rank"), S.options()).result, 10.seconds).value,
    Await.result(engine.find(S.find(), S.units(), S.options()).result, 10.seconds).value,
    Await.result(engine.annotate(S.annotate(), S.input("session-annotate"), S.options()).result, 10.seconds).value,
    Await.result(engine.recognize(S.recognize(), S.input("Maria Chen"), S.options()).result, 10.seconds).value,
    Await.result(engine.relate(S.relate(), S.entities(), S.options()).result, 10.seconds).value
    )
    val edge = answers.last.asInstanceOf[List[thinkthen.Results.RelatedEntityEdge]].head
    println(edge.sourceName() + " -> " + edge.targetName())
    answers
  }
}
