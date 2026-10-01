import scala.concurrent.ExecutionContext.Implicits.global
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def tag(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "Which labels fit this message?"
    val labels = """["praise", "bug", "billing"]"""
    val message =
      "Love the new dashboard, but export crashes " +
        "the app,\nand I was charged twice.\n"
    val fittingLabels = Json.parseObject(tt.call(
      s"""{"tag": ${Json.quote(question)},
          "labels": $labels,
          "evidence": ${Json.quote(message)}}"""
    )).get("value").asInstanceOf[java.util.List[?]]
    assert(
      fittingLabels.asScala ==
        Seq("praise", "bug", "billing")
    )
  }
