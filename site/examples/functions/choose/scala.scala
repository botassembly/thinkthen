import scala.concurrent.Await
import scala.concurrent.duration.*
import scala.util.Using
import scala.collection.immutable.ListMap
import scala.jdk.CollectionConverters.*
import thinkthen.Inputs.*
import thinkthen.scala.ScalaEngine

@main def choose(): Unit =
  Using.resource(new ScalaEngine()) { tt =>
    val question = "Which team owns this?"
    val teams = ListMap(
      "billing" -> "Invoices, fees, and refunds.",
      "shipping" -> "Parcels and delivery.",
      "account" -> "Logins and passwords.",
    ).map((name, text) => name -> AuthoredDescription(text))
    val owner = RequestQuestionDefinition().value(
      RequestDefinitionAuthoredChoose()
        .choose(AuthoredQuestionText(question))
        .options(AuthoredOptions(teams.asJava)))
    val parcel = RequestInputText().text(
      "My parcel went to the wrong address.")
    val call = tt.choose(owner, parcel)
    val team = Await.result(call.result, 10.seconds)
      .choose()
    assert(team == "shipping")
  }
