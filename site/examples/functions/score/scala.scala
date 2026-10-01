import java.math.BigDecimal
import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.{Door, Json}

@main def score(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "How urgent is this?"
    val levels = """["Routine.", "Soon.", "Immediate."]"""
    val text = "Our checkout page is down " +
      "and customers cannot pay.\n"
    val urgency = Json.parseObject(tt.call(
      s"""{"score": ${Json.quote(question)},
          "levels": $levels,
          "evidence": ${Json.quote(text)}}"""
    )).get("value")
    assert(urgency == BigDecimal("2.0"))
  }
