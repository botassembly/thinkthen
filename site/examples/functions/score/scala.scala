import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.Results.SessionPacketScoreRow
import thinkthen.scala.ScalaEngine

@main def score(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = "How urgent is this?"
    val levels = List("Routine.", "Soon.", "Immediate.")
    val scale = RequestQuestionDefinition().value(
      RequestDefinitionFieldsScore()
        .score(AuthoredQuestionText(question))
        .levels(AuthoredLevels(levels.asJava)))
    val outage = RequestInputText().text(
      "Our checkout page is down " +
        "and customers cannot pay.\n")
    val call = tt.score(scale, outage)
    val urgency = Await.result(call.result, 10.seconds)
      .packets.collectFirst {
        case row: SessionPacketScoreRow => row.value.value
      }
    assert(urgency.contains(BigDecimal(2)))
  }
