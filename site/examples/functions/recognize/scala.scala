import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.collection.immutable.ListMap
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.Results.*
import thinkthen.scala.ScalaEngine

@main def recognize(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val kinds = ListMap(
      "person" -> null,
      "product" -> null,
      "place" -> null,
    )
    val ask = RequestQuestionDefinition().value(
      RequestDefinitionFieldsRecognizeVersion
        .fromJson(Map("version" -> 1,
          "recognize" -> Map("kinds" -> kinds)).asJava))
    val text = RequestInputText().text(
      "Sam Patel says his Kindle arrived broken " +
        "in Denver.")
    val call = tt.recognize(ask, text)
    val facts = Await.result(call.result, 10.seconds)
      .packets.collectFirst {
        case found: SessionPacketRecognizeAggregate =>
          found.value.head.value
      }
    val names = facts.toList.flatMap {
      case found: RecognizeFieldsEntities =>
        found.entities.map(one => one.text -> one.kind)
      case _ => Nil
    }
    assert(names == List(
      "Sam Patel" -> "person",
      "Kindle" -> "product",
      "Denver" -> "place",
    ))
  }
