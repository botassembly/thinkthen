import examples.SampleInputs as S
import thinkthen.scala.ScalaEngine
import scala.concurrent.Await
import scala.concurrent.duration.*
object ScalaExamples {
  def run(engine: ScalaEngine): List[Any] = {
    val answers = List(
    Await.result(engine.decide(S.decide(), S.input("session-decide"), S.options()).result, 10.seconds).decide(),
    Await.result(engine.choose(S.choose(), S.input("session-choose"), S.options()).result, 10.seconds).choose(),
    Await.result(engine.tag(S.tag(), S.input("session-tag"), S.options()).result, 10.seconds).tag(),
    Await.result(engine.score(S.score(), S.input("session-score"), S.options()).result, 10.seconds).score(),
    Await.result(engine.filter(S.decide(), S.input("session-filter"), S.options()).result, 10.seconds).filter(),
    Await.result(engine.rank(S.decide(), S.input("session-rank"), S.options()).result, 10.seconds).rank(),
    Await.result(engine.find(S.find(), S.units(), S.options()).result, 10.seconds).find(),
    Await.result(engine.annotate(S.annotate(), S.input("session-annotate"), S.options()).result, 10.seconds).annotate(),
    Await.result(engine.recognize(S.recognize(), S.input("Maria Chen"), S.options()).result, 10.seconds).recognize(),
    Await.result(engine.relate(S.relate(), S.entities(), S.options()).result, 10.seconds).relate()
    )
    val edge = answers.last.asInstanceOf[thinkthen.Results.Relation].value().getFirst()
    println(edge.sourceName() + " -> " + edge.targetName())
    answers
  }
}
