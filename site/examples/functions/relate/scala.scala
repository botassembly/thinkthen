import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def relate(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val makes = Map("name" -> "makes",
      "source" -> "company", "target" -> "product",
      "either" -> false)
    val ask = RequestQuestionDefinition().value(
      RequestDefinitionAuthoredRelate
        .fromJson(Map("version" -> 1, "relate" ->
          Map("relations" -> List(makes))).asJava))
    val apple = Map(
      "name" -> "Apple", "kind" -> "company")
    val amazon = Map(
      "name" -> "Amazon", "kind" -> "company")
    val iphone = Map(
      "name" -> "iPhone", "kind" -> "product")
    val kindle = Map(
      "name" -> "Kindle", "kind" -> "product")
    val names = RequestInputRecords().items(
      List(apple, amazon, iphone, kindle).map { one =>
        RequestItem().original(
          RequestOriginalJson().value(one))
      }.asJava)
    val call = tt.relate(ask, names)
    val whoMakes = Await.result(call.result, 10.seconds)
      .relate().value().asScala.toList
    val pairs = whoMakes.map { edge =>
      edge.sourceName() -> edge.targetName()
    }
    assert(pairs == List(
      "Apple" -> "iPhone",
      "Amazon" -> "Kindle",
    ))
  }
