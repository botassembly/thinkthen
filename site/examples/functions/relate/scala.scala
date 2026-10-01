import java.nio.charset.StandardCharsets.UTF_8
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def relate(): Unit =
  Using.resource(Door()) { tt =>
    val sings = """
      {"version": 1, "relate": {"relations": [
        {"name": "sings",
         "source": "singer", "target": "song"}]}}"""
    val names = Seq(
      "Paul McCartney" -> "singer",
      "Ringo Starr" -> "singer",
      "Yesterday" -> "song",
      "Octopus's Garden" -> "song"
    )
    val records = names.map { (name, kind) =>
      s"""{"name": ${Json.quote(name)},
          "kind": ${Json.quote(kind)}}""".getBytes(UTF_8)
    }.toArray
    val whoSings = tt.relate(sings, records).value()
    val edges = whoSings.get("edges")
      .asInstanceOf[java.util.List[?]].asScala
    val pairs = edges.map { one =>
      val edge = one.asInstanceOf[java.util.Map[?, ?]]
      val singer = edge.get("source")
        .asInstanceOf[java.util.Map[?, ?]]
      val song = edge.get("target")
        .asInstanceOf[java.util.Map[?, ?]]
      (singer.get("name"), song.get("name"))
    }
    assert(pairs == Seq(
      ("Paul McCartney", "Yesterday"),
      ("Ringo Starr", "Octopus's Garden")
    ))
  }
