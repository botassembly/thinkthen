import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def choose(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = "Which team owns this?"
    val teams = List(
      "billing" -> "Invoices, fees, and refunds.",
      "shipping" -> "Parcels and delivery.",
      "account" -> "Logins and passwords.",
    ).foldLeft(new Ordered[AuthoredDescription]()) {
      case (values, (name, text)) =>
        values.entry(name, AuthoredDescription(text))
    }
    val owner = RequestQuestionDefinition().value(
      RequestDefinitionAuthoredChoose()
        .choose(AuthoredQuestionText(question))
        .options(AuthoredOptions(teams)))
    val parcel = RequestInputText().text(
      "My parcel went to the wrong address.")
    val call = tt.choose(owner, parcel)
    val team = Await.result(call.result, 10.seconds)
      .value
    assert(team.contains("shipping"))
  }
