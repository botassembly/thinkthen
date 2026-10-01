import java.nio.charset.StandardCharsets.UTF_8
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def recognize(): Unit =
  Using.resource(Door()) { tt =>
    val kinds = Seq("person", "organization", "place")
    val spec = kinds
      .map(kind => s"${Json.quote(kind)}: null")
      .mkString(
        """{"version": 1, "recognize": {"kinds": {""",
        ", ",
        "}}}"
      )
    val text = "Maria Chen joined Northwind Freight, " +
      "a company in Chicago."
    val facts = tt.recognize(spec, text.getBytes(UTF_8))
      .value()
    val entities = facts.get("entities")
      .asInstanceOf[java.util.List[?]].asScala
    val names = entities.map { one =>
      val name = one.asInstanceOf[java.util.Map[?, ?]]
      (name.get("text"), name.get("kind"))
    }
    assert(names == Seq(
      ("Maria Chen", "person"),
      ("Northwind Freight", "organization"),
      ("Chicago", "place")
    ))
  }
