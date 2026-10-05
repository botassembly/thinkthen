import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.Door
import thinkthen.Door.Outcome

@main def backends(): Unit =
  val settings = Seq(
    "{\"backend\":\"typesafe\"}",
    "{\"backend\":\"liquid\"}",
    "{\"backend\":\"ollama\",\"base_url\":" +
      "\"http://localhost:11535/v1\"}",
  )
  for setting <- settings do
    Using.resource(Door(setting)) { engine =>
      val tt = ScalaFacade(engine)
      val question =
        "Does the customer ask for a refund?"
      val brokenIsRefund = tt.decide(
        question,
        "Please refund my order. It arrived broken."
      )
      val thanksIsRefund = tt.decide(
        question,
        "Thanks for the quick help yesterday!"
      )
      val broken = Door.outcome(brokenIsRefund.value())
      val thanks = Door.outcome(thanksIsRefund.value())
      assert(broken == Outcome.YES)
      assert(thanks == Outcome.NO)
    }
