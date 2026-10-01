import java.math.BigDecimal
import java.nio.file.{Files, Path}
import scala.concurrent.ExecutionContext.Implicits.global
import scala.jdk.CollectionConverters.*
import scala.util.Using
import thinkthen.{Door, Json}

@main def annotate(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val form = Files.readString(Path.of("form.json"))
    val body = "Steps: click Log in. Nobody gets in."
    val triage = Json.parseObject(tt.call(
      s"""{"annotate": $form,
          "records": [${Json.quote(body)}]}"""
    )).get("value").asInstanceOf[java.util.List[?]]
    val rows = triage.asScala.map(
      _.asInstanceOf[java.util.Map[?, ?]].asScala
    )
    assert(rows == Seq(Map(
      "steps" -> true,
      "area" -> "login",
      "impact" -> BigDecimal("1.98")
    )))
  }
