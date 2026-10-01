import java.nio.charset.StandardCharsets.UTF_8
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def recognize(): Unit =
  Using.resource(Door()) { tt =>
    val kinds = Seq(
      "PER" -> "Part of a person's name.",
      "ORG" -> ("Part of the name of an organization: " +
        "a company, band, team, agency, " +
        "government body, or media outlet."),
      "LOC" -> ("Part of the name of a place: " +
        "a country, region, city, " +
        "or geographic feature."),
      "MISC" -> ("Part of another named entity: " +
        "a nationality, an event, a product, " +
        "or the name of a creative work.")
    )
    val spec = kinds
      .map((kind, means) =>
        s"${Json.quote(kind)}: ${Json.quote(means)}")
      .mkString(
        """{"version": 1, "recognize": {"kinds": {""",
        ", ",
        "}}}"
      )
    val text = "Maria Chen joined Northwind Freight " +
      "in Chicago last spring."
    val facts = tt.recognize(spec, text.getBytes(UTF_8))
      .value()
    val entities = facts.get("entities")
      .asInstanceOf[java.util.List[?]].asScala
    val names = entities.map { one =>
      val name = one.asInstanceOf[java.util.Map[?, ?]]
      (name.get("text"), name.get("kind"))
    }
    assert(names == Seq(
      ("Maria Chen", "PER"),
      ("Northwind Freight", "ORG"),
      ("Chicago", "LOC")
    ))
  }
