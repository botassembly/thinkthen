import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.Results.SessionPacketFilterRow
import thinkthen.scala.ScalaEngine

@main def filter(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = RequestQuestionText()
      .text("Is this a complaint?")
    val reviews = List(
      "Arrived a day early. Thank you!",
      "The zipper broke the first time I used it.",
      "Does this come in blue?",
      "The strap snapped on day two.",
    )
    val records = RequestInputRecords().items(
      reviews.map { text =>
        RequestItem().original(
          RequestOriginalText().text(text))
      }.asJava)
    val call = tt.filter(question, records)
    val complaints = Await.result(call.result, 10.seconds)
      .packets().asScala.collect {
        case row: SessionPacketFilterRow =>
          row.value().input().value()
      }.toList
    assert(complaints == List(reviews(1), reviews(3)))
  }
