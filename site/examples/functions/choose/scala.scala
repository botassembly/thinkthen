import scala.concurrent.ExecutionContext.Implicits.global
import scala.util.Using
import thinkthen.{Door, Json}

@main def choose(): Unit =
  Using.resource(Door()) { engine =>
    val tt = ScalaFacade(engine)
    val question = "Which team owns this?"
    val teams = """
      {"billing": "Invoices, fees, and refunds.",
       "shipping": "Parcels and delivery.",
       "account": "Logins and passwords."}"""
    val text =
      "Please refund the extra fee on my invoice."
    val owner = Json.parseObject(tt.call(
      s"""{"choose": ${Json.quote(question)},
          "options": $teams,
          "evidence": ${Json.quote(text)}}"""
    )).get("value")
    assert(owner == "billing")
  }
