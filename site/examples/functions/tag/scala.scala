import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def tag(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = "Which labels fit this message?"
    val labels = List("praise", "bug", "billing")
    val fits = RequestQuestionDefinition().value(
      RequestDefinitionAuthoredTag()
        .tag(AuthoredQuestionText(question))
        .labels(AuthoredLabels(labels.asJava)))
    val message = RequestInputText().text(
      "Love the new dashboard, " +
        "but export crashes the app,\n" +
        "and I was charged twice.\n")
    val call = tt.tag(fits, message)
    val fittingLabels =
      Await.result(call.result, 10.seconds)
      .value
    assert(fittingLabels == labels)
  }
