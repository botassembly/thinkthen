import java.math.BigDecimal
import scala.concurrent.ExecutionContext.Implicits.global
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def rank(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "Is this urgent?"
    val inbox = Seq(
      "Newsletter: our autumn catalog is here. " +
        "No reply needed.",
      "Our checkout page is down and customers cannot pay",
      "Reminder: your invoice is due in 30 days",
      "Please send the signed quote by 5 pm today"
    )
    val records = inbox.map(Json.quote).mkString(", ")
    val byUrgency = Json.parseObject(tt.call(
      s"""{"rank": ${Json.quote(question)},
          "records": [$records]}"""
    )).get("value").asInstanceOf[java.util.List[?]]
    val order = byUrgency.asScala.map { one =>
      val index = one.asInstanceOf[java.util.Map[?, ?]]
        .get("index").asInstanceOf[BigDecimal]
      index.intValue
    }
    assert(order == Seq(1, 3, 2, 0))
  }
