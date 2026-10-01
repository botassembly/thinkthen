import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.{Door, Json}

@main def find(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question =
      "Which line gives the refund deadline?"
    val policy = Seq(
      "Returns need the original receipt.",
      "Refunds are issued within 30 days of purchase.",
      "Shipping is free on orders over $50.",
      "Gift cards cannot be exchanged for cash."
    )
    val units = policy.map(Json.quote).mkString(", ")
    val refundDeadline = Json.parseObject(tt.call(
      s"""{"find": ${Json.quote(question)},
          "units": [$units]}"""
    )).get("value").asInstanceOf[java.util.Map[?, ?]]
    assert(refundDeadline.get("unit") == policy(1))
  }
