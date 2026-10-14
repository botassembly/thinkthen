import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.Results.*
import thinkthen.scala.ScalaEngine

@main def annotate(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val form = RequestQuestionFile().path("form.json")
    val report = RequestInputText()
      .text("Steps: click Log in. Nobody gets in.")
    val call = tt.annotate(form, report)
    val triage = Await.result(call.result, 10.seconds)
      .annotate().head.value().value()
    assert(triage == Map(
      "steps" -> AnnotatedFieldBoolean(true),
      "area" -> AnnotatedFieldString("login"),
      "impact" -> AnnotatedFieldNumber(
        java.math.BigDecimal("1.98")),
    ).asJava)
  }
