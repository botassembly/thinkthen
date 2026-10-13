import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.collection.immutable.ListMap
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.Results.*
import thinkthen.scala.ScalaEngine

@main def recognize(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val kinds = ListMap(
      "person" -> null,
      "product" -> null,
      "place" -> null,
    )
    val ask = RequestQuestionDefinition().value(
      RequestDefinitionRecognizeDefinition
        .fromJson(Map("version" -> 1,
          "recognize" -> Map("kinds" -> kinds)).asJava))
    val text = RequestInputText().text(
      "Sam Patel says his Kindle arrived broken " +
        "in Denver.")
    val call = tt.recognize(ask, text)
    val facts = Await.result(call.result, 10.seconds)
      .recognize().getFirst().value()
      .asInstanceOf[RecognizeEntities]
    val names = facts.entities().asScala.toList
      .map(one => one.text() -> one.kind())
    assert(names == List(
      "Sam Patel" -> "person",
      "Kindle" -> "product",
      "Denver" -> "place",
    ))
  }
