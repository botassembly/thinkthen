import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.Door
import thinkthen.Door.Outcome

@main def decide(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question =
      "Does the customer ask for a refund?"
    val text =
      "Please refund my order. It arrived broken."
    val isRefund = tt.decide(question, text).value()
    assert(Door.outcome(isRefund) == Outcome.YES)
  }
