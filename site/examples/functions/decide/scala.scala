import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import thinkthen.Inputs.*
import thinkthen.scala.Results.SessionPacketDecideRow
import thinkthen.scala.ScalaEngine

@main def decide(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = RequestQuestionText()
      .text("Does the customer ask for a refund?")
    val broken = RequestInputText().text(
      "Please refund my order. It arrived broken.")
    val call = tt.decide(question, broken)
    val isRefund = Await.result(call.result, 10.seconds)
      .packets.collectFirst {
        case row: SessionPacketDecideRow =>
          row.value.value.value
      }
    assert(isRefund.contains(true))
  }
