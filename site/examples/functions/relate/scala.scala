import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.Results.SessionPacketRelateAggregate
import thinkthen.scala.ScalaEngine

@main def relate(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val sings = Map("name" -> "sings",
      "source" -> "singer", "target" -> "song",
      "either" -> false)
    val ask = RequestQuestionDefinition().value(
      RequestDefinitionFieldsRelateVersion
        .fromJson(Map("version" -> 1, "relate" ->
          Map("relations" -> List(sings))).asJava))
    val paul = Map(
      "name" -> "Paul McCartney", "kind" -> "singer")
    val ringo = Map(
      "name" -> "Ringo Starr", "kind" -> "singer")
    val yesterday = Map(
      "name" -> "Yesterday", "kind" -> "song")
    val garden = Map(
      "name" -> "Octopus's Garden", "kind" -> "song")
    val names = RequestInputRecords().items(
      List(paul, ringo, yesterday, garden).map { one =>
        RequestItem().original(
          RequestOriginalJson().value(one))
      }.asJava)
    val call = tt.relate(ask, names)
    val whoSings = Await.result(call.result, 10.seconds)
      .packets.collectFirst {
        case related: SessionPacketRelateAggregate =>
          related.value.value
      }.get
    val pairs = whoSings.map { edge =>
      edge.source.json -> edge.target.json
    }
    assert(pairs == List(
      paul.asJava -> yesterday.asJava,
      ringo.asJava -> garden.asJava,
    ))
  }
