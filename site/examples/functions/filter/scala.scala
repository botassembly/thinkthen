import scala.concurrent.ExecutionContext.Implicits.global
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def filter(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "Is this a complaint?"
    val reviews = Seq(
      "Arrived a day early. Thank you!",
      "The zipper broke the first time I used it.",
      "Does this come in blue?",
      "The strap snapped on day two."
    )
    val records = reviews.map(Json.quote).mkString(", ")
    val complaints = Json.parseObject(tt.call(
      s"""{"filter": ${Json.quote(question)},
          "records": [$records]}"""
    )).get("value").asInstanceOf[java.util.List[?]]
    assert(
      complaints.asScala == Seq(reviews(1), reviews(3))
    )
  }
