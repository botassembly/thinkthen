import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def rank(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = RequestQuestionText()
      .text("Is this urgent?")
    val inbox = List(
      "Newsletter: our autumn catalog is here. " +
        "No reply needed.",
      "Our checkout page is down " +
        "and customers cannot pay",
      "Reminder: your invoice is due in 30 days",
      "Please send the signed quote by 5 pm today",
    )
    val records = RequestInputRecords().items(
      inbox.map { text =>
        RequestItem().original(
          RequestOriginalText().text(text))
      }.asJava)
    val call = tt.rank(question, records)
    val order = Await.result(call.result, 10.seconds)
      .value.map(_.intValue())
    assert(order == List(1, 3, 2, 0))
  }
