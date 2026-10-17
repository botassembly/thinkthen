import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def find(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = RequestQuestionText()
      .text("Which line gives the refund deadline?")
    val policy = List(
      "Returns need the original receipt.",
      "Refunds are issued within 30 days of purchase.",
      "Shipping is free on orders over $50.",
      "Gift cards cannot be exchanged for cash.",
    )
    val records = RequestInputRecords().items(
      policy.map { text =>
        RequestItem().original(
          RequestOriginalText().text(text))
      }.asJava)
    val call = tt.find(question, records)
    val refundDeadline =
      Await.result(call.result, 10.seconds)
      .value
    assert(refundDeadline.contains(policy(1)))
  }
