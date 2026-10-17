import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.Results.*
import thinkthen.scala.ScalaEngine

@main def recognize(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val kinds = List(
      "person" -> null,
      "product" -> null,
      "place" -> null,
    ).foldLeft(new Ordered[AuthoredDescription]()) {
      case (values, (name, description)) =>
        values.entry(name, description)
    }
    val ask = RequestQuestionDefinition().value(
      RequestDefinitionRecognizeDefinition()
        .recognize(RecognizeQuestion().kinds(kinds)))
    val text = RequestInputText().text(
      "Sam Patel says his Kindle arrived broken " +
        "in Denver.")
    val call = tt.recognize(ask, text)
    val facts = Await.result(call.result, 10.seconds)
      .value
      .asInstanceOf[RecognizeEntities]
    val names = facts.entities().asScala.toList
      .map(one => one.text() -> one.kind())
    assert(names == List(
      "Sam Patel" -> "person",
      "Kindle" -> "product",
      "Denver" -> "place",
    ))
  }
