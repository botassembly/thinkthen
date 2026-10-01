import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.Door
import thinkthen.Door.Outcome

@main def firstCall(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question =
      "Does the customer ask for a refund?"
    var isRefund = tt.decide(
      question,
      "Please refund my order. It arrived broken."
    )
    assert(Door.outcome(isRefund.value()) == Outcome.YES)

    val refund =
      s"""{"decide": "$question", """ +
        """"threshold": "0.2:0.8"}"""
    isRefund = tt.decide(
      refund,
      "I want to send this back."
    )
    val outcome = Door.outcome(isRefund.value())
    assert(outcome == Outcome.NOT_SURE)
  }
